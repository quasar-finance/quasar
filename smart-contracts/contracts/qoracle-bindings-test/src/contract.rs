#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp,
    Uint128,
};
use cw2::set_contract_version;
use quasar_bindings::querier::{self, QuasarQuerier};
use quasar_bindings::query::QuasarQuery;

use crate::error::ContractError;
use crate::execute::{demo_fetch_oracle_prices, demo_fetch_pool_info, demo_fetch_pools};
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
        ExecuteMsg::DemoOsmosisPools {} => demo_fetch_pools(deps),
        ExecuteMsg::DemoOsmosisPoolInfo {} => demo_fetch_pool_info(deps),
        ExecuteMsg::DemoOraclePrices {} => demo_fetch_oracle_prices(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {}
