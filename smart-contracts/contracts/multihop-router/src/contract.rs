#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::helpers::is_contract_admin;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Destination, Hop, ROUTES};

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
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddRoute { destination, hops } => {
            execute_add_route(deps, env, info, &destination.into(), hops)
        }
        ExecuteMsg::MutateRoute { destination, hops } => {
            execute_mutate_route(deps, env, info, &destination.into(), hops)
        }
        ExecuteMsg::RemoveRoute { destination } => {
            execute_remove_route(deps, env, info, &destination.into())
        }
    }
}

pub fn execute_add_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: &Destination,
    hop: Hop,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    if ROUTES.has(deps.storage, destination) {
        return Err(ContractError::DestinationAlreadyExists);
    }

    ROUTES.save(deps.storage, destination, &hop)?;

    Ok(Response::new()
        .add_attribute("action", "add_route")
        .add_attribute("destination", format!("{}", destination))
        .add_attribute("hops", format!("{:?}", hop)))
}

pub fn execute_mutate_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: &Destination,
    hop: Hop,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    ROUTES.save(deps.storage, destination, &hop)?;

    Ok(Response::new()
        .add_attribute("action", "mutate_route")
        .add_attribute("destination", format!("{}", destination))
        .add_attribute("hops", format!("{:?}", hop)))
}

pub fn execute_remove_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: &Destination,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    if !ROUTES.has(deps.storage, destination) {
        return Err(ContractError::DestinationNotExists);
    }

    ROUTES.remove(deps.storage, destination);

    Ok(Response::new()
        .add_attribute("action", "remove_route")
        .add_attribute("destination", format!("{}", destination)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
