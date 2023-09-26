use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::admin::{
    execute_add_users, execute_remove_users, execute_set_users, execute_update_airdrop_config,
    execute_withdraw_funds,
};
use crate::error::AirdropErrors;
use crate::helpers::is_contract_admin;
use crate::msg::{AdminExecuteMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_config, query_contract_state, query_sanity_check, query_user};
use crate::state::AIRDROP_CONFIG;
use crate::users::execute_claim;

// version info for migration info
const CONTRACT_NAME: &str = "quasar_airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, AirdropErrors> {
    // Set the contract version in storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do not instantiate the contract if start height or end height is not set to zero
    if msg.config.start_height != 0 && msg.config.end_height != 0 {
        return Err(AirdropErrors::InvalidAirdropWindow {});
    }

    // Save the new airdrop configuration to storage
    AIRDROP_CONFIG.save(deps.storage, &msg.config)?;

    // Return a default response to indicate success
    Ok(Response::default())
}

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AirdropConfigQuery {} => to_binary(&query_config(deps)?),
        QueryMsg::UserInfoQuery { user } => to_binary(&query_user(deps, user)?),
        QueryMsg::ContractStateQuery {} => to_binary(&query_contract_state(deps)?),
        QueryMsg::SanityCheckQuery {} => to_binary(&query_sanity_check(deps, env)?),
    }
}
