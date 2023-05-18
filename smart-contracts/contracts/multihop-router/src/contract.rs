#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::helpers::is_contract_admin;
use crate::msg::{
    ExecuteMsg, GetMemoResponse, GetRouteResponse, InstantiateMsg, ListRoutesResponse, QueryMsg,
};
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        QueryMsg::GetMemo {
            destination,
            timeout,
            retries,
            actual_memo,
        } => Ok(to_binary(&handle_get_memo(
            deps,
            destination.into(),
            timeout,
            retries,
            actual_memo,
        )?)?),
        QueryMsg::GetRoute { destination } => {
            Ok(to_binary(&handle_get_route(deps, destination.into())?)?)
        }
        QueryMsg::ListRoutes {} => Ok(to_binary(&handle_list_routes(deps)?)?),
    }
}

fn handle_get_memo(
    deps: Deps,
    dst: Destination,
    timeout: String,
    retries: i64,
    actual_memo: Option<Binary>,
) -> ContractResult<GetMemoResponse> {
    let memo = ROUTES
        .may_load(deps.storage, &dst)?
        .ok_or(ContractError::DestinationNotExists)?
        .to_memo(timeout, retries, actual_memo);
    Ok(GetMemoResponse { memo })
}

fn handle_get_route(deps: Deps, dst: Destination) -> ContractResult<GetRouteResponse> {
    let hops = ROUTES
        .may_load(deps.storage, &dst)?
        .ok_or(ContractError::DestinationNotExists)?;
    Ok(GetRouteResponse { hops })
}

fn handle_list_routes(deps: Deps) -> ContractResult<ListRoutesResponse> {
    let routes: StdResult<Vec<(Destination, Hop)>> = ROUTES
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

    use crate::{
        msg::ListRoutesResponse,
        state::{Destination, Hop, ROUTES},
    };

    use super::query;

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

        let hop2 = Hop::new(
            "channel-866",
            "transfer",
            "osmoBob",
            Some(Hop::new("channel-644", "transfer", "quasarBob", None)),
        );

        ROUTES
            .save(
                deps.as_mut().storage,
                &Destination("osmosis".to_string()),
                &hop1,
            )
            .unwrap();

        ROUTES
            .save(
                deps.as_mut().storage,
                &Destination("gaia".to_string()),
                &hop2,
            )
            .unwrap();

        let result = query(deps.as_ref(), env, crate::msg::QueryMsg::ListRoutes {}).unwrap();
        let response: ListRoutesResponse = from_binary(&result).unwrap();
        assert_eq!(
            response,
            ListRoutesResponse {
                routes: vec![
                    ("osmosis".to_string().into(), hop1),
                    ("gaia".to_string().into(), hop2)
                ]
            }
        )
    }
}
