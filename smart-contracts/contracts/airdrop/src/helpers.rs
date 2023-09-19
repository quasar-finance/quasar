use cosmwasm_std::{Addr, Api, Env, Order, QuerierWrapper, Storage, Uint128};

use crate::state::{UserInfo, USER_INFO};
use crate::AirdropErrors;

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), AirdropErrors> {
    // Get the contract admin address from the contract's information
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;

    // Check if the contract admin address exists
    if let Some(contract_admin) = contract_admin {
        // Compare the contract admin address with the provided sus_admin address
        if contract_admin != *sus_admin {
            // If they don't match, return an Unauthorized error
            return Err(AirdropErrors::Unauthorized {});
        }
    } else {
        // If the contract admin address doesn't exist, return an Unauthorized error
        return Err(AirdropErrors::Unauthorized {});
    }

    // If all checks pass, return Ok() to indicate success
    Ok(())
}

pub fn get_total_in_user_info(storage: &mut dyn Storage) -> Uint128 {
    let mut total_claimable_amount = Uint128::zero();

    for res in USER_INFO.range(storage, None, None, Order::Ascending) {
        total_claimable_amount += res.unwrap().1.get_claimable_amount()
    }

    // Return the total claimable amount
    total_claimable_amount
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;

    use super::*;

    #[test]
    fn test_get_total_in_user_info() {
        // Create a mock context and storage
        let mut deps = mock_dependencies();

        // Initialize the USER_INFO map in the mock storage with sample data
        USER_INFO
            .save(
                &mut deps.storage,
                "user1".parse().unwrap(),
                &UserInfo {
                    claimable_amount: Uint128::new(100),
                    claimed_flag: false,
                },
            )
            .unwrap();
        USER_INFO
            .save(
                &mut deps.storage,
                "user2".parse().unwrap(),
                &UserInfo {
                    claimable_amount: Uint128::new(200),
                    claimed_flag: false,
                },
            )
            .unwrap();

        // Call the function to be tested
        let total_claimable_amount = get_total_in_user_info(deps.as_mut().storage);

        // Check the result against the expected total claimable amount
        assert_eq!(total_claimable_amount, Uint128::new(300));
    }
}
