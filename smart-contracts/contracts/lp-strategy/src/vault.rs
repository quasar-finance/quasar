use cosmwasm_std::{Coin, Uint128, Response};

use crate::{error::ContractError, state::Claim};



pub fn deposit(funds: Vec<Coin>) -> Result<Response, ContractError> {
    // deposit need to internally rebuild the amount of funds under the smart contract, can this be just deposited + already autocompounded?
    todo!()
}

// create_claim 
fn create_claim(total_balance: Uint128) -> String {
    todo!()
}

fn create_share(claim: Claim) -> Result<Response, ContractError> {
    // call into the minter and mint shares for the according to the claim
    todo!()
}

/// calculate the amount of for the claim of the user
/// user_shares = (user_balance / vault_balance) * vault_total_shares = (user_balance * vault_total_shares) / vault_balance
fn calculate_claim(user_balance: Uint128, total_balance: Uint128, total_shares: Uint128) -> Result<Uint128, ContractError> {
    Ok(user_balance.checked_mul(total_shares)?.checked_div(total_balance)?)
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