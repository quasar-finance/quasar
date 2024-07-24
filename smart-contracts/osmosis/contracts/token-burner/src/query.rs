use crate::BurnErrors;
use cosmwasm_std::{Coin, Deps, Order, StdResult};

use crate::msg::TotalBurntResponse;
use crate::state::AMOUNT_BURNT;

pub fn query_total_burn(deps: Deps) -> StdResult<TotalBurntResponse> {
    let total_burn_amount: Result<Vec<Coin>, BurnErrors> = AMOUNT_BURNT
        .range(deps.storage, None, None, Order::Ascending)
        .map(|result| {
            result
                .map_err(BurnErrors::from)
                .map(|(denom, amount)| Coin { denom, amount })
        })
        .collect();

    Ok(TotalBurntResponse {
        amount: total_burn_amount.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn test_query_total_burn_empty() {
        // Create a mock dependency with empty storage
        let deps = mock_dependencies();

        // Call the query function
        let res = query_total_burn(deps.as_ref()).unwrap();

        // Check the result
        assert!(res.amount.is_empty());
    }

    #[test]
    fn test_query_total_burn_single_entry() {
        // Create a mock dependency with empty storage
        let mut deps = mock_dependencies();

        // Set up some example data in the map
        let denom = "denom1".to_string();
        let amount = Uint128::new(100);
        AMOUNT_BURNT
            .save(deps.as_mut().storage, denom.clone(), &amount)
            .unwrap();

        // Call the query function
        let res = query_total_burn(deps.as_ref()).unwrap();

        // Check the result
        assert_eq!(res.amount.len(), 1);
        assert_eq!(
            res.amount[0],
            Coin {
                denom: denom.clone(),
                amount
            }
        );
    }

    #[test]
    fn test_query_total_burn_ordered() {
        // Create a mock dependency with empty storage
        let mut deps = mock_dependencies();

        // Set up some example data in the map
        let denom1 = "denom1".to_string();
        let denom2 = "denom2".to_string();
        let amount1 = Uint128::new(100);
        let amount2 = Uint128::new(200);
        AMOUNT_BURNT
            .save(deps.as_mut().storage, denom2.clone(), &amount2)
            .unwrap();
        AMOUNT_BURNT
            .save(deps.as_mut().storage, denom1.clone(), &amount1)
            .unwrap();

        // Call the query function
        let res = query_total_burn(deps.as_ref()).unwrap();

        // Check the result
        assert_eq!(res.amount.len(), 2);

        // Check if the result is ordered by denomination
        assert_eq!(
            res.amount[0],
            Coin {
                denom: denom1.clone(),
                amount: amount1
            }
        );
        assert_eq!(
            res.amount[1],
            Coin {
                denom: denom2.clone(),
                amount: amount2
            }
        );
    }
}
