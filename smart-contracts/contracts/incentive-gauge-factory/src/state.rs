use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::{Item, Map};

use crate::gauge::Gauge;

pub const GAUGES: Map<Addr, Gauge> = Map::new("gauges");

pub const GAUGE_CODE_ID: Item<u64> = Item::new("gauge_code_id");