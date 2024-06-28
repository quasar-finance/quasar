use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;

use crate::{
    admin::{execute::AdminExecuteMsg, query::AdminQueryMsg},
    range::{execute::RangeExecuteMsg, query::RangeQueryMsg},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub range_submitter_admin: String,
    pub range_executor_admin: String,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    /// range operations
    RangeMsg(RangeExecuteMsg),
    /// admin operations
    AdminMsg(AdminExecuteMsg),
}

impl From<RangeExecuteMsg> for ExecuteMsg {
    fn from(msg: RangeExecuteMsg) -> Self {
        ExecuteMsg::RangeMsg(msg)
    }
}

impl From<AdminExecuteMsg> for ExecuteMsg {
    fn from(msg: AdminExecuteMsg) -> Self {
        ExecuteMsg::AdminMsg(msg)
    }
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    /// range queries
    #[returns(Empty)]
    RangeQuery(RangeQueryMsg),
    /// admin queries
    #[returns(Empty)]
    AdminQuery(AdminQueryMsg),
}

impl From<RangeQueryMsg> for QueryMsg {
    fn from(msg: RangeQueryMsg) -> Self {
        QueryMsg::RangeQuery(msg)
    }
}

impl From<AdminQueryMsg> for QueryMsg {
    fn from(msg: AdminQueryMsg) -> Self {
        QueryMsg::AdminQuery(msg)
    }
}

#[cw_serde]
pub struct MigrateMsg {}
