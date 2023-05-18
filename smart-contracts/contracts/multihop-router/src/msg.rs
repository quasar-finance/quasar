use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::state::{Destination, Hop, Memo};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddRoute { destination: String, hops: Hop },
    MutateRoute { destination: String, hops: Hop },
    RemoveRoute { destination: String },
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
    pub memo: Memo,
}

#[cw_serde]
pub struct GetRouteResponse {
    pub hops: Hop,
}

#[cw_serde]
pub struct ListRoutesResponse {
    pub routes: Vec<(Destination, Hop)>,
}
