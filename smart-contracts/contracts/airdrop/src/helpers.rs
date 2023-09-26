use cosmwasm_std::{Addr, Env, Order, QuerierWrapper, Response, StdError, Storage, Uint128};

use crate::state::{AirdropConfig, AIRDROP_CONFIG, USER_INFO};
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

pub fn check_amounts_and_airdrop_size(
    total_in_user_info: Uint128,
    current_airdrop_amount: Uint128,
) -> Result<Response, AirdropErrors> {
    // Check if the total claimable amount exceeds the airdrop amount
    if total_in_user_info > current_airdrop_amount {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Total amount in the given user amounts ".to_string()
                + &*total_in_user_info.to_string()
                + &*" is greater than ".to_string()
                + &*current_airdrop_amount.to_string(),
        }));
    }
    Ok(Response::default())
}

pub fn validate_amount(amount: &Uint128, index: usize) -> Result<Response, AirdropErrors> {
    // Check if the total claimable amount exceeds the airdrop amount
    if amount != Uint128::zero() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Amount at index :".to_string() + &*index.to_string() + &*"is zero".to_string(),
        }));
    }
    Ok(Response::default())
}

pub fn validate_update_config(
    config: AirdropConfig,
    storage: &dyn Storage,
    querier: QuerierWrapper,
    env: Env,
) -> Result<Response, AirdropErrors> {
    // Check if the start height and end height are not zero,
    // indicating a valid airdrop window
    if config.start_height != 0 || config.end_height != 0 {
        // Check if the current block height is less than the start height
        // and if the start height is less than the end height
        if env.block.height < config.start_height && config.start_height < config.end_height {
            // Check if the airdrop amount is sufficient to supply all users
            if config.airdrop_amount >= get_total_in_user_info(storage) {
                // Get the contract's bank balance
                let current_airdrop_config = AIRDROP_CONFIG.load(storage)?;
                let contract_balance = current_airdrop_config
                    .airdrop_asset
                    .query_balance(&querier, &env.contract.address)?;

                // Check if the contract has enough funds for the airdrop
                if contract_balance < config.airdrop_amount {
                    return Err(AirdropErrors::Std(StdError::GenericErr {
                        msg:
                            "Failed due to insufficient balance in the contract account. Balance : "
                                .to_string()
                                + &contract_balance.to_string(),
                    }));
                }
            } else {
                return Err(AirdropErrors::Std(StdError::GenericErr {
                    msg: "Failed due to config has less amount than the amount allowed to the users to claim".to_string(),
                }));
            }
        } else {
            return Err(AirdropErrors::Std(StdError::GenericErr {
                msg: "Failed as the heights given do not satisfy the conditions".to_string(),
            }));
        }
    }
    Ok(Response::default())
}

pub fn get_total_in_user_info(storage: &dyn Storage) -> Uint128 {
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

    use crate::state::UserInfo;

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
