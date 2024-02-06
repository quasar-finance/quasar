use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::{Item, Map};

pub const CLAIMED_INCENTIVES: Map<Addr, Vec<Coin>> = Map::new("claimed_incentives");

pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const INCENTIVES_ADMIN: Item<Addr> = Item::new("incentives_admin");
