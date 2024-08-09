use crate::ContractError;
use cosmwasm_std::{coin, Coin};

pub(crate) fn must_pay_one_or_two(
    funds: &[Coin],
    denoms: (String, String),
) -> Result<(Coin, Coin), ContractError> {
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
