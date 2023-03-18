use std::fmt;

use cosmwasm_std::{DepsMut, Env, Response, Storage, SubMsg, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::IcaMessages,
    state::{PendingBond, RawAmount, TRAPS, LP_SHARES, LpCache},
    unbond::{transfer_batch_unbond, PendingReturningUnbonds, ReturningUnbond, do_exit_swap},
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
                    let msg = handle_transfer_recovery(deps.storage, env, pending, amount)?;
                    Ok(Response::new()
                        .add_submessage(msg)
                        .add_attribute("error-recover-sequence", error_sequence.to_string())
                        .add_attribute("error-recovery-value", error.last_succesful.to_string()))
                }
                crate::helpers::IbcMsgKind::Ica(ica) => todo!(),
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
) -> Result<SubMsg, ContractError> {
    let unbonds: Result<Vec<ReturningUnbond>, ContractError> = bonds
        .bonds
        .iter()
        .map(|bond| {
            if let RawAmount::LocalDenom(val) = bond.raw_amount {
                Ok(ReturningUnbond {
                    amount: RawAmount::LocalDenom(val),
                    owner: bond.owner.clone(),
                    id: bond.bond_id.clone(),
                })
            } else {
                Err(ContractError::ReturningTransferIncorrectAmount)
            }
        })
        .collect();

    let returning: PendingReturningUnbonds = PendingReturningUnbonds { unbonds: unbonds? };

    // TODO, assert that raw amounts equal amount
    transfer_batch_unbond(storage, env, returning, amount)
}

fn handle_ica_recovery(
    storage: &mut dyn Storage,
    env: &Env,
    ica: IcaMessages,
) -> Result<Response, ContractError> {
    match ica {
        IcaMessages::JoinSwapExternAmountIn(pending) => handle_join_swap_recovery,
        IcaMessages::LockTokens(_) => todo!(),
        IcaMessages::BeginUnlocking(_) => todo!(),
        IcaMessages::ExitPool(_) => todo!(),
        IcaMessages::ReturnTransfer(_) => todo!(),
    }
}

// if the join_swap was succesful, the refund path means we have to
fn handle_join_swap_recovery(storage: &mut dyn Storage, env: &Env, pending: PendingBond) -> Result<SubMsg, ContractError> {
    
    let exits: Result<Vec<ReturningUnbond>, ContractError> = pending.bonds.iter().map(|val| {
        if let RawAmount::LpShares(amount) = val.raw_amount {
            Ok(ReturningUnbond { amount: val.raw_amount.clone(), owner: val.owner.clone(), id: val.bond_id.clone() })
        } else {
            Err(ContractError::IncorrectRawAmount)
        }
    }).collect();
    let total_exit: Uint128 = exits?.iter().try_fold(Uint128::zero(), |acc, val| -> Result<Uint128, ContractError> {
        match val.amount  {
            RawAmount::LocalDenom(_) => unimplemented!(),
            RawAmount::LpShares(amount) => Ok(amount.checked_add(acc)?),
        }
    })?;


    LP_SHARES.update(storage, |mut old| -> Result<LpCache, ContractError> {
        // we remove the amount of shares we are are going to unlock from the locked amount
        old.d_unlocked_shares = old.d_unlocked_shares.checked_sub(total_exit)?;
        // we add the amount of shares we are going to unlock to the total unlocked
        old.w_unlocked_shares = old.w_unlocked_shares.checked_add(total_exit)?;
        Ok(old)
    })?;

    todo!()
    // do_exit_swap()
}

fn handle_lock_recovery() {}

fn handle_begin_unlocking_recovery() {}

fn handle_exit_pool_recovery() {}

fn handle_return_transfer_recovery() {}
