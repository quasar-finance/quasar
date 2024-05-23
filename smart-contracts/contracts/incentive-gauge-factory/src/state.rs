use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::types::{Gauge, GaugesCodes};

pub const GAUGES: Map<Addr, Gauge> = Map::new("gauges");
pub const GAUGE_CODES: Item<GaugesCodes> = Item::new("gauge_codes");
