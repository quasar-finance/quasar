use std::string::String;

use crate::helpers::get_total_in_user_info;
use cosmwasm_std::{Deps, Env, Order, StdResult};

use crate::msg::{ConfigResponse, ContractStateResponse, SanityCheckResponse, UserInfoResponse};
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
