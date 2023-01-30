use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage, SubMsg, Uint128};
use cw_utils::must_pay;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::ContractError,
    helpers::get_total_shares,
    icq::try_icq,
    state::{OngoingDeposit, RawAmount, BONDING_CLAIMS, BOND_QUEUE, CONFIG, SHARES},
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Bond {
    pub amount: Uint128,
    pub owner: Addr,
    pub bond_id: String,
}

impl Bond {
    fn validate(&self) -> Result<(), ContractError> {
        Ok(())
    }
}

// A deposit starts of by querying the state of the ica counterparty contract
pub fn do_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    bond_id: String,
) -> Result<Option<SubMsg>, ContractError> {
    let amount = must_pay(&info, &CONFIG.load(deps.storage)?.local_denom)?;

    BOND_QUEUE.push_back(
        deps.storage,
        &Bond {
            amount: amount,
            owner: info.sender,
            bond_id,
        },
    )?;

    try_icq(deps.storage, env)
}

/// fold_queue folds the queue and attributes shares to the depositors according to the given total value
pub fn batch_bond(
    storage: &mut dyn Storage,
    total_balance: Uint128,
) -> Result<(Uint128, Vec<OngoingDeposit>), ContractError> {
    let mut total = Uint128::zero();
    let mut deposits: Vec<OngoingDeposit> = vec![];
    while !BOND_QUEUE.is_empty(storage)? {
        let item: Bond = BOND_QUEUE
            .pop_front(storage)?
            .ok_or(ContractError::QueueItemNotFound)?;
        let claim_amount = create_claim(storage, item.amount, item.owner.clone(), total_balance)?;
        total = total.checked_add(item.amount)?;
        deposits.push(OngoingDeposit {
            claim_amount,
            owner: item.owner,
            raw_amount: RawAmount::LocalDenom(item.amount),
            bond_id: item.bond_id,
        });
    }
    Ok((total, deposits))
}

// create_claim
fn create_claim(
    storage: &mut dyn Storage,
    user_balance: Uint128,
    address: Addr,
    total_balance: Uint128,
) -> Result<Uint128, ContractError> {
    let total_shares = get_total_shares(storage)?;

    // calculate the correct size of the claim
    let claim_amount = calculate_claim(user_balance, total_balance, total_shares)?;
    BONDING_CLAIMS.save(storage, address, &claim_amount)?;
    Ok(claim_amount)
}

// create a share and remove the amount from the claim
pub fn create_share(
    storage: &mut dyn Storage,
    owner: Addr,
    amount: Uint128,
) -> Result<Uint128, ContractError> {
    let claim = BONDING_CLAIMS.load(storage, owner.clone())?;
    if claim < amount {
        return Err(ContractError::InsufficientClaims);
    }

    if claim <= amount {
        BONDING_CLAIMS.remove(storage, owner.clone());
    } else {
        BONDING_CLAIMS.save(storage, owner.clone(), &claim.checked_sub(amount)?)?;
    }

    // TODO do we want to make shares fungible using cw20? if so, call into the minter and mint shares for the according to the claim
    SHARES.save(storage, owner, &amount)?;
    Ok(claim)
}

/// calculate the amount of for the claim of the user
/// user_shares = (user_balance / vault_balance) * vault_total_shares = (user_balance * vault_total_shares) / vault_balance
fn calculate_claim(
    user_balance: Uint128,
    total_balance: Uint128,
    total_shares: Uint128,
) -> Result<Uint128, ContractError> {
    Ok(user_balance
        .checked_mul(total_shares)?
        .checked_div(total_balance)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO rewrite this to a proptest
    #[test]
    fn calculate_claim_works() {
        let val = calculate_claim(Uint128::new(10), Uint128::new(100), Uint128::new(10)).unwrap();
        assert_eq!(val, Uint128::one())
    }
}
