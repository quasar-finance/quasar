use cosmwasm_std::{StdError, Uint128};
use quasar_traits::traits::ShareDistributor;
use cw20::Cw20Coin;

/// DumbDistributor is a simple implementation of the ShareDistributor trait, it returns 1 share
/// per cw20 coin. This should probably not be used in any production environment, but is useful for
/// demonstration purposes.
/// The test distributor_does_not_work_uneven_amounts() demonstrates what is wrong with this implementation.
pub struct DumbDistributor {}

impl ShareDistributor for DumbDistributor {
    fn deposit_funds(&mut self, deposit: &Vec<Cw20Coin>, state: &Vec<Cw20Coin>) -> Result<Uint128, StdError> {
        if deposit.len() != state.len() {
            return Err(StdError::GenericErr { msg:"deposit and state must be the same length".into()});
        }
        if deposit.len() != 2 {
            return Err(StdError::GenericErr { msg:"deposit can only be two tokens".into()});
        }
        if state.len() != 2 {
            return Err(StdError::GenericErr { msg:"state can only be two tokens".into()});
        }
        if eq_token(&deposit, &state) {
            return Err(StdError::GenericErr { msg:"deposit and state must contain same tokens".into()});
        }
        Ok(deposit.iter().fold(Uint128::zero(), |acc, coin| {
            acc + Uint128::from(coin.amount)
        }))
    }

    // distribute shares on a 1:1 basis, this is silly, never do this in an actual contract. Actual implementation
    // probably needs to support some state withing the contract or the distributor.
    fn withdraw_funds(&mut self, shares: &Uint128, state: &Vec<Cw20Coin>) -> Result<Vec<Cw20Coin>, StdError> {
        if state.len() != 2 {
            return Err(StdError::GenericErr { msg:"state can only be two tokens".into()});
        }
        if shares > &state[0].amount || shares > &state[1].amount {
            return Err(StdError::GenericErr { msg: "not enough funds".to_string() });
        }
        let amount1 = if *shares % Uint128::from(2 as u8) == Uint128::zero() {
            *shares / Uint128::from(2 as u8)
        } else {
            (*shares / Uint128::from(2 as u8)) + Uint128::from(1 as u8)
        };
        let token1 = Cw20Coin {
            address: state[0].clone().address,
            amount: amount1
        };
        let token2 = Cw20Coin {
            address: state[1].clone().address,
            amount: *shares / Uint128::from(2 as u8)
        };
        Ok(vec![token1, token2])
    }
}

impl DumbDistributor {
    pub fn new() -> DumbDistributor {
        DumbDistributor{}
    }
}

fn eq_token(a: &Vec<Cw20Coin>, b: &Vec<Cw20Coin>) -> bool {
    if a[0].address != b[0].address || a[0].address != b[1].address {
        return false;
    }
    if a[1].address != b[0].address || a[1].address != b[1].address {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distributor_works_even_amounts() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(100) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(100)}];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(10000) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(10000)}];
        let mut dist = DumbDistributor::new();

        let shares = dist.deposit_funds(&funds, &state).unwrap();
        assert_eq!(shares, Uint128::new(200));
        let withdraw = dist.withdraw_funds(&shares, &state).unwrap();
        assert_eq!(withdraw, funds)
    }

    #[test]
    fn distributor_does_not_work_uneven_amounts() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(100) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(101)}];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(10000) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(10000)}];
        let mut dist = DumbDistributor::new();

        // Here we see why this is a bad implementation, we can deposit funds at some state, and not
        // get the same amount of shares when we withdraw funds at that same state
        let shares = dist.deposit_funds(&funds, &state).unwrap();
        assert_eq!(shares, Uint128::new(200));
        let withdraw = dist.withdraw_funds(&shares, &state).unwrap();
        assert_ne!(withdraw, funds)
    }
}