#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
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

    let pool_response = querier.osmosis_pool("2".to_string())?;
    // let all_pool_response = querier.all_osmosis_pools(Option::None)?;
    // let pool_ranking = querier.osmosis_pool_ranking()?;
    // let pool_info = querier.osmosis_pool_info("2".to_string())?;
    // let pool_info_all = querier.all_osmosis_pool_info(Option::None)?;
    // let oracle_prices = querier.oracle_prices()?;

    Ok(Response::new()
        .add_attribute("pool_creator", pool_response.poolPosition.creator)
        .add_attribute("pool_metrics", pool_response.poolPosition.metrics.tVL)
        // .add_attribute(
        //     "num_pool_positions",
        //     all_pool_response.poolPositions.len().to_string(),
        // )
        // .add_attribute(
        //     "pool_ranking",
        //     pool_ranking.poolRanking.poolIdsSortedByAPY.first().unwrap(),
        // )
        // .add_attribute("pool_ranking", Uint128::from(pool_info.poolInfo.info.id))
        // .add_attribute("num_pool_info", pool_info_all.poolInfo.len().to_string())
        // .add_attribute(
        //     "oracle_prices",
        //     oracle_prices.prices.first().unwrap().denom.clone(),
        // )
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {}
