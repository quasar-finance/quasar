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
pub enum ExecuteMsg {
    RangeMsg(RangeExecuteMsg),
    AdminMsg(AdminExecuteMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Empty)]
    RangeQuery(RangeQueryMsg),
    #[returns(Empty)]
    AdminQuery(AdminQueryMsg),
}
#[cw_serde]
pub struct MigrateMsg {}
