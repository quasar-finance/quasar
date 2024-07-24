use cosmwasm_std::{Addr, Env, Order, QuerierWrapper, Response, Storage, Uint128};

use crate::error::AirdropErrors;
use crate::msg::User;
use crate::state::{AirdropConfig, AIRDROP_CONFIG, USER_INFO};

/// Checks if the sender is the contract admin. Returns an error if not authorized.
///
/// # Arguments
///
/// * `querier` - QuerierWrapper to query contract admin information.
/// * `env` - Environment information.
/// * `sus_admin` - Address of the sender.
///
/// # Returns
///
/// Returns `Ok(())` if the sender is authorized as the contract admin, otherwise returns an Unauthorized error.
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

/// Checks if the total claimable amount exceeds the airdrop amount.
///
/// # Arguments
///
/// * `total_in_user_info` - Total claimable amount from all users.
/// * `current_airdrop_amount` - Current airdrop amount in the contract.
///
/// # Returns
///
/// Returns a default response if the total claimable amount does not exceed the airdrop amount,
/// otherwise returns an error indicating insufficient funds.
pub fn check_amounts_and_airdrop_size(
    total_in_user_info: Uint128,
    current_airdrop_amount: Uint128,
) -> Result<Response, AirdropErrors> {
    // Check if the total claimable amount exceeds the airdrop amount
    if total_in_user_info > current_airdrop_amount {
        return Err(AirdropErrors::UserAmountIsGreaterThanTotal {
            total_in_user_info,
            current_airdrop_amount,
        });
    }
    Ok(Response::default())
}

/// Validates that an amount is not zero.
///
/// # Arguments
///
/// * `amount` - Amount to validate.
/// * `index` - Index of the amount in a list (used for error message).
///
/// # Returns
///
/// Returns a default response if the amount is not zero, otherwise returns an error indicating a zero amount.
pub fn validate_amount(user: User) -> Result<Response, AirdropErrors> {
    // Check if the amount is not zero
    if user.amount == Uint128::zero() {
        return Err(AirdropErrors::ZeroAmount {
            address: user.address,
        });
    }
    Ok(Response::default())
}

/// Validates and checks the airdrop configuration update.
///
/// # Arguments
///
/// * `config` - New airdrop configuration to validate.
/// * `storage` - Storage to access contract state.
/// * `querier` - QuerierWrapper to query contract state.
/// * `env` - Environment information.
///
/// # Returns
///
/// Returns a default response if the configuration update is valid, otherwise returns an error
/// indicating the reason for the validation failure.
pub fn validate_update_config(
    config: AirdropConfig,
    storage: &dyn Storage,
    querier: QuerierWrapper,
    env: Env,
) -> Result<Response, AirdropErrors> {
    // Check if the start height and end height are not zero,
    // indicating a valid airdrop window
    if config.total_claimed == Uint128::zero() {
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
                        return Err(AirdropErrors::InsufficientFundsInContractAccount {
                            balance: contract_balance,
                        });
                    }
                } else {
                    return Err(AirdropErrors::ConfigAmountLessThanTotalClaimable {});
                }
            } else {
                return Err(AirdropErrors::InvalidAirdropWindow {});
            }
        }
    } else {
        return Err(AirdropErrors::NonZeroClaimedAmount {});
    }
    Ok(Response::default())
}

/// Calculates the total claimable amount from all users.
///
/// # Arguments
///
/// * `storage` - Storage to access user information.
///
/// # Returns
///
/// Returns the total claimable amount from all users.
pub fn get_total_in_user_info(storage: &dyn Storage) -> Uint128 {
    let mut total_claimable_amount = Uint128::zero();

    for res in USER_INFO.range(storage, None, None, Order::Ascending) {
        let claimed = res.as_ref().unwrap().1.get_claimed_flag();
        if !claimed {
            total_claimable_amount += res.unwrap().1.get_claimable_amount()
        }
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

    #[test]
    fn test_validate_amount() {
        let err = validate_amount(User {
            address: "test".to_string(),
            amount: Uint128::new(0),
        })
        .unwrap_err();
        assert_eq!(
            AirdropErrors::ZeroAmount {
                address: "test".to_string()
            },
            err
        );
        let _resp = validate_amount(User {
            address: "test".to_string(),
            amount: Uint128::new(10),
        })
        .unwrap();
    }

    // #[test]
    // fn test_is_contract_admin() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info = mock_info("admin", &[Coin::new(100000000, "uqsr")]);
    //
    //
    //     let qx: MockQuerier<Empty> = MockQuerier::new(&[]);
    //     let q = QuerierWrapper::new(&qx);
    //
    //
    //
    //     let a= is_contract_admin(&q, &env, &Addr::unchecked("admin"));
    //     println!("{:?}", a);
    // }
}
