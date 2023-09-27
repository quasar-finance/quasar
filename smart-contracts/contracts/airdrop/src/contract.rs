use cosmwasm_std::{
    entry_point, to_binary, Attribute, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply,
    Response, StdResult, Uint128,
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
use crate::query::{query_config, query_contract_state, query_sanity_check, query_user};
use crate::reply::handle_reply;
use crate::state::{AIRDROP_CONFIG, REPLY_MAP};
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

    // Create attributes for tracking the airdrop configuration
    let attributes: Vec<Attribute> = vec![
        Attribute {
            key: "description".to_string(),
            value: msg.config.airdrop_description.to_string(),
        },
        Attribute {
            key: "airdrop_amount".to_string(),
            value: msg.config.airdrop_amount.to_string(),
        },
        Attribute {
            key: "airdrop_asset".to_string(),
            value: msg.config.airdrop_asset.to_string(),
        },
        Attribute {
            key: "claimed".to_string(),
            value: msg.config.total_claimed.to_string(),
        },
        Attribute {
            key: "start_height".to_string(),
            value: msg.config.start_height.to_string(),
        },
        Attribute {
            key: "end_height".to_string(),
            value: msg.config.end_height.to_string(),
        },
    ];

    // Return a response indicating successful contract instantiation with attributes
    Ok(Response::default()
        .add_event(Event::new("instantiate_airdrop_contract").add_attributes(attributes)))
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
                AdminExecuteMsg::AddUsers { users, amounts } => {
                    // Call the function to add users and their amounts
                    execute_add_users(deps, users, amounts)
                }
                AdminExecuteMsg::RemoveUsers(users) => execute_remove_users(deps, users),
                AdminExecuteMsg::WithdrawFunds(withdraw_address) => {
                    execute_withdraw_funds(deps, env, withdraw_address)
                }
                AdminExecuteMsg::SetUsers { users, amounts } => {
                    execute_set_users(deps, users, amounts)
                }
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
        QueryMsg::SanityCheckQuery {} => to_binary(&query_sanity_check(deps, env)?),
    }
}

/// Handles replies from external contracts.
///
/// # Arguments
///
/// * `deps` - Dependencies to access storage and external data.
/// * `_env` - Environment information, not used in this function.
/// * `msg` - Reply message containing the reply ID.
///
/// # Returns
///
/// Returns a response based on the received reply or an error if the reply handling fails.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, AirdropErrors> {
    let address = REPLY_MAP.load(deps.storage, msg.id)?;
    handle_reply(deps, address)
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
