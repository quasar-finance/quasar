#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use quasar_bindings::querier::{self, QuasarQuerier};
use quasar_bindings::query::{OsmosisPoolPositionResponse, QuasarQuery};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, ACKTRIGGERED, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:intergamm-bindings-test-2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<QuasarQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RunQOracleTest {} => run_qoracle_test(deps),
    }
}

pub fn run_qoracle_test(deps: DepsMut<QuasarQuery>) -> Result<Response, ContractError> {
    let querier = QuasarQuerier::new(&deps.querier);

    let pool_response: OsmosisPoolPositionResponse = querier.osmosis_pool("2".to_string())?;

    Ok(Response::new()
        .add_attribute("pool_creator", pool_response.poolPosition.creator)
        .add_attribute("pool_metrics", pool_response.poolPosition.metrics.tVL))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {}
