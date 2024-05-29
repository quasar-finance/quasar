#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::admin::execute::handle_execute_admin;
use crate::admin::query::handle_query_admin;
use crate::error::ContractError;
use crate::incentives::execute::handle_execute_incentives;
use crate::incentives::query::handle_query_incentives;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{CONFIG, INCENTIVES_ADMIN};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:merkle-incentives";
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

    CONFIG.save(deps.storage, &msg.config)?;

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
            handle_execute_incentives(deps, env, incentives_msg)
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
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}

fn migrate_contract(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let (version_previous, version_new) = get_versions(deps.storage, msg)?;

    if version_new >= version_previous {
        set_contract_version(deps.storage, CONTRACT_NAME, version_new.to_string())?;
    }

    Ok(Response::new().add_attribute("action", "migrate"))
}

fn get_versions(
    storage: &dyn Storage,
    msg: MigrateMsg,
) -> Result<(Version, Version), ContractError> {
    let version_previous: Version = get_contract_version(storage)?
        .version
        .parse()
        .map_err(|_| ContractError::ParsingPrevVersion)?;

    let version_new: Version = env!("CARGO_PKG_VERSION")
        .parse()
        .map_err(|_| ContractError::ParsingNewVersion)?;

    if version_new.to_string() != msg.version {
        Err(ContractError::ImproperMsgVersion)?;
    }

    Ok((version_previous, version_new))
}

#[cfg(test)]
mod tests {}
