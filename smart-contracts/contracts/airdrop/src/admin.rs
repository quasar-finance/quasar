use cosmwasm_std::{DepsMut, Env, Response, StdError, Uint128};
use cw20_base::contract::query_balance;
use std::string::String;

use crate::helpers::get_total_in_user_info;
use crate::state::{AirdropConfig, AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn execute_update_airdrop_config(
    deps: DepsMut,
    env: Env,
    config: AirdropConfig,
) -> Result<Response, AirdropErrors> {
    // checks to see if the current height, end height are not equal to zero,
    // checks if current block height is less than the start height
    // checks if start height is lesser than the end height
    // checks if airdrop amount is more than enough to supply to all the users
    if config.start_height != 0
        && config.end_height != 0
        && env.block.height < config.start_height
        && config.start_height < config.end_height
        && config.airdrop_amount >= get_total_in_user_info(deps.storage)
    {
        let admin_address = deps
            .querier
            .query_wasm_contract_info(&env.contract.address)?
            .admin;
        let contract_bank_balance = query_balance(deps.as_ref(), admin_address.unwrap())
            .unwrap()
            .balance;
        if contract_bank_balance < config.airdrop_amount {
            return Err(AirdropErrors::InvalidChangeInConfig {});
        }
    }

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &config)?;

    // Return a default response to indicate success
    // TODO add events
    Ok(Response::default())
}

pub fn execute_add_users(
    deps: DepsMut,
    users: Vec<String>,
    amounts: Vec<Uint128>,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    if current_airdrop_config.start_height != 0 && current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeInConfig {});
    }

    // Check if the number of users and amounts provided match
    if users.len() != amounts.len() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    // Loop through the provided users and amounts
    for number in 0..=users.len() {
        // Validate the user's address
        deps.api.addr_validate(&users[number].to_string())?;

        // Validate that the amount is not negative
        if amounts[number] < Uint128::zero() {
            return Err(AirdropErrors::Std(StdError::GenericErr {
                msg: "Amount at index :".to_string()
                    + &*number.to_string()
                    + &*"is negative".to_string(),
            }));
        }

        // update all the users with the given info
        let user_info = USER_INFO.load(deps.storage, users[number].clone())?;
        if user_info.get_claimable_amount() == Uint128::zero() && !user_info.get_claimed_flag() {
            USER_INFO.save(deps.storage, users[number].clone(), &user_info)?;
        }
    }

    // Calculate the total claimable amount from USER_INFO
    let total_in_user_info = get_total_in_user_info(deps.storage);

    // Check if the total claimable amount exceeds the airdrop amount
    if total_in_user_info > current_airdrop_config.airdrop_amount {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Total amount in the given user amounts".to_string()
                + &*total_in_user_info.to_string()
                + &*" is greater than ".to_string()
                + &*current_airdrop_config.airdrop_amount.to_string(),
        }));
    }

    // Return a default response if all checks pass
    // TODO add events
    Ok(Response::default())
}

pub fn execute_set_users(
    deps: DepsMut,
    users: Vec<String>,
    amounts: Vec<Uint128>,
) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    if current_airdrop_config.start_height != 0 && current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeInConfig {});
    }

    // Check if the number of users and amounts provided match
    if users.len() != amounts.len() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
        }));
    }

    for number in 0..=users.len() {
        // Validate the user's address
        deps.api.addr_validate(&users[number].to_string())?;

        // Validate that the amount is not negative
        if amounts[number] < Uint128::zero() {
            return Err(AirdropErrors::Std(StdError::GenericErr {
                msg: "Amount at index :".to_string()
                    + &*number.to_string()
                    + &*"is negative".to_string(),
            }));
        }

        // update all the users with the given info
        let user_info = USER_INFO.load(deps.storage, users[number].clone())?;
        if !user_info.get_claimed_flag() {
            USER_INFO.save(deps.storage, users[number].clone(), &user_info)?;
        }
    }

    // Calculate the total claimable amount from USER_INFO
    let total_in_user_info = get_total_in_user_info(deps.storage);

    // Check if the total claimable amount exceeds the airdrop amount
    if total_in_user_info > current_airdrop_config.airdrop_amount {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Total amount in the given user amounts".to_string()
                + &*total_in_user_info.to_string()
                + &*" is greater than ".to_string()
                + &*current_airdrop_config.airdrop_amount.to_string(),
        }));
    }

    // Return a default response if all checks pass
    // TODO add events
    Ok(Response::default())
}

pub fn execute_remove_users(deps: DepsMut, users: Vec<String>) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the current block height is greater than or equal to the start height in the current config.
    // If true, it's not allowed to update the configuration, return an error.
    if current_airdrop_config.start_height != 0 && current_airdrop_config.end_height != 0 {
        return Err(AirdropErrors::InvalidChangeInConfig {});
    }

    // Iterate through the list of users to be removed
    for user in users.iter() {
        // Validate the user's address
        deps.api.addr_validate(&user)?;

        // Load the user_info entry from storage
        let user_info = USER_INFO.load(deps.storage, user.clone())?;

        // Check if the claimed flag is false, indicating that the user has not claimed
        if !user_info.get_claimed_flag() {
            // Remove the user's entry from the USER_INFO map
            USER_INFO.remove(deps.storage, user.clone())?;
        }
    }

    // Return a default response if all checks pass
    // TODO add events
    Ok(Response::default())
}
