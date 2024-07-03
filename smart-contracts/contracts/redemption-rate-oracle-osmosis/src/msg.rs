use crate::contract::RedemptionRateOracle;

use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg(not(target_arch = "wasm32"))]
use cosmwasm_std::Decimal;
use mars_owner::OwnerUpdate;

abstract_app::app_msg_types!(
    RedemptionRateOracle,
    RedemptionRateOracleExecuteMsg,
    RedemptionRateOracleQueryMsg
);

#[cw_serde]
pub struct RedemptionRateOracleInstantiateMsg {
    pub owner: String,
    pub stride_oracle: String,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum RedemptionRateOracleExecuteMsg {
    Update { stride_oracle: Option<String> },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
pub struct OracleInfo {
    pub name: String,
    pub address: String,
}

#[cw_serde]
pub struct OraclesResponse {
    pub oracles: Vec<OracleInfo>,
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum RedemptionRateOracleQueryMsg {
    #[returns(Decimal)]
    RedemptionRate { denom: String },
    #[returns(OraclesResponse)]
    Oracles {},
}

#[cw_serde]
pub struct RedemptionRateOracleMigrateMsg {}
