use cosmwasm_schema::{cw_serde, QueryResponses};
// use cosmwasm_std::Addr;

use crate::types::{Fee, Gauge, GaugeKind};

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum FeeMsg {
    /// addr is the gauge contract
    Distribute {
        addr: String,
    },

    /// addr is the gauge contract
    Update {
        addr: String,
        fees: Fee,
    },
}

#[cw_serde]
pub enum GaugeMsg {
    CodeUpdate {
        code: u64,
    },

    Create {
        kind: GaugeKind,
        gauge: Gauge,
        fee: Fee,
    },

    /// addr is the gauge contract
    Update {
        addr: String,
        gauge: Gauge,
        fees: Option<Fee>,
        kind: Option<GaugeKind>,
    },

    /// addr is the gauge contract
    MerkleUpdate {
        addr: String,
        merkle: String,
    },
    // addr is the gauge contract
    // GaugePause { addr: String },
}

#[cw_serde]
pub enum ExecuteMsg {
    GaugeMsg(GaugeMsg),

    FeeMsg(FeeMsg),

    AdminUpdate {
        addr: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GaugeResponse)]
    Gauge { address: String },

    #[returns(ListGaugesResponse)]
    ListGauges {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct GaugeResponse {}

#[cw_serde]
pub struct ListGaugesResponse {}
