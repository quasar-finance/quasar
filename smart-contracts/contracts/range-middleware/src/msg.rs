use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Empty};
use cw_dex_router::operations::SwapOperationsListUnchecked;

use crate::{
    admin::{execute::AdminExecuteMsg, query::AdminQueryMsg},
    range::{execute::RangeExecuteMsg, query::RangeQueryMsg},
    state::NewRange,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub range_submitter_admin: String,
    pub range_executor_admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// range operations
    RangeMsg(RangeExecuteMsg),
    /// admin operations
    AdminMsg(AdminExecuteMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// range queries
    #[returns(Empty)]
    RangeQuery(RangeQueryMsg),
    /// admin queries
    #[returns(Empty)]
    AdminQuery(AdminQueryMsg),
}
