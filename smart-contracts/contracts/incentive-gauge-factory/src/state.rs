use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};

use crate::types::{Fee, Gauge, GaugeInProcess, GaugeKind};

/// Admin of the contract who can update config or set new admin.
/// Not the same as the contract initializer admin
pub const ADMIN: Admin = Admin::new("owner");

pub const GAUGE_CODE: Item<u64> = Item::new("gauge_codes");

/// addr is the guage contract address
pub const GAUGES: Map<Addr, Gauge> = Map::new("gauges");

/// addr is the guage contract address
pub const GAUGE_FEES: Map<Addr, Fee> = Map::new("gauge_fees");

/// addr is the guage contract address
pub const GAUGE_KINDS: Map<Addr, GaugeKind> = Map::new("gauge_kinds");

pub const GAUGE_IN_PROCESS: Item<GaugeInProcess> = Item::new("tmp_gauge");
