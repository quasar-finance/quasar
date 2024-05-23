use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::gauge::{Gauge, GaugeKind};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateIncentiveGauge { kind: GaugeKind, gauge: Gauge },
    ClaimGaugeFees { gauge_address: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GaugeResponse)]
    Gauge { address: String },

    #[returns(ListGaugesResponse)]
    ListGauges {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct GaugeResponse {}

#[cw_serde]
pub struct ListGaugesResponse {}
