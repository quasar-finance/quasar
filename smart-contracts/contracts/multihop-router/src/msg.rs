use cosmwasm_schema::{cw_serde, QueryResponses};

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
    #[returns(GetMemoResponse)]
    GetMemo { destination: String },
    #[returns(GetRouteResponse)]
    GetRoute { destination: String },
    #[returns(ListRoutesResponse)]
    ListRoutes {},
}

#[cw_serde]
pub struct GetMemoResponse {
    memo: Memo,
}

#[cw_serde]
pub struct GetRouteResponse {
    hops: Hop,
}

#[cw_serde]
pub struct ListRoutesResponse {
    memos: Vec<(Destination, Hop)>,
}
