#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::helpers::is_contract_admin;
use crate::msg::{
    ExecuteMsg, GetMemoResponse, GetRouteResponse, InstantiateMsg, ListRoutesResponse,
    MemoResponse, QueryMsg,
};
use crate::route::{Route, RouteId};
use crate::state::ROUTES;

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
        ExecuteMsg::AddRoute { route_id, route } => {
            execute_add_route(deps, env, info, &route_id, route)
        }
        ExecuteMsg::MutateRoute {
            route_id,
            new_route,
        } => execute_mutate_route(deps, env, info, &route_id, new_route),
        ExecuteMsg::RemoveRoute { route_id } => execute_remove_route(deps, env, info, &route_id),
    }
}

pub fn execute_add_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    route_id: &RouteId,
    route: Route,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    if ROUTES.has(deps.storage, route_id) {
        return Err(ContractError::DestinationAlreadyExists);
    }

    ROUTES.save(deps.storage, route_id, &route)?;

    Ok(Response::new()
        .add_attribute("action", "add_route")
        .add_attribute("destination", route_id.to_string())
        .add_attribute("destination-value", route.to_string()))
}

pub fn execute_mutate_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    route_id: &RouteId,
    route: Route,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    ROUTES.save(deps.storage, route_id, &route)?;

    Ok(Response::new()
        .add_attribute("action", "mutate_route")
        .add_attribute("destination", route_id.to_string())
        .add_attribute("destination-value", route.to_string()))
}

pub fn execute_remove_route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    route_id: &RouteId,
) -> ContractResult<Response> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    if !ROUTES.has(deps.storage, route_id) {
        return Err(ContractError::DestinationNotExists);
    }

    ROUTES.remove(deps.storage, route_id);

    Ok(Response::new()
        .add_attribute("action", "remove_route")
        .add_attribute("destination", format!("{}", route_id)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        QueryMsg::GetMemo {
            route_id,
            timeout,
            retries,
            actual_memo,
        } => Ok(to_binary(&handle_get_memo(
            deps,
            route_id,
            timeout,
            retries,
            actual_memo,
        )?)?),
        QueryMsg::GetRoute { route_id } => Ok(to_binary(&handle_get_route(deps, route_id)?)?),
        QueryMsg::ListRoutes {} => Ok(to_binary(&handle_list_routes(deps)?)?),
    }
}

fn handle_get_memo(
    deps: Deps,
    route_id: RouteId,
    timeout: String,
    retries: i64,
    actual_memo: Option<Binary>,
) -> ContractResult<GetMemoResponse> {
    let route = ROUTES
        .may_load(deps.storage, &route_id)?
        .ok_or(ContractError::DestinationNotExists)?;
    match route.hop {
        Some(hop) => Ok(GetMemoResponse {
            channel: route.channel,
            port: route.port,
            memo: MemoResponse::Forward(hop.to_memo(timeout, retries, actual_memo)),
        }),
        None => Ok(GetMemoResponse {
            channel: route.channel,
            port: route.port,
            memo: MemoResponse::Actual(actual_memo),
        }),
    }
}

fn handle_get_route(deps: Deps, route_id: RouteId) -> ContractResult<GetRouteResponse> {
    let destination = ROUTES
        .may_load(deps.storage, &route_id)?
        .ok_or(ContractError::DestinationNotExists)?;
    Ok(GetRouteResponse { destination })
}

fn handle_list_routes(deps: Deps) -> ContractResult<ListRoutesResponse> {
    let routes: StdResult<Vec<(RouteId, Route)>> = ROUTES
        .range(deps.storage, None, None, Order::Descending)
        .collect();
    Ok(ListRoutesResponse {
        routes: routes?
            .iter()
            .map(|(dst, val)| (dst.clone(), val.clone()))
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env},
    };

    use crate::route::{Hop, Destination};

    use super::*;

    #[test]
    fn query_list_routes_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let hop1 = Hop::new(
            "channel-1",
            "transfer",
            "cosmosBob",
            Some(Hop::new("channel-2", "transfer", "quasarBob", None)),
        );

        let route1 = Route::new("channel-2", "transfer", Some(hop1));

        let hop2 = Hop::new(
            "channel-866",
            "transfer",
            "osmoBob",
            Some(Hop::new("channel-644", "transfer", "quasarBob", None)),
        );

        let route2 = Route::new("channel-3", "transfer", Some(hop2));

        ROUTES
            .save(
                deps.as_mut().storage,
                &RouteId {
                    destination: Destination("osmosis".to_string()),
                    asset: "ibc/123".to_string(),
                },
                &route1,
            )
            .unwrap();

        ROUTES
            .save(
                deps.as_mut().storage,
                &RouteId {
                    destination: Destination("gaia".to_string()),
                    asset: "osmo".to_string(),
                },
                &route2,
            )
            .unwrap();

        let result = query(deps.as_ref(), env, crate::msg::QueryMsg::ListRoutes {}).unwrap();
        let response: ListRoutesResponse = from_binary(&result).unwrap();
        assert_eq!(
            response,
            ListRoutesResponse {
                routes: vec![
                    (
                        RouteId {
                            destination: Destination("osmosis".to_string()),
                            asset: "ibc/123".to_string()
                        },
                        route1
                    ),
                    (
                        RouteId {
                            destination: Destination("gaia".to_string()),
                            asset: "osmo".to_string()
                        },
                        route2
                    )
                ]
            }
        )
    }
}
