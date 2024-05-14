use crate::error::VaultRewardsError;
use crate::execute::admin::{
    execute_add_distribution_schedule, execute_auto_claim, execute_remove_distribution_schedule,
    execute_update_distribution_schedule, execute_withdraw_funds,
};
use crate::execute::user::execute_claim;
use crate::execute::vault::execute_update_user_reward_index;
use crate::helpers::is_contract_admin;
use crate::msg::{
    AdminExecuteMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, VaultExecuteMsg,
};
use crate::query::{
    query_all_users, query_config, query_pending_rewards, query_user_rewards_index,
};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, Uint128};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, VaultRewardsError> {
    let mut config = Config {
        vault_token: deps.api.addr_validate(&msg.vault_token)?,
        reward_token: msg.reward_token,
        distribution_schedules: vec![],
        total_claimed: Uint128::zero(),
    };
    config.add_distribution_schedules(&deps.querier, &env, msg.distribution_schedules)?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, VaultRewardsError> {
    match msg {
        ExecuteMsg::Claim {} => execute_claim(deps, &env, info.sender),
        ExecuteMsg::Admin(admin_msg) => {
            is_contract_admin(&deps.querier, &env, &info.sender)?;
            match admin_msg {
                AdminExecuteMsg::WithdrawFunds(asset) => {
                    execute_withdraw_funds(deps, env, info.sender, asset)
                }
                AdminExecuteMsg::AddDistributionSchedule(schedule) => {
                    execute_add_distribution_schedule(deps, env, schedule)
                }
                AdminExecuteMsg::UpdateDistributionSchedule { id, update } => {
                    execute_update_distribution_schedule(deps, env, id, update)
                }
                AdminExecuteMsg::RemoveDistributionSchedule(id) => {
                    execute_remove_distribution_schedule(deps, env, id)
                }
                AdminExecuteMsg::AutoClaim { user_addresses } => {
                    execute_auto_claim(deps, env, user_addresses)
                }
            }
        }
        ExecuteMsg::Vault(vault_msg) => {
            let vault_token = CONFIG.load(deps.storage)?.vault_token;
            if info.sender != vault_token {
                return Err(VaultRewardsError::Unauthorized {});
            }
            match vault_msg {
                VaultExecuteMsg::UpdateUserRewardIndex(user) => {
                    let user = deps.api.addr_validate(&user)?;
                    execute_update_user_reward_index(deps, env, user)
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, VaultRewardsError> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps, env)?),
        QueryMsg::PendingRewards(user) => {
            let user = deps.api.addr_validate(&user)?;
            to_json_binary(&query_pending_rewards(deps, env, user)?)
        }
        QueryMsg::GetUserRewardsIndex(user) => {
            let user = deps.api.addr_validate(&user)?;
            to_json_binary(&query_user_rewards_index(deps, user)?)
        }
        QueryMsg::AllUsers { start_after, limit } => to_json_binary(&query_all_users(
            deps,
            Option::from(start_after as usize),
            Option::from(limit as usize),
        )?),
    }
    .map_err(|e| e.into())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, VaultRewardsError> {
    Ok(Response::new().add_attribute("success", "true"))
}
