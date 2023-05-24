use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::state::{Route, Memo, RouteName};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddRoute { destination_name: String, destination: Route },
    MutateRoute { destination_name: String, destination: Route },
    RemoveRoute { destination_name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // TODO change timeout to either a Timestamp or a Duration, also find a datastructure that is always a json
    #[returns(GetMemoResponse)]
    GetMemo {
        destination: String,
        timeout: String,
        retries: i64,
        actual_memo: Option<Binary>,
    },
    #[returns(GetRouteResponse)]
    GetRoute { destination: String },
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
    Actual(Option<Binary>)
}

#[cw_serde]
pub struct GetRouteResponse {
    pub destination: Route,
}

#[cw_serde]
pub struct ListRoutesResponse {
    pub routes: Vec<(RouteName, Route)>,
}
