use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::gauge::{Gauge, GaugeType};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateIncentiveGauge { r#type: GaugeType, gauge: Gauge },
    ClaimGaugeFees { gauge_address: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GaugeResponse)]
    Gauge {address: String},
    #[returns(ListGaugesResponse)]
    ListGauges {}
}

#[cw_serde]
pub struct GaugeResponse {

}

#[cw_serde]
pub struct ListGaugesResponse {

}
