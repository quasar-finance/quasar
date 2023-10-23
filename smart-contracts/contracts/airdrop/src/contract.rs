use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw20_base::msg::MigrateMsg;

use crate::admin::{
    execute_add_users, execute_remove_users, execute_set_users, execute_update_airdrop_config,
    execute_withdraw_funds,
};
use crate::error::AirdropErrors;
use crate::helpers::is_contract_admin;
use crate::msg::{AdminExecuteMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{
    query_airdrop_readiness, query_config, query_contract_state, query_user, query_users_stats,
};
use crate::state::AIRDROP_CONFIG;
use crate::users::execute_claim;

// version info for migration info
const CONTRACT_NAME: &str = "quasar_airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Instantiates the airdrop contract with the provided configuration.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `_env` - Environment information, not used in this function.
/// * `_info` - Message sender's information, not used in this function.
/// * `msg` - Instantiate message containing the airdrop configuration.
///
/// # Errors
///
/// Returns an error if the airdrop configuration is invalid, specifically if start height,
/// end height, and total claimed are not set to zero.
///
/// # Returns
///
/// Returns a response indicating the success of contract instantiation and includes attributes
/// describing the airdrop configuration.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, AirdropErrors> {
    // Set the contract version in storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Check if the contract should not be instantiated due to an invalid airdrop window
    if msg.config.start_height != 0
        || msg.config.end_height != 0
        || msg.config.total_claimed != Uint128::zero()
    {
        return Err(AirdropErrors::InvalidAirdropWindow {});
    }

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &msg.config)?;

    // Return a response indicating successful contract instantiation with attributes
    Ok(Response::new().add_event(
        Event::new("instantiate_airdrop_contract")
            .add_attribute(
                "description".to_string(),
                msg.config.airdrop_description.to_string(),
            )
            .add_attribute(
                "airdrop_amount".to_string(),
                msg.config.airdrop_amount.to_string(),
            )
            .add_attribute(
                "airdrop_asset".to_string(),
                msg.config.airdrop_asset.to_string(),
            )
            .add_attribute("claimed".to_string(), msg.config.total_claimed.to_string())
            .add_attribute(
                "start_height".to_string(),
                msg.config.start_height.to_string(),
            )
            .add_attribute("end_height".to_string(), msg.config.end_height.to_string()),
    ))
}

/// Executes contract operations based on the received message.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `env` - Environment information.
/// * `info` - Message sender's information.
/// * `msg` - Execute message to determine the operation.
///
/// # Returns
///
/// Returns a response based on the executed operation or an error if the operation fails.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, AirdropErrors> {
    match msg {
        ExecuteMsg::Admin(admin_msg) => {
            // Check if the sender is a contract admin
            is_contract_admin(&deps.querier, &env, &info.sender)?;

            match admin_msg {
                AdminExecuteMsg::UpdateAirdropConfig(airdrop_config) => {
                    // Call the function to update the airdrop configuration
                    execute_update_airdrop_config(deps, env, airdrop_config)
                }
                AdminExecuteMsg::AddUsers { users } => {
                    // Call the function to add users and their amounts
                    execute_add_users(deps, users)
                }
                AdminExecuteMsg::RemoveUsers(users) => execute_remove_users(deps, users),
                AdminExecuteMsg::WithdrawFunds(withdraw_address) => {
                    execute_withdraw_funds(deps, env, withdraw_address)
                }
                AdminExecuteMsg::SetUsers { users } => execute_set_users(deps, users),
            }
        }
        ExecuteMsg::ClaimAirdrop() => execute_claim(deps, env, info.sender),
    }
}

/// Queries contract state based on the received query message.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `env` - Environment information.
/// * `msg` - Query message to determine the requested information.
///
/// # Returns
///
/// Returns a binary response containing the queried information or an error if the query fails.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AirdropConfigQuery {} => to_binary(&query_config(deps)?),
        QueryMsg::UserInfoQuery { user } => to_binary(&query_user(deps, user)?),
        QueryMsg::ContractStateQuery {} => to_binary(&query_contract_state(deps)?),
        QueryMsg::SanityCheckQuery {} => to_binary(&query_airdrop_readiness(deps, env)?),
        QueryMsg::UsersStatsQuery {} => to_binary(&query_users_stats(deps)?),
    }
}

/// Migrates the contract to a new version (not implemented).
///
/// # Arguments
///
/// * `_deps` - Dependencies to access storage and external data, not used in this function.
/// * `_env` - Environment information, not used in this function.
/// * `_msg` - Migrate message, not used in this function.
///
/// # Returns
///
/// Returns a response indicating a successful migration (not implemented).
#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, AirdropErrors> {
    Ok(Response::new().add_attribute("migrate", "successful"))
}
