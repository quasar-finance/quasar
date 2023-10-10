use std::string::String;

use cosmwasm_std::{Attribute, DepsMut, Env, Event, Response, StdError, Uint128};
use cw_asset::Asset;

use crate::helpers::{
    check_amounts_and_airdrop_size, get_total_in_user_info, validate_amount, validate_update_config,
};
use crate::state::{AirdropConfig, UserInfo, AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

/// Updates the airdrop configuration of the contract.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `env` - The current contract execution environment.
/// * `config` - The new airdrop configuration to be set.
///
/// # Errors
///
/// Returns an error if the airdrop has already ended or if the new configuration is invalid.
///
/// # Returns
///
/// Returns a response indicating the success of the update operation and includes
/// relevant attributes in the event.
pub fn execute_update_airdrop_config(
    deps: DepsMut,
    env: Env,
    config: AirdropConfig,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    if current_airdrop_config.is_airdrop_active(env.block.height) {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Failed to execute update as airdrop is active".to_string(),
        }));
    }

    // Check if an airdrop has been executed on the contract and if the update is allowed
    if current_airdrop_config.end_height != 0
        && env.block.height > current_airdrop_config.end_height
    {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Failed to execute update as it is post airdrop ending".to_string(),
        }));
    }

    // Check various conditions to validate the airdrop configuration update
    validate_update_config(config.clone(), deps.storage, deps.querier, env)?;

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &config)?;

    // Build attributes for the event
    let attributes: Vec<Attribute> = vec![
        Attribute {
            key: "description".to_string(),
            value: config.airdrop_description.to_string(),
        },
        Attribute {
            key: "airdrop_amount".to_string(),
            value: config.airdrop_amount.to_string(),
        },
        Attribute {
            key: "airdrop_asset".to_string(),
            value: config.airdrop_asset.to_string(),
        },
        Attribute {
            key: "claimed".to_string(),
            value: config.total_claimed.to_string(),
        },
        Attribute {
            key: "start_height".to_string(),
            value: config.start_height.to_string(),
        },
        Attribute {
            key: "end_height".to_string(),
            value: config.end_height.to_string(),
        },
    ];

    // Return a default response to indicate success with an "update_airdrop_config" event
    Ok(Response::default()
        .add_event(Event::new("update_airdrop_config").add_attributes(attributes)))
}

/// Adds new users and their respective amounts to the airdrop configuration.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `users` - A vector of user addresses to be added.
/// * `amounts` - A vector of amounts to be allocated to each user.
///
/// # Errors
///
/// Returns an error if the airdrop window is not open, the number of users and amounts provided do not match,
/// or if any of the provided users already have existing claims or allocations.
///
/// # Returns
///
/// Returns a response indicating the success of the addition operation and includes relevant attributes
/// in the event.
pub fn execute_add_users(
    deps: DepsMut,
    users: Vec<String>,
    amounts: Vec<Uint128>,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current airdrop window is not open (start_height or end_height not zero)
    if current_airdrop_config.start_height != 0 || current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeUserInfo {});
    }

    // Check if the number of users and amounts provided match
    if users.len() != amounts.len() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    // Initialize an empty vector to store attributes for the event
    let mut attributes: Vec<Attribute> = Vec::new();

    // Loop through the provided users and amounts
    for (index, user_and_amount) in users.iter().zip(amounts.iter()).enumerate() {
        // Validate the user's address
        deps.api.addr_validate(user_and_amount.0)?;

        // Validate that the amount is not zero
        validate_amount(*user_and_amount.1, index)?;

        // Attempt to load user_info from storage
        let maybe_user_info = USER_INFO.may_load(deps.storage, user_and_amount.0.clone())?;

        // Check if the user_info exists (is not empty)
        if let Some(user_info) = maybe_user_info {
            // User info exists, perform your checks here
            if user_info.get_claimable_amount() != Uint128::zero() || user_info.get_claimed_flag() {
                // Handle the case where user_info exists
                return Err(AirdropErrors::AlreadyExists {});
            }
        } else {
            // User info does not exist, create a new entry
            let new_user_info = UserInfo {
                claimable_amount: *user_and_amount.1,
                claimed_flag: false,
            };
            USER_INFO.save(deps.storage, user_and_amount.0.clone(), &new_user_info)?;

            // Add user and amount to attributes for the event
            attributes.push(Attribute {
                key: "address".to_string(),
                value: user_and_amount.0.to_string(),
            });
            attributes.push(Attribute {
                key: "amount".to_string(),
                value: user_and_amount.1.to_string(),
            });
        }
    }

    // Check if the total claimable amount exceeds the airdrop amount
    check_amounts_and_airdrop_size(
        get_total_in_user_info(deps.storage),
        current_airdrop_config.airdrop_amount,
    )?;

    // Return a default response if all checks pass with an "airdrop_add_users" event
    Ok(Response::default().add_event(Event::new("airdrop_add_users").add_attributes(attributes)))
}

/// Sets or updates the allocation of claimable amounts for a list of users in the airdrop configuration.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `users` - A vector of user addresses to set or update allocations for.
/// * `amounts` - A vector of amounts to be allocated to each user.
///
/// # Errors
///
/// Returns an error if the airdrop window is not open (start_height or end_height not zero),
/// the number of users and amounts provided do not match, or if any of the provided users have claimed their allocations.
///
/// # Returns
///
/// Returns a response indicating the success of the set/update operation and includes relevant attributes
/// in the event.
pub fn execute_set_users(
    deps: DepsMut,
    users: Vec<String>,
    amounts: Vec<Uint128>,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current airdrop window is not open (start_height or end_height not zero)
    if current_airdrop_config.start_height != 0 || current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeUserInfo {});
    }

    // Check if the number of users and amounts provided match
    if users.len() != amounts.len() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    // Initialize an empty vector to store attributes for the event
    let mut attributes: Vec<Attribute> = Vec::new();

    for (index, user_and_amount) in users.iter().zip(amounts.iter()).enumerate() {
        // Validate the user's address
        deps.api.addr_validate(user_and_amount.0)?;

        // Validate that the amount is not zero
        validate_amount(*user_and_amount.1, index)?;

        // Load the user's current information from storage
        let user_info = USER_INFO.load(deps.storage, user_and_amount.0.clone())?;

        // Check if the user has not claimed
        if !user_info.get_claimed_flag() {
            // Update all the users with the given info
            let new_user_info = UserInfo {
                claimable_amount: *user_and_amount.1,
                claimed_flag: false,
            };
            USER_INFO.save(deps.storage, user_and_amount.0.clone(), &new_user_info)?;

            // Add user and amount to attributes for the event
            attributes.push(Attribute {
                key: "address".to_string(),
                value: user_and_amount.0.to_string(),
            });
            attributes.push(Attribute {
                key: "amount".to_string(),
                value: user_and_amount.1.to_string(),
            })
        }
    }

    // Check if the total claimable amount exceeds the airdrop amount
    check_amounts_and_airdrop_size(
        get_total_in_user_info(deps.storage),
        current_airdrop_config.airdrop_amount,
    )?;

    // Return a default response if all checks pass with an "airdrop_set_users" event
    Ok(Response::default().add_event(Event::new("airdrop_set_users").add_attributes(attributes)))
}

/// Removes specified users from the airdrop configuration if they have not claimed their allocations.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `users` - A vector of user addresses to remove from the airdrop configuration.
///
/// # Errors
///
/// Returns an error if the airdrop window is not open (start_height or end_height not zero).
///
/// # Returns
///
/// Returns a response indicating the success of the removal operation and includes relevant attributes
/// in the event for each removed user.
pub fn execute_remove_users(deps: DepsMut, users: Vec<String>) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current airdrop window is not open (start_height or end_height not zero)
    if current_airdrop_config.start_height != 0 || current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeUserInfo {});
    }

    // Initialize vectors to store removed users and attributes for the event
    let mut removed_users: Vec<String> = Vec::new();
    let mut attributes: Vec<Attribute> = Vec::new();

    // Iterate through the list of users to be removed
    for user in users.iter() {
        // Validate the user's address
        deps.api.addr_validate(user)?;

        // Load the user_info entry from storage
        let user_info = USER_INFO.load(deps.storage, user.to_string())?;

        // Check if the claimed flag is false, indicating that the user has not claimed
        if !user_info.get_claimed_flag() {
            // Add the user's address to the list of removed users
            removed_users.push(user.to_string());

            // Remove the user's entry from the USER_INFO map
            USER_INFO.remove(deps.storage, user.to_string());

            // Add user address as an attribute for the event
            attributes.push(Attribute {
                key: "address".to_string(),
                value: user.to_string(),
            });
        }
    }

    // Return a default response if all checks pass with an "airdrop_remove_users" event
    Ok(
        Response::default()
            .add_event(Event::new("airdrop_remove_users").add_attributes(attributes)),
    )
}

/// Withdraws airdrop funds to the specified address after the airdrop window has ended.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage, external data, and assets.
/// * `env` - Environment information including the current block height.
/// * `withdraw_address` - The address to which the airdrop funds will be withdrawn.
///
/// # Errors
///
/// Returns an error if the current block height is not within the airdrop window or the window is open-ended.
/// Also returns an error if the withdrawal address is invalid.
///
/// # Returns
///
/// Returns a response indicating the success of the withdrawal and includes attributes in the response for tracking.
pub fn execute_withdraw_funds(
    deps: DepsMut,
    env: Env,
    withdraw_address: String,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current block height is within the airdrop window or the window is open-ended
    if env.block.height < current_airdrop_config.end_height
        || current_airdrop_config.end_height == 0
        || current_airdrop_config.start_height == 0
    {
        return Err(AirdropErrors::InvalidWithdraw {});
    }

    // Validate the withdrawal address
    deps.api.addr_validate(&withdraw_address)?;

    // Load the current airdrop configuration again to ensure consistency
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Query the contract's balance of the airdrop asset
    let contract_balance = current_airdrop_config
        .airdrop_asset
        .query_balance(&deps.querier, &env.contract.address)?;

    // Transfer the airdrop asset to the withdrawal address
    let withdraw = Asset::new(current_airdrop_config.airdrop_asset, contract_balance)
        .transfer_msg(&withdraw_address)?;

    // Return a response and add the withdraw transfer message
    Ok(Response::new().add_message(withdraw).add_attributes(vec![
        ("action", "withdraw"),
        ("address", env.contract.address.as_ref()),
        ("amount", &contract_balance.to_string()),
    ]))
}

// Import necessary modules for testing
#[cfg(test)]
mod tests {
    // Import the necessary items for testing
    use super::*;
    use cosmwasm_std::{Addr, Coin};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw_asset::AssetInfo;

    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;

    // Define a helper function to create a mock contract configuration
    fn mock_config_1() -> AirdropConfig {
        AirdropConfig {
            airdrop_description: "Test Airdrop".to_string(),
            airdrop_amount: Uint128::new(1000000), // Adjust this value as needed
            airdrop_asset: AssetInfo::Native("uqsr".parse().unwrap()),
            total_claimed: Uint128::new(0),
            start_height: 12346, // Adjust this value as needed
            end_height: 14567,   // Adjust this value as needed
        }
    }

    // Define a helper function to create a mock contract configuration
    fn mock_config_2() -> AirdropConfig {
        AirdropConfig {
            airdrop_description: "Test Airdrop".to_string(),
            airdrop_amount: Uint128::new(1000000), // Adjust this value as needed
            airdrop_asset: AssetInfo::Native("uqsr".parse().unwrap()),
            total_claimed: Uint128::new(0),
            start_height: 0, // Adjust this value as needed
            end_height: 0,   // Adjust this value as needed
        }
    }

    // Define a test case for updating the airdrop configuration
    #[test]
    fn test_execute_update_airdrop_config() {
        // Create mock dependencies, environment, and config
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("admin", &[Coin::new(100000000, "uqsr")]);
        let config = mock_config_1();

        // Execute the instantiate function to set up the initial state (if needed)
        let instantiate_msg_1 = InstantiateMsg { config: config.clone() };
        instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg_1).unwrap_err();

        // instantiate with a correct config
        let instantiate_msg_2 = InstantiateMsg{config: mock_config_2()};
        instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg_2).unwrap();

        // try to update config with wrong conditions
        execute_update_airdrop_config(deps.as_mut(), env.clone(), config.clone()).unwrap_err();

        // add users to the airdrop
        let users: Vec<String> = vec!["user1".to_string(), "user2".to_string(), "user3".to_string()];
        let amounts: Vec<Uint128> = vec![Uint128::new(330000), Uint128::new(330000), Uint128::new(330000)];
        let add_users_response = execute_add_users(deps.as_mut(), users, amounts).unwrap();
        assert_eq!(add_users_response.events[0].attributes.len(), 6);

        // set a user so that the total is higher than airdrop size
        let users: Vec<String> = vec!["user1".to_string()];
        let amounts: Vec<Uint128> = vec![Uint128::new(630000)];
        execute_set_users(deps.as_mut(), users, amounts).unwrap_err();

        // set a user so that the total is lower than airdrop size
        let users: Vec<String> = vec!["user1".to_string(), "user2".to_string()];
        let amounts: Vec<Uint128> = vec![Uint128::new(230000), Uint128::new(430000)];
        let set_users_response = execute_set_users(deps.as_mut(), users, amounts).unwrap();
        assert_eq!(set_users_response.events[0].attributes.len(), 4);

        // update the airdrop config with
        let new_balance: Vec<Coin> = vec![Coin{denom: "uqsr".to_string(), amount: Uint128::new(1000000)}];
        let address = Addr::unchecked("cosmos2contract");
        deps.querier.update_balance(address, new_balance);
        let execute_response = execute_update_airdrop_config(deps.as_mut(), env.clone(), config.clone()).unwrap();

        // Ensure that the response is successful
        assert_eq!(execute_response.events[0].attributes.len(), 6); // Check for expected attributes

        // Verify that the new configuration is stored
        let stored_config = AIRDROP_CONFIG.load(deps.as_ref().storage);
        assert_eq!(stored_config.unwrap(), config);
    }
}
