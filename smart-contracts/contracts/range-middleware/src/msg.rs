use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;
use mars_owner::OwnerUpdate;

use crate::{
    admin::{execute::AdminExecuteMsg, query::AdminQueryMsg},
    range::{execute::RangeExecuteMsg, query::RangeQueryMsg},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub range_submitter_owner: String,
    pub range_executor_owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// range operations
    RangeMsg(RangeExecuteMsg),
    /// admin operations
    AdminMsg(AdminExecuteMsg),
    /// contract owner updtae
    UpdateOwner(OwnerUpdate),
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
#[cw_serde]
pub struct MigrateMsg {}
