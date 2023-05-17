use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{Destination, Hop, Memo};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddRoute { hop: Hop },
    MutateRoute { destination: String },
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
