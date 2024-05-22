use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;

use crate::{
    admin::{execute::AdminExecuteMsg, query::AdminQueryMsg},
    incentives::{execute::IncentivesExecuteMsg, query::IncentivesQueryMsg},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub incentives_admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// range operations
    IncentivesMsg(IncentivesExecuteMsg),
    /// admin operations
    AdminMsg(AdminExecuteMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// range queries
    #[returns(Empty)]
    IncentivesQuery(IncentivesQueryMsg),
    /// admin queries
    #[returns(Empty)]
    AdminQuery(AdminQueryMsg),
}

#[cw_serde]
pub struct MigrateMsg {}
