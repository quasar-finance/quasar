use std::string::String;

use cosmwasm_std::{DepsMut, Env, Response, StdError, Uint128};
use cw_asset::Asset;

use crate::helpers::{
    check_amounts_and_airdrop_size, get_total_in_user_info, validate_amount, validate_update_config,
};
use crate::state::{AirdropConfig, UserInfo, AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn execute_update_airdrop_config(
    deps: DepsMut,
    env: Env,
    config: AirdropConfig,
) -> Result<Response, AirdropErrors> {
    // load current airdrop config
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // check if an airdrop has been executed on the contract, if yes then return an error
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

    // Return a default response to indicate success
    // TODO: Add events
    Ok(Response::default())
}

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

    // Loop through the provided users and amounts
    for (index, user_and_amount) in users.iter().zip(amounts.iter()).enumerate() {
        // Validate the user's address
        deps.api.addr_validate(user_and_amount.0)?;

        // Validate that the amount is not zero
        if user_and_amount.1 != Uint128::zero() {
            return Err(AirdropErrors::Std(StdError::GenericErr {
                msg: "Amount at index :".to_string()
                    + &*index.to_string()
                    + &*"is zero".to_string(),
            }));
        }

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
        }
    }

    // Check if the total claimable amount exceeds the airdrop amount
    check_amounts_and_airdrop_size(
        get_total_in_user_info(deps.storage),
        current_airdrop_config.airdrop_amount,
    )?;

    // Return a default response if all checks pass
    // TODO: Add events
    Ok(Response::default())
}

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

    for (index, user_and_amount) in users.iter().zip(amounts.iter()).enumerate() {
        // Validate the user's address
        deps.api.addr_validate(user_and_amount.0)?;

        // Validate that the amount is not zero
        validate_amount(user_and_amount.1, index)?;

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
        }
    }

    // Check if the total claimable amount exceeds the airdrop amount
    check_amounts_and_airdrop_size(
        get_total_in_user_info(deps.storage),
        current_airdrop_config.airdrop_amount,
    )?;

    // Return a default response if all checks pass
    // TODO: Add events
    Ok(Response::default())
}

pub fn execute_remove_users(deps: DepsMut, users: Vec<String>) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current airdrop window is not open (start_height or end_height not zero)
    if current_airdrop_config.start_height != 0 || current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeUserInfo {});
    }

    let mut removed_users: Vec<String> = Vec::new();
    // Iterate through the list of users to be removed
    for user in users.iter() {
        // Validate the user's address
        deps.api.addr_validate(user)?;

        // Load the user_info entry from storage
        let user_info = USER_INFO.load(deps.storage, user.to_string())?;

        // Check if the claimed flag is false, indicating that the user has not claimed
        if !user_info.get_claimed_flag() {
            removed_users.push(user.to_string());
            // Remove the user's entry from the USER_INFO map
            USER_INFO.remove(deps.storage, user.to_string());
        }
    }

    // Return a default response if all checks pass
    // TODO: Add events
    Ok(Response::default())
}

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

    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;
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
