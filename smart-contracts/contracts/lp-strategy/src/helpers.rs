use crate::{
    error::ContractError,
    error_recovery::PendingReturningRecovery,
    ibc_lock::Lock,
    msg::ExecuteMsg,
    state::{
        PendingBond, PendingSingleUnbond, RawAmount, BOND_QUEUE, CHANNELS, CONFIG,
        FAILED_JOIN_QUEUE, IBC_LOCK, REJOIN_QUEUE, REPLIES, SHARES, START_UNBOND_QUEUE, TRAPS,
        UNBOND_QUEUE,
    },
    unbond::PendingReturningUnbonds,
};
use cosmwasm_std::{
    from_json, to_json_binary, Addr, BankMsg, Binary, CosmosMsg, DepsMut, Env, IbcMsg,
    IbcPacketAckMsg, Order, QuerierWrapper, Response, StdError, Storage, SubMsg, Uint128, WasmMsg,
};
use prost::Message;
use quasar_types::{callback::Callback, ibc::MsgTransferResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_total_primitive_shares(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let mut sum = Uint128::zero();
    for val in SHARES.range(storage, None, None, Order::Ascending) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

pub fn get_ica_address(store: &dyn Storage, channel_id: String) -> Result<String, ContractError> {
    let chan = CHANNELS.load(store, channel_id)?;
    match chan.channel_type {
        quasar_types::ibc::ChannelType::Icq { channel_ty: _ } => Err(ContractError::NoIcaChannel),
        quasar_types::ibc::ChannelType::Ica {
            channel_ty: _,
            counter_party_address,
        } => {
            if let Some(addr) = counter_party_address {
                Ok(addr)
            } else {
                Err(ContractError::NoCounterpartyIcaAddress)
            }
        }
        quasar_types::ibc::ChannelType::Ics20 { channel_ty: _ } => Err(ContractError::NoIcaChannel),
    }
}

pub fn check_icq_channel(storage: &dyn Storage, channel: String) -> Result<(), ContractError> {
    let chan = CHANNELS.load(storage, channel)?;
    match chan.channel_type {
        quasar_types::ibc::ChannelType::Icq { channel_ty: _ } => Ok(()),
        quasar_types::ibc::ChannelType::Ica {
            channel_ty: _,
            counter_party_address: _,
        } => Err(ContractError::NoIcqChannel),
        quasar_types::ibc::ChannelType::Ics20 { channel_ty: _ } => Err(ContractError::NoIcqChannel),
    }
}

pub fn create_callback_submsg(
    storage: &mut dyn Storage,
    cosmos_msg: CosmosMsg,
    owner: Addr,
    callback_id: String,
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }

    let local_denom = CONFIG.load(storage)?.local_denom;
    let data: SubMsgKind = match &cosmos_msg {
        CosmosMsg::Wasm(WasmMsg::Execute { msg, funds, .. }) => {
            SubMsgKind::Callback(ContractCallback::Callback {
                callback: from_json(msg)?,
                // if we send funds, we expect them to be in local denom
                amount: funds
                    .iter()
                    .find(|c| c.denom == local_denom)
                    .map(|val| val.amount),
                owner,
            })
        }
        CosmosMsg::Bank(bank_msg) => SubMsgKind::Callback(ContractCallback::Bank {
            bank_msg: bank_msg.to_owned(),
            unbond_id: callback_id,
        }),
        _ => return Err(StdError::generic_err("Unsupported WasmMsg")),
    };

    REPLIES.save(storage, id, &data)?;
    Ok(SubMsg::reply_always(cosmos_msg, id))
}

pub(crate) fn lock_try_icq(
    deps: DepsMut,
    sub_msg: Option<cosmwasm_std::SubMsg>,
) -> Result<Response, ContractError> {
    let mut res = Response::new();
    let mut lock = IBC_LOCK.load(deps.storage)?;

    if let Some(sub_msg) = sub_msg {
        if !BOND_QUEUE.is_empty(deps.storage)? {
            lock = lock.lock_bond();
            res = res.add_attribute("bond_queue", "locked");
        } else if !START_UNBOND_QUEUE.is_empty(deps.storage)? {
            lock = lock.lock_start_unbond();
            res = res.add_attribute("start_unbond_queue", "locked");
        } else if !UNBOND_QUEUE.is_empty(deps.storage)? {
            lock = lock.lock_unbond();
            res = res.add_attribute("unbond_queue", "locked");
        }
        if lock.is_unlocked() {
            res = res.add_attribute("IBC_LOCK", "unlocked");
        }
        IBC_LOCK.save(deps.storage, &lock)?;
        res = res.add_submessage(sub_msg);
        res = res.add_attribute("kind", "dispatch");
    } else {
        res = res.add_attribute("IBC_LOCK", "locked");
        res = res.add_attribute("kind", "queue");
    }
    Ok(res)
}

pub fn get_usable_compound_balance(
    storage: &dyn Storage,
    balance: Uint128,
) -> Result<Uint128, ContractError> {
    // two cases where we exclude funds, either transfer succeeded, but not ica, or transfer succeeded and subsequent ica failed
    let traps = TRAPS.range(storage, None, None, Order::Ascending);

    let failed_join_queue_amount = FAILED_JOIN_QUEUE.iter(storage)?.try_fold(
        Uint128::zero(),
        |acc, val| -> Result<Uint128, ContractError> { Ok(acc + val?.amount) },
    )?;

    let rejoin_queue_amount = REJOIN_QUEUE.iter(storage)?.try_fold(
        Uint128::zero(),
        |acc, val| -> Result<Uint128, ContractError> {
            match val?.raw_amount {
                crate::state::RawAmount::LocalDenom(amount) => Ok(amount + acc),
                crate::state::RawAmount::LpShares(_) => Err(ContractError::IncorrectRawAmount),
            }
        },
    )?;

    let trapped_errors_amount = traps.fold(Uint128::zero(), |acc, wrapped_trap| {
        let trap = wrapped_trap.unwrap().1;
        if trap.last_succesful {
            if let IbcMsgKind::Transfer { pending: _, amount } = trap.step {
                acc + amount
            } else {
                acc
            }
        // if last msg was not succesful, we did not join the pool, so we have base_denom funds on the
        } else if let IbcMsgKind::Ica(IcaMessages::JoinSwapExternAmountIn(pb)) = trap.step {
            pb.bonds.iter().fold(acc, |acc2, bond| {
                if let RawAmount::LocalDenom(local_denom_amount) = &bond.raw_amount {
                    acc2 + local_denom_amount
                } else {
                    acc2
                }
            })
        } else {
            acc
        }
    });

    let excluded_funds = failed_join_queue_amount
        .checked_add(rejoin_queue_amount)?
        .checked_add(trapped_errors_amount)?;

    Ok(balance.saturating_sub(excluded_funds))
}

pub fn create_ibc_ack_submsg(
    storage: &mut dyn Storage,
    pending: IbcMsgKind,
    msg: IbcMsg,
    channel: String,
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &SubMsgKind::Ibc(pending, channel))?;
    Ok(SubMsg::reply_always(msg, id))
}

pub fn ack_submsg(
    storage: &mut dyn Storage,
    env: Env,
    msg: IbcPacketAckMsg,
    channel: String,
) -> Result<SubMsg, ContractError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }

    // register the message in the replies for handling
    // TODO do we need this state item here? or do we just need the reply hook
    REPLIES.save(
        storage,
        id,
        &SubMsgKind::Ack(msg.original_packet.sequence, channel),
    )?;

    // TODO for an ack, should the reply hook be always or only on error? Probably only on error
    // On succeses, we need to cleanup the state item from REPLIES
    Ok(SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_json_binary(&ExecuteMsg::Ack { ack: msg })?,
            funds: vec![],
        },
        id,
    ))
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IbcMsgKind {
    Transfer {
        pending: PendingBond,
        amount: Uint128,
    },
    Ica(IcaMessages),
    Icq,
}

// All enums supported by this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IcaMessages {
    JoinSwapExternAmountIn(PendingBond),
    // pending bonds int the lock and total shares to be locked
    // should be gotten from the join pool
    LockTokens(PendingBond, Uint128),
    BeginUnlocking(Vec<PendingSingleUnbond>, Uint128),
    ExitPool(PendingReturningUnbonds),
    ReturnTransfer(PendingReturningUnbonds),
    RecoveryExitPool(PendingReturningRecovery),
    RecoveryReturnTransfer(PendingReturningRecovery),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubMsgKind {
    Ibc(IbcMsgKind, String),
    Ack(u64, String),
    Callback(ContractCallback), // in reply match for callback variant
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ContractCallback {
    Callback {
        callback: Callback,
        amount: Option<Uint128>,
        owner: Addr,
    },
    Bank {
        bank_msg: BankMsg,
        unbond_id: String,
    },
}

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub(crate) fn parse_seq(data: Binary) -> Result<u64, ContractError> {
    let resp = MsgTransferResponse::decode(data.0.as_slice())?;
    Ok(resp.seq)
}

pub(crate) fn unlock_on_error(
    storage: &mut dyn Storage,
    kind: &IbcMsgKind,
) -> Result<(), ContractError> {
    match kind {
        IbcMsgKind::Transfer {
            pending: _,
            amount: _,
        } => {
            IBC_LOCK.update(storage, |lock| {
                Ok::<Lock, ContractError>(lock.unlock_bond())
            })?;
            Ok(())
        }
        IbcMsgKind::Ica(ica) => match ica {
            IcaMessages::JoinSwapExternAmountIn(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_bond())
                })?;
                Ok(())
            }
            IcaMessages::LockTokens(_, _) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_bond())
                })?;
                Ok(())
            }
            IcaMessages::BeginUnlocking(_, _) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_start_unbond())
                })?;
                Ok(())
            }
            IcaMessages::ExitPool(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_unbond())
                })?;
                Ok(())
            }
            IcaMessages::ReturnTransfer(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_unbond())
                })?;
                Ok(())
            }
            IcaMessages::RecoveryExitPool(_) => todo!(),
            IcaMessages::RecoveryReturnTransfer(_) => todo!(),
        },
        IbcMsgKind::Icq => {
            IBC_LOCK.update(storage, |lock| {
                Ok::<Lock, ContractError>(lock.unlock_bond().unlock_start_unbond().unlock_unbond())
            })?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reply_seq() {
        let seq = 35;
        let resp = Binary::from(MsgTransferResponse { seq }.encode_to_vec());
        let parsed_seq = parse_seq(resp).unwrap();
        assert_eq!(seq, parsed_seq)
    }
}
