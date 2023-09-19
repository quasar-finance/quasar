use std::string::String;

use cosmwasm_std::{DepsMut, Env, Response, StdError, Uint128};
use cw20_base::contract::query_balance;
use cw_asset::Asset;

use crate::helpers::get_total_in_user_info;
use crate::state::{AirdropConfig, AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn execute_update_airdrop_config(
    deps: DepsMut,
    env: Env,
    config: AirdropConfig,
) -> Result<Response, AirdropErrors> {
    // Check various conditions to validate the airdrop configuration update

    // Check if the start height and end height are not zero,
    // indicating a valid airdrop window
    if config.start_height != 0 && config.end_height != 0 {
        // Check if the current block height is less than the start height
        // and if the start height is less than the end height
        if env.block.height < config.start_height && config.start_height < config.end_height {
            // Check if the airdrop amount is sufficient to supply all users
            if config.airdrop_amount >= get_total_in_user_info(deps.storage) {
                // Get the admin address of the contract
                let admin_address = deps
                    .querier
                    .query_wasm_contract_info(&env.contract.address)?
                    .admin;

                // Get the contract's bank balance
                let contract_bank_balance = query_balance(deps.as_ref(), admin_address.unwrap())
                    .unwrap()
                    .balance;

                // Check if the contract has enough funds for the airdrop
                if contract_bank_balance < config.airdrop_amount {
                    return Err(AirdropErrors::InvalidChangeInConfig {});
                }
            }
        }
    }

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

        // Load the user's current information from storage
        let user_info = USER_INFO.load(deps.storage, users[number].clone())?;

        // Check if the user has not claimed and has no existing claimable amount
        if user_info.get_claimable_amount() == Uint128::zero() && !user_info.get_claimed_flag() {
            // Save the user's information with the given info
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

        // Load the user's current information from storage
        let user_info = USER_INFO.load(deps.storage, users[number].clone())?;

        // Check if the user has not claimed
        if !user_info.get_claimed_flag() {
            // Update all the users with the given info
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

    // Iterate through the list of users to be removed
    for user in users.iter() {
        // Validate the user's address
        deps.api.addr_validate(&user)?;

        // Load the user_info entry from storage
        let user_info = USER_INFO.load(deps.storage, user.clone())?;

        // Check if the claimed flag is false, indicating that the user has not claimed
        if !user_info.get_claimed_flag() {
            // Remove the user's entry from the USER_INFO map
            USER_INFO.remove(deps.storage, user.clone());
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

    // Get the admin address of the contract
    let admin_address = deps
        .querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;

    // Get the contract's bank balance
    let contract_bank_balance = query_balance(deps.as_ref(), admin_address.unwrap())
        .unwrap()
        .balance;

    // Transfer the airdrop asset to the withdrawal address
    // TODO: Store this transaction as an event
    Asset::new(
        current_airdrop_config.airdrop_asset.clone(),
        contract_bank_balance,
    )
    .transfer_msg(&withdraw_address)?;

    // Return a default response if all checks pass
    // TODO: Add events
    Ok(Response::default())
}
