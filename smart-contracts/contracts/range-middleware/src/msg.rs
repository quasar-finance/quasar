use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::{ExecuteFns, QueryFns};

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
#[cfg_attr(not(target_arch = "wasm32"), derive(ExecuteFns))]
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
#[cfg_attr(not(target_arch = "wasm32"), derive(QueryFns))]
#[derive(QueryResponses)]
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
