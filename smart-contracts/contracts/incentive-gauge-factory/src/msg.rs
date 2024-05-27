use cosmwasm_schema::{cw_serde, QueryResponses};
// use cosmwasm_std::Addr;

use crate::types::{Fee, Gauge, GaugeKind};

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// contract admin (defaults to sender during initilization)
    pub admin: Option<String>,

    /// guage contract code id (can be set later on)
    pub gauge_codeid: Option<u64>,
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

    Remove {
        addr: String
    },

    /// addr is the gauge contract
    MerkleUpdate {
        addr: String,
        merkle: String,
    },
    // addr is the gauge contract
    // GaugePause { addr: String },
}

#[allow(clippy::large_enum_variant)]
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

    #[returns(GaugeListResponse)]
    ListGauges {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

/// This is used to when quering for the list of the gauges
/// from JavaScript this can be accessed like this:
/// const gauge = { gauge: list.gauges[0], kind: list.kinds[0], fee: list.fees[0] }
#[cw_serde]
pub struct GaugeListResponse {
    pub gauges: Vec<Gauge>,
    pub kinds: Vec<GaugeKind>,
    pub fees: Vec<Fee>,
}

#[cw_serde]
pub struct GaugeResponse {
    pub gauge: Gauge,
    pub kind: GaugeKind,
    pub fee: Fee,
}