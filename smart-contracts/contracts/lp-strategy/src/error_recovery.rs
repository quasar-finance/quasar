use cosmwasm_std::{
    from_binary, Addr, DepsMut, Env, IbcAcknowledgement, Response, Storage, SubMsg, Uint128,
};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;
use quasar_types::{
    ibc::IcsAck,
    ica::{packet::AckBody, traits::Unpack},
    types::ItemShouldLoad,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, IbcMsgKind, IcaMessages},
    ibc_util::calculate_token_out_min_amount,
    state::{
        FundPath, LpCache, PendingBond, RawAmount, CONFIG, ICA_CHANNEL, LP_SHARES, RECOVERY_ACK,
        TRAPS,
    },
    unbond::do_exit_swap,
};

// start_recovery fetches an error from the TRAPPED_ERRORS and start the appropriate recovery from there
pub fn _start_recovery(
    deps: DepsMut,
    env: &Env,
    error_sequence: u64,
    channel: String,
) -> Result<Response, ContractError> {
    let error = TRAPS.load(deps.storage, (error_sequence, channel.clone()))?;
    match error.last_succesful {
        true => {
            match error.step {
                // if the transfer failed. The funds in pending are still located on Quasar, meaning we
                crate::helpers::IbcMsgKind::Transfer { pending, amount } => {
                    // cleanup error state to prevent multiple error recoveries
                    TRAPS.remove(deps.storage, (error_sequence, channel));
                    let msg = handle_transfer_recovery(
                        deps.storage,
                        env,
                        pending,
                        amount,
                        error_sequence,
                    )?;
                    Ok(Response::new()
                        .add_submessage(msg)
                        .add_attribute("error-recover-sequence", error_sequence.to_string())
                        .add_attribute("error-recovery-value", error.last_succesful.to_string()))
                }
                crate::helpers::IbcMsgKind::Ica(_ica) => todo!(),
                // if ICQ was the last successful step, all we failed trying to empty our queues and dispatching any following
                // IBC messages, meaning we don't have to do anything with a seperate try_icq endpoint
                crate::helpers::IbcMsgKind::Icq => unimplemented!(),
            }
        }
        false => {
            todo!()
        }
    }
}

// amount is the total amount we will try to transfer back to quasar, pending bond is the users the funds should return back to
#[allow(dead_code)]
fn handle_transfer_recovery(
    storage: &mut dyn Storage,
    _env: &Env,
    bonds: PendingBond,
    _amount: Uint128,
    trapped_id: u64,
) -> Result<SubMsg, ContractError> {
    let _config = CONFIG.should_load(storage)?;
    let returning: Result<Vec<ReturningRecovery>, ContractError> = bonds
        .bonds
        .iter()
        .map(|bond| {
            if let RawAmount::LocalDenom(val) = bond.raw_amount {
                Ok(ReturningRecovery {
                    amount: RawAmount::LocalDenom(val),
                    owner: bond.owner.clone(),
                    id: FundPath::Bond {
                        id: bond.bond_id.clone(),
                    },
                })
            } else {
                Err(ContractError::ReturningTransferIncorrectAmount)
            }
        })
        .collect();

    let _returning = PendingReturningRecovery {
        returning: returning?,
        trapped_id,
    };

    // big TODO here, below should be modified for error recovery criteria
    // leaving as todo because this code is not live yet.
    // we want to call do_transfer_batch_unbond with the returning unbonds but in the context of error recovery

    // let msg = todo!();
    // Ok(create_ibc_ack_submsg(
    //     storage,
    //     IbcMsgKind::Ica(IcaMessages::RecoveryReturnTransfer(returning)),
    //     msg,
    //     _config.transfer_channel,
    // )?)
    todo!()
}

fn _handle_last_succesful_ica_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    ica: IcaMessages,
    _last_succesful: bool,
    trapped_id: u64,
) -> Result<SubMsg, ContractError> {
    match ica {
        // in every succesful ica recovery, our last message was succesful, but our pending state is not,
        // For a join swap, we need to exit all shares
        IcaMessages::JoinSwapExternAmountIn(pending) => {
            handle_join_swap_recovery(storage, env, pending, trapped_id)
        }
        IcaMessages::LockTokens(_, _) => todo!(),
        IcaMessages::BeginUnlocking(_, _) => todo!(),
        IcaMessages::ExitPool(_) => todo!(),
        IcaMessages::ReturnTransfer(_) => todo!(),
        IcaMessages::RecoveryExitPool(_) => todo!(),
        IcaMessages::RecoveryReturnTransfer(_) => todo!(),
    }
}
fn _handle_last_failed_ica_recovery(
    _storage: &mut dyn Storage,
    _env: &Env,
    ica: IcaMessages,
    _trapped_id: u64,
) -> Result<SubMsg, ContractError> {
    match ica {
        // recover by sending funds back
        // since the ica failed, we should transfer the funds back, we do not yet expect a raw amount to be set to lp shares
        // how do we know how much to return here?
        IcaMessages::JoinSwapExternAmountIn(_pending) => {
            todo!()
        }
        IcaMessages::LockTokens(_, _) => todo!(),
        // if BeginUnlocking followup failed, our tokens, the amount of tokens in the request was actually succesful,
        // so we continue to a recovery exit swap
        IcaMessages::BeginUnlocking(_, _) => todo!(),
        // if the exit pool was actually succesful, we need to deserialize the saved ack result again to get the amount of tokens
        // users should get
        // TODO, can they get rejoined to the lp pool here? Maybe????
        // probably gets compounded back again, how do we know here?, do we know at all?
        // we let the exit pool get autocompounded by the contract again, to recover from here, we start a start_unlock for all stuck funds
        IcaMessages::ExitPool(_pending) => {
            todo!()
            // let msg = do_begin_unlocking(storage, env, to_unbond)?;
        }
        // we just retry the transfer here
        IcaMessages::ReturnTransfer(_) => todo!(),
        // same as regular exit pool recovery
        IcaMessages::RecoveryExitPool(_) => todo!(),
        // same as regular transfer recovery
        IcaMessages::RecoveryReturnTransfer(_) => todo!(),
    }
}

// kinda messed up that we create duplicate code here, should be solved with a single unpacking function that accepts
// a closure for IcsAck::Result and IcsAck::Error
#[allow(dead_code)]
fn de_succcesful_join(
    ack_bin: IbcAcknowledgement,
) -> Result<MsgJoinSwapExternAmountInResponse, ContractError> {
    let ack: IcsAck = from_binary(&ack_bin.data)?;
    if let IcsAck::Result(val) = ack {
        let ack_body = AckBody::from_bytes(val.0.as_ref())?.to_any()?;
        let ack = MsgJoinSwapExternAmountInResponse::unpack(ack_body)?;
        Ok(ack)
    } else {
        Err(ContractError::IncorrectRecoveryAck)
    }
}

// if the join_swap was succesful, the refund path means we have to
#[allow(dead_code)]
fn handle_join_swap_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    pending: PendingBond,
    trapped_id: u64,
) -> Result<SubMsg, ContractError> {
    let channel = ICA_CHANNEL.should_load(storage)?;
    let ack_bin = RECOVERY_ACK.load(storage, (trapped_id, channel.clone()))?;
    // in this case the recovery ack should contain a joinswapexternamountin response
    // we try to deserialize it
    let join_result = de_succcesful_join(ack_bin)?;

    // we expect the total amount here to be in local_denom since, although the join was succesful
    // the RawAmount cannot yet have been up updated
    let total_lp = pending.bonds.iter().try_fold(Uint128::zero(), |acc, val| {
        if let RawAmount::LocalDenom(amount) = val.raw_amount {
            Ok(acc.checked_add(amount)?)
        } else {
            Err(ContractError::IncorrectRawAmount)
        }
    })?;
    // now we need to divide up the lp shares amount our users according to the individual local denom amount
    let exits_res: Result<Vec<ReturningRecovery>, ContractError> = pending
        .bonds
        .iter()
        .map(|val| {
            // since the ICA followup failed, we need to figure out how to convert
            if let RawAmount::LocalDenom(amount) = val.raw_amount {
                Ok(ReturningRecovery {
                    // lp_shares_i = tokens_i * lp_total / tokens_total
                    amount: RawAmount::LpShares(amount.checked_mul(total_lp)?.checked_div(
                        Uint128::new(join_result.share_out_amount.parse().map_err(|err| {
                            ContractError::ParseIntError {
                                error: format!("join_swap_recovery:{err}"),
                                value: join_result.share_out_amount.clone(),
                            }
                        })?),
                    )?),
                    owner: val.owner.clone(),
                    // since we are recovering from a join swap, we need do save
                    // as a Bond for bookkeeping on returned
                    id: FundPath::Bond {
                        id: val.bond_id.clone(),
                    },
                })
            } else {
                Err(ContractError::IncorrectRawAmount)
            }
        })
        .collect();

    let exits = exits_res?;
    let total_exit: Uint128 = exits.iter().try_fold(
        Uint128::zero(),
        |acc, val| -> Result<Uint128, ContractError> {
            match val.amount {
                RawAmount::LocalDenom(_) => unimplemented!(),
                RawAmount::LpShares(amount) => Ok(amount.checked_add(acc)?),
            }
        },
    )?;

    LP_SHARES.update(storage, |mut old| -> Result<LpCache, ContractError> {
        // we remove the amount of shares we are are going to unlock from the locked amount
        old.d_unlocked_shares = old.d_unlocked_shares.checked_sub(total_exit)?;
        // we add the amount of shares we are going to unlock to the total unlocked
        old.w_unlocked_shares = old.w_unlocked_shares.checked_add(total_exit)?;
        Ok(old)
    })?;

    // TODO update me
    let locked_shares = Uint128::from(100u128);

    let token_out_min_amount =
        calculate_token_out_min_amount(storage, total_exit, locked_shares).unwrap();

    let exit = do_exit_swap(storage, env, token_out_min_amount, total_exit)?;
    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::RecoveryExitPool(PendingReturningRecovery {
            returning: exits,
            trapped_id,
        })),
        exit,
        channel,
    )?)
}

// fn create_recovery_submsg(
//     msg: IbcMsg,
//     kind: IbcMsgKind
// ) -> Result<SubMsg, ContractError> {

// }

fn _handle_lock_recovery() {}

fn _handle_begin_unlocking_recovery() {}

fn _handle_exit_pool_recovery() {}

fn _handle_return_transfer_recovery() {}

// TODO refactor bonds/unbonds to a single struct item that is bidirectional with a direction enum
// because we did not abstract nicely, we need a separate struct here
// we should feel bad
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PendingReturningRecovery {
    pub returning: Vec<ReturningRecovery>,
    pub trapped_id: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ReturningRecovery {
    pub amount: RawAmount,
    pub owner: Addr,
    pub id: FundPath,
}
