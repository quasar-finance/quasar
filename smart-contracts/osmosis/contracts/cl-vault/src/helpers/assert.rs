use cosmwasm_std::{coin, Addr, Coin, Deps, Storage};

use crate::{
    state::{ADMIN_ADDRESS, RANGE_ADMIN},
    ContractError,
};

/// This function compares the address of the message sender (caller) with the current admin
/// address stored in the state. This provides a convenient way to verify if the caller
/// is the admin in a single line.
pub fn assert_admin(deps: Deps, caller: &Addr) -> Result<Addr, ContractError> {
    if ADMIN_ADDRESS.load(deps.storage)? != caller {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(caller.clone())
    }
}

pub fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

/// Returns the Coin of the needed denoms in the order given in denoms
pub(crate) fn must_pay_one_or_two(
    funds: &[Coin],
    denoms: (String, String),
) -> Result<(Coin, Coin), ContractError> {
    if funds.len() != 2 && funds.len() != 1 {
        return Err(ContractError::IncorrectAmountFunds);
    }

    let token0 = funds
        .iter()
        .find(|coin| coin.denom == denoms.0)
        .cloned()
        .unwrap_or(coin(0, denoms.0));

    let token1 = funds
        .iter()
        .find(|coin| coin.denom == denoms.1)
        .cloned()
        .unwrap_or(coin(0, denoms.1));

    Ok((token0, token1))
}

pub(crate) fn must_pay_one_or_two_from_balance(
    funds: Vec<Coin>,
    denoms: (String, String),
) -> Result<(Coin, Coin), ContractError> {
    if funds.len() < 2 {
        return Err(ContractError::IncorrectAmountFunds);
    }

    let token0 = funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.0)
        .unwrap_or(coin(0, denoms.0));

    let token1 = funds
        .clone()
        .into_iter()
        .find(|coin| coin.denom == denoms.1)
        .unwrap_or(coin(0, denoms.1));

    Ok((token0, token1))
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::coin;

    use super::*;

    #[test]
    fn must_pay_one_or_two_works_ordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let funds = vec![expected0.clone(), expected1.clone()];
        let (token0, token1) =
            must_pay_one_or_two(&funds, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_one_or_two_works_unordered() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let funds = vec![expected0.clone(), expected1.clone()];
        let (token0, token1) =
            must_pay_one_or_two(&funds, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!(expected0, token0);
        assert_eq!(expected1, token1);
    }

    #[test]
    fn must_pay_one_or_two_rejects_three() {
        let expected0 = coin(100, "uatom");
        let expected1 = coin(200, "uosmo");
        let funds = vec![expected0, expected1, coin(200, "uqsr")];
        let _err =
            must_pay_one_or_two(&funds, ("uatom".to_string(), "uosmo".to_string())).unwrap_err();
    }

    #[test]
    fn must_pay_one_or_two_accepts_second_token() {
        let funds = vec![coin(200, "uosmo")];
        let res = must_pay_one_or_two(&funds, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!((coin(0, "uatom"), coin(200, "uosmo")), res)
    }

    #[test]
    fn must_pay_one_or_two_accepts_first_token() {
        let funds = vec![coin(200, "uatom")];
        let res = must_pay_one_or_two(&funds, ("uatom".to_string(), "uosmo".to_string())).unwrap();
        assert_eq!((coin(200, "uatom"), coin(0, "uosmo")), res)
    }
}
