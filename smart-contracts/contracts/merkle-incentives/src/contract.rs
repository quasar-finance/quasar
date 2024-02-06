#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::admin::execute::execute_admin_msg;
use crate::admin::query::query_admin;
use crate::error::ContractError;
use crate::incentives::execute::execute_incentives_msg;
use crate::incentives::query::query_incentives;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::INCENTIVES_ADMIN;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:range-middleware";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    INCENTIVES_ADMIN.save(deps.storage, &info.sender)?;

    Ok(Response::default().add_attribute("incentive_admin", info.sender))
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
            execute_incentives_msg(deps, env, info, incentives_msg)
        }
        ExecuteMsg::AdminMsg(admin_msg) => execute_admin_msg(deps, env, info, admin_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IncentivesQuery(range_query) => query_incentives(deps, env, range_query),
        QueryMsg::AdminQuery(admin_query) => query_admin(deps, env, admin_query),
    }
}

#[cfg(test)]
mod tests {}
