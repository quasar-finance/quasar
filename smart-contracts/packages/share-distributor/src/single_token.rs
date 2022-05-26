use cosmwasm_std::{OverflowError, OverflowOperation, StdError, Uint128};
use cw20::Cw20Coin;
use quasar_traits::traits::ShareDistributor;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SingleToken {}

impl ShareDistributor for SingleToken {
    fn deposit_funds(&mut self, deposit: &Vec<Cw20Coin>, state: &Vec<Cw20Coin>) -> Result<Uint128, StdError> {
        if state.len() != 1  || deposit.len() != 1{
            return Err(StdError::GenericErr {msg: "state can only be a single token".to_string()})
        }
        // make sure that we can add the deposited funds to the state later
        if deposit[0].amount.checked_add(state[0].amount).is_err() {
            return Err(StdError::Overflow { source: OverflowError {
                operation: OverflowOperation::Add,
                operand1: deposit[0].amount.to_string(),
                operand2: state[0].amount.to_string()
            } })
        }
        Ok(deposit[0].amount)
    }

    fn withdraw_funds(&mut self, shares: &Uint128, state: &Vec<Cw20Coin>) -> Result<Vec<Cw20Coin>, StdError> {
        if state.len() != 1 {
            return Err(StdError::GenericErr {msg: "state can only be a single token".to_string()})
        }
        // if we have more shares than funds in the current state of the contract, we return an error
        if shares > &state[0].amount {
            return Err(StdError::GenericErr { msg: "not enough funds in state for amount of shares".to_string() })
        }
        Ok(vec![Cw20Coin{address: state[0].clone().address, amount: shares.clone()}])
    }
}

impl SingleToken {
    pub fn new() -> SingleToken {
        SingleToken{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_tokens_fail() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(100) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(100)}];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(10000) }, Cw20Coin{ address: "another_token".to_string(), amount: Uint128::new(10000)}];
        let mut dist = SingleToken::new();

        let shares = dist.deposit_funds(&funds, &state).unwrap_err();
        assert_eq!(shares, StdError::GenericErr { msg: "state can only be a single token".to_string() });
        let withdraw = dist.withdraw_funds(&Uint128::new(200), &state).unwrap_err();
        assert_eq!(withdraw, StdError::GenericErr { msg: "state can only be a single token".to_string() });
    }

    #[test]
    fn insufficient_managed_funds_fail() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(100) }];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(10) }];
        let mut dist = SingleToken::new();

        // We should be able to deposit more funds
        let shares = dist.deposit_funds(&funds, &state).unwrap();
        assert_eq!(shares, Uint128::new(100));
        // we should not be able to deposit more funds
        let withdraw = dist.withdraw_funds(&Uint128::new(200), &state).unwrap_err();
        assert_eq!(withdraw, StdError::GenericErr { msg: "not enough funds in state for amount of shares".to_string() });
    }

    #[test]
    fn distributor_does_not_overflow() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::MAX }];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::MAX }];
        let mut dist = SingleToken::new();


        let shares = dist.deposit_funds(&funds, &state).unwrap_err();
        assert_eq!(shares, StdError::Overflow { source: OverflowError {
            operation: OverflowOperation::Add,
            operand1: funds[0].amount.to_string(),
            operand2: state[0].amount.to_string()
        } });
        let withdraw = dist.withdraw_funds(&Uint128::MAX, &state).unwrap();
        assert_eq!(withdraw, funds);
    }

    #[test]
    fn distributor_works() {
        let funds = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(100) }];
        let state = vec![Cw20Coin{ address: "some_token".to_string(), amount: Uint128::new(1000) }];
        let mut dist = SingleToken::new();


        let shares = dist.deposit_funds(&funds, &state).unwrap();
        assert_eq!(shares, Uint128::new(100));
        let withdraw = dist.withdraw_funds(&shares, &state).unwrap();
        assert_eq!(withdraw, funds);
    }
}