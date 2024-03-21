use cosmwasm_std::{Addr, Decimal, Coin};
use cw_storage_plus::{Item, Map};
use cosmwasm_schema::cw_serde;

use crate::gauge::Gauge;

pub const GAUGES: Map<Addr, Gauge> = Map::new("gauges");

pub const GAUGE_CODE_ID: Item<u64> = Item::new("gauge_code_id");

