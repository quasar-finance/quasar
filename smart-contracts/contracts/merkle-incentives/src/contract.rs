#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::admin::execute::handle_execute_admin;
use crate::admin::query::handle_query_admin;
use crate::error::ContractError;
use crate::incentives::execute::handle_execute_incentives;
use crate::incentives::query::handle_query_incentives;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::INCENTIVES_ADMIN;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:merkle-incentives";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    INCENTIVES_ADMIN.save(
        deps.storage,
        &deps.api.addr_validate(&msg.incentives_admin)?,
    )?;

    Ok(Response::default().add_attribute("incentive_admin", msg.incentives_admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncentivesMsg(incentives_msg) => {
            handle_execute_incentives(deps, incentives_msg)
        }
        ExecuteMsg::AdminMsg(admin_msg) => handle_execute_admin(deps, env, info, admin_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IncentivesQuery(range_query) => handle_query_incentives(deps, env, range_query),
        QueryMsg::AdminQuery(admin_query) => handle_query_admin(deps, env, admin_query),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("migrate", "successful"))
}

#[cfg(test)]
mod tests {}
