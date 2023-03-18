use crate::{
    error::ContractError,
    ibc_lock::Lock,
    msg::ExecuteMsg,
    state::{PendingBond, PendingSingleUnbond, CHANNELS, IBC_LOCK, REPLIES, SHARES},
    unbond::PendingReturningUnbonds,
};
use cosmwasm_std::{
    from_binary, to_binary, BankMsg, Binary, CosmosMsg, Env, IbcMsg, IbcPacketAckMsg, Order,
    StdError, Storage, SubMsg, Uint128, WasmMsg,
};
use prost::Message;
use quasar_types::{callback::Callback, ibc::MsgTransferResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_total_primitive_shares(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    // workaround for a div-by-zero error on multi-asset vault side
    let mut sum = Uint128::one();
    for val in SHARES.range(storage, None, None, Order::Ascending) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

pub fn get_raw_total_shares(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    // workaround for a div-by-zero error on multi-asset vault side
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
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }

    let data: SubMsgKind = match &cosmos_msg {
        CosmosMsg::Wasm(WasmMsg::Execute { msg, .. }) => {
            SubMsgKind::Callback(ContractCallback::Callback(from_binary(msg)?))
        }
        CosmosMsg::Bank(bank_msg) => {
            SubMsgKind::Callback(ContractCallback::Bank(bank_msg.to_owned()))
        }
        _ => return Err(StdError::generic_err("Unsupported WasmMsg")),
    };

    REPLIES.save(storage, id, &data)?;
    Ok(SubMsg::reply_always(cosmos_msg, id))
}

pub fn create_ibc_ack_submsg(
    storage: &mut dyn Storage,
    pending: IbcMsgKind,
    msg: IbcMsg,
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &SubMsgKind::Ibc(pending))?;
    Ok(SubMsg::reply_always(msg, id))
}

pub fn ack_submsg(
    storage: &mut dyn Storage,
    env: Env,
    msg: IbcPacketAckMsg,
) -> Result<SubMsg, ContractError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0 + 1;
    }

    // register the message in the replies for handling
    // TODO do we need this state item here? or do we just need the reply hook
    REPLIES.save(storage, id, &SubMsgKind::Ack(msg.original_packet.sequence))?;

    // TODO for an ack, should the reply hook be always or only on error? Probably only on error
    // On succeses, we need to cleanup the state item from REPLIES
    Ok(SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::Ack { ack: msg })?,
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
    LockTokens(PendingBond),
    BeginUnlocking(Vec<PendingSingleUnbond>),
    ExitPool(PendingReturningUnbonds),
    ReturnTransfer(PendingReturningUnbonds),
    RecoveryExitPool(PendingReturningUnbonds),
    RecoveryReturnTransfer(PendingReturningUnbonds),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubMsgKind {
    Ibc(IbcMsgKind),
    Ack(u64),
    Callback(ContractCallback), // in reply match for callback variant
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ContractCallback {
    Callback(Callback),
    Bank(BankMsg),
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
            IcaMessages::LockTokens(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_bond())
                })?;
                Ok(())
            }
            IcaMessages::BeginUnlocking(_) => {
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
