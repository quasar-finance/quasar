use cosmwasm_std::{DepsMut, Env, Response, StdError, Uint128};
use schemars::schema::InstanceType::String;
use std::backtrace::Backtrace;

use crate::helpers::get_total_in_user_info;
use crate::state::{AirdropConfig, UserInfo, AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn execute_update_airdrop_config(
    deps: DepsMut,
    env: Env,
    config: AirdropConfig,
) -> Result<Response, AirdropErrors> {
    /// Load the current airdrop configuration from storage
    let mut current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    /// Get the start and end heights from the current config
    let heights = current_airdrop_config.get_start_and_end_heights();

    /// Check if the current block height is greater than or equal to the start height in the current config.
    /// If true, it's not allowed to update the configuration, return an error.
    if env.block.height >= heights.0 {
        return Err(AirdropErrors::InvalidChangeInConfig {});
    }

    /// Check if the provided end height is less than or equal to the start height.
    if config.end_height <= config.start_height {
        return Err(AirdropErrors::InvalidAirdropWindow {});
    }

    /// Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &config)?;

    /// Return a default response to indicate success
    Ok(Response::default())
}

pub fn execute_add_users(
    deps: DepsMut,
    _env: Env,
    users: Vec<String>,
    amounts: Vec<Uint128>,
) -> Result<Response, AirdropErrors> {
    /// Capture a backtrace for error reporting
    let backtrace = Backtrace::capture();

    /// Check if the number of users and amounts provided match
    if users.len() != amounts.len() {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Deposit amount weight for primitive is zero".to_string(),
            backtrace,
        }));
    }

    /// Loop through the provided users and amounts
    for number in 0..=users.len() {
        /// Validate the user's address
        deps.api.addr_validate(&users[number].to_string())?;

        /// Validate that the amount is not negative
        if amounts[number] < Uint128::zero() {
            return Err(AirdropErrors::Std(StdError::GenericErr {
                msg: "Amount at index :" + number.to_string() + "is negative",
                backtrace,
            }));
        }

        /// update all the users with the give info
        let mut user_info = USER_INFO.load(deps.storage, users[number])?;
        user_info.push(UserInfo {
            claimable_amount: amounts[number],
            claimed_flag: false,
        });
        USER_INFO.save(deps.storage, users[number], &user_info)?;
    }

    /// Calculate the total claimable amount from USER_INFO
    let total_in_user_info = get_total_in_user_info(deps);

    /// Load the current airdrop configuration
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    /// Check if the total claimable amount exceeds the airdrop amount
    if total_in_user_info > current_airdrop_config.airdrop_amount {
        return Err(AirdropErrors::Std(StdError::GenericErr {
            msg: "Total amount in the given user amounts"
                + total_in_user_info.to_string()
                + " is greater than "
                + current_airdrop_config.airdrop_amount.to_string(),
            backtrace,
        }));
    }

    /// Return a default response if all checks pass
    Ok(Response::default())
}
