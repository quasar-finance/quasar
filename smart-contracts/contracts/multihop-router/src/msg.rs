use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::route::{Memo, Route, RouteId};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddRoute {
        route_id: RouteId,
        route: Route,
    },
    #[cfg(feature = "mutable")]
    MutateRoute {
        route_id: RouteId,
        new_route: Route,
    },
    #[cfg(feature = "mutable")]
    RemoveRoute {
        route_id: RouteId,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // TODO change timeout to either a Timestamp or a Duration, also find a datastructure that is always a json
    #[returns(GetMemoResponse)]
    GetMemo {
        route_id: RouteId,
        timeout: String,
        retries: i64,
        actual_memo: Option<Binary>,
    },
    #[returns(GetRouteResponse)]
    GetRoute { route_id: RouteId },
    #[returns(ListRoutesResponse)]
    ListRoutes {},
}

#[cw_serde]
pub struct GetMemoResponse {
    pub channel: String,
    pub port: String,
    pub memo: MemoResponse,
}

#[cw_serde]
#[serde(untagged)]
pub enum MemoResponse {
    Forward(Memo),
    Actual(Option<Binary>),
}

#[cw_serde]
pub struct GetRouteResponse {
    pub route: Route,
}

#[cw_serde]
pub struct ListRoutesResponse {
    pub routes: Vec<(RouteId, Route)>,
}
