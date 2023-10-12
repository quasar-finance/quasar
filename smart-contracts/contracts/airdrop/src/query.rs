use std::string::String;

use crate::helpers::get_total_in_user_info;
use cosmwasm_std::{Deps, Env, Order, StdResult};

use crate::msg::{
    ConfigResponse, ContractStateResponse, SanityCheckResponse, UserInfoResponse,
    UsersStatsResponse,
};
use crate::state::{UserInfo, AIRDROP_CONFIG, USER_INFO};

/// Queries and returns the current airdrop configuration.
///
/// # Arguments
///
/// * `deps` - Deps is a struct providing access to the contract's dependencies like storage.
///
/// # Returns
///
/// Returns a `ConfigResponse` containing the current airdrop configuration.
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = AIRDROP_CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        airdrop_config: config,
    })
}

/// Queries and returns the information of a specific user.
///
/// # Arguments
///
/// * `deps` - Deps is a struct providing access to the contract's dependencies like storage.
/// * `user` - The address of the user for which to retrieve information.
///
/// # Returns
///
/// Returns a `UserInfoResponse` containing the user's information.
pub fn query_user(deps: Deps, user: String) -> StdResult<UserInfoResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_INFO.load(deps.storage, user_addr.to_string())?;
    Ok(UserInfoResponse { user_info })
}

/// Queries and returns the entire contract state, including airdrop configuration and user information.
///
/// # Arguments
///
/// * `deps` - Deps is a struct providing access to the contract's dependencies like storage.
///
/// # Returns
///
/// Returns a `ContractStateResponse` containing the airdrop configuration and user information.
pub fn query_contract_state(deps: Deps) -> StdResult<ContractStateResponse> {
    let config = AIRDROP_CONFIG.load(deps.storage)?;
    let mut user_infos: Vec<(String, UserInfo)> = Vec::new();
    for res in USER_INFO.range(deps.storage, None, None, Order::Ascending) {
        let unwrapped_res = res.unwrap();
        user_infos.push((unwrapped_res.0, unwrapped_res.1))
    }
    Ok(ContractStateResponse {
        airdrop_config: config,
        user_info: user_infos,
    })
}

/// Performs a sanity check to verify if there are sufficient funds for airdrop payments.
///
/// # Arguments
///
/// * `deps` - Deps is a struct providing access to the contract's dependencies like storage and querier.
/// * `env` - Environment information.
///
/// # Returns
///
/// Returns a `SanityCheckResponse` indicating whether there are sufficient funds for airdrop payments.
pub fn query_sanity_check(deps: Deps, env: Env) -> StdResult<SanityCheckResponse> {
    // Check if the airdrop amount is sufficient to supply all users
    let airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;
    if airdrop_config.airdrop_amount >= get_total_in_user_info(deps.storage) {
        // Get the contract's bank balance
        let contract_balance = airdrop_config
            .airdrop_asset
            .query_balance(&deps.querier, env.contract.address)
            .unwrap();

        // Check if the contract has enough funds for the airdrop
        if contract_balance < airdrop_config.airdrop_amount {
            return Ok(SanityCheckResponse { response: false });
        }
    } else {
        return Ok(SanityCheckResponse { response: false });
    }
    Ok(SanityCheckResponse { response: true })
}

/// Query user statistics including the number of claimed and unclaimed users.
///
/// # Arguments
///
/// - `deps`: A reference to the dependencies needed for the query.
///
/// # Returns
///
/// A result containing a `UsersStatsResponse`, which includes the counts of
/// claimed and unclaimed users, and the total number of users.
///
/// # Errors
///
/// This function returns a `StdResult` that can hold a `CosmosSDK` error.
///
pub fn query_users_stats(deps: Deps) -> StdResult<UsersStatsResponse> {
    // Initialize counters for claimed and total users.
    let mut claimed_users_count = 0u64;
    let mut total_users_count = 0u64;

    // Iterate through user info and count claimed and total users.
    for res in USER_INFO.range(deps.storage, None, None, Order::Ascending) {
        let claimed = res.as_ref().unwrap().1.get_claimed_flag();
        if claimed {
            claimed_users_count += 1;
        }
        total_users_count += 1;
    }

    // Create and return the UsersStatsResponse.
    Ok(UsersStatsResponse {
        claimed_users_count,
        unclaimed_users_count: total_users_count - claimed_users_count,
        total_users_count,
    })
}
