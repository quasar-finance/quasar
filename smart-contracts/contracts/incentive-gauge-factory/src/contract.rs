#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    executes, migrate,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    queries,
};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:incentive-gauge-factory";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateIncentiveGauge { kind: _, gauge: _ } => {
            executes::create_incentive_gauge()
        }
        ExecuteMsg::ClaimGaugeFees { address } => executes::claim_fees(deps, env, address),
        ExecuteMsg::SetGaugeCodes { codes } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Gauge { address } => to_json_binary(&queries::query_gauge(
            deps,
            deps.api.addr_validate(&address)?,
        )?),
        QueryMsg::ListGauges { start_after, limit } => {
            to_json_binary(&queries::query_gauge_list(deps, start_after, limit)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate::migrate_contract(deps, env, msg)
}

#[cfg(test)]
mod tests {}
