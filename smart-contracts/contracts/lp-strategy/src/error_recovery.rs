use cosmwasm_std::{Addr, DepsMut, Env, Response, Storage, SubMsg, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::{create_ibc_ack_submsg, IbcMsgKind, IcaMessages},
    ibc_util::calculate_token_out_min_amount,
    state::{FundPath, LpCache, PendingBond, RawAmount, LP_SHARES, TRAPS},
    unbond::{do_exit_swap, do_transfer_batch_unbond},
};

// start_recovery fetches an error from the TRAPPED_ERRORS and start the appropriate recovery from there
pub fn start_recovery(
    deps: DepsMut,
    env: &Env,
    error_sequence: u64,
) -> Result<Response, ContractError> {
    let error = TRAPS.load(deps.storage, error_sequence)?;
    match error.last_succesful {
        true => {
            match error.step {
                // if the transfer failed. The funds in pending are still located on Quasar, meaning we
                crate::helpers::IbcMsgKind::Transfer { pending, amount } => {
                    // cleanup error state to prevent multiple error recoveries
                    TRAPS.remove(deps.storage, error_sequence);
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
fn handle_transfer_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    bonds: PendingBond,
    amount: Uint128,
    trapped_id: u64,
) -> Result<SubMsg, ContractError> {
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

    let returning = PendingReturningRecovery {
        returning: returning?,
        trapped_id: trapped_id,
    };

    let msg = do_transfer_batch_unbond(storage, env, amount)?;
    Ok(create_ibc_ack_submsg(
        storage,
        IbcMsgKind::Ica(IcaMessages::RecoveryReturnTransfer(returning)),
        msg,
    )?)
}

fn handle_ica_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    ica: IcaMessages,
    last_succesful: bool,
    trapped_id: u64,
) -> Result<Response, ContractError> {
    match ica {
        IcaMessages::JoinSwapExternAmountIn(pending) => {
            handle_join_swap_recovery(storage, env, pending, trapped_id)?;
            todo!()
        }
        IcaMessages::LockTokens(_) => todo!(),
        IcaMessages::BeginUnlocking(_) => todo!(),
        IcaMessages::ExitPool(_) => todo!(),
        IcaMessages::ReturnTransfer(_) => todo!(),
        IcaMessages::RecoveryExitPool(_) => todo!(),
        IcaMessages::RecoveryReturnTransfer(_) => todo!(),
    }
}

// if the join_swap was succesful, the refund path means we have to
fn handle_join_swap_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    pending: PendingBond,
    trapped_id: u64,
) -> Result<SubMsg, ContractError> {
    let exits_res: Result<Vec<ReturningRecovery>, ContractError> = pending
        .bonds
        .iter()
        .map(|val| {
            if let RawAmount::LpShares(_amount) = val.raw_amount {
                Ok(ReturningRecovery {
                    amount: val.raw_amount.clone(),
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
    )?)
}

// fn create_recovery_submsg(
//     msg: IbcMsg,
//     kind: IbcMsgKind
// ) -> Result<SubMsg, ContractError> {

// }

fn handle_lock_recovery() {}

fn handle_begin_unlocking_recovery() {}

fn handle_exit_pool_recovery() {}

fn handle_return_transfer_recovery() {}

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
