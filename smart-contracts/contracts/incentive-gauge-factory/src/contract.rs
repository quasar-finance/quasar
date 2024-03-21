#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::helpers::is_contract_admin;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multihop-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateIncentiveGauge { r#type, gauge } => todo!(),
        ExecuteMsg::ClaimGaugeFees { gauge_address } => todo!(),
    }
}

pub fn execute_create_incentive_gauge() -> Result<Response, ContractError> {
    // instantiate an instance of the incentive gauge

    // save the instance in the Gauge overview
    
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {}
