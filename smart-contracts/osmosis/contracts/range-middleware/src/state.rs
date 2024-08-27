use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct NewRange {
    pub cl_vault_address: String,
    pub lower_price: Decimal,
    pub upper_price: Decimal,
}

pub const PENDING_RANGES: Map<Addr, NewRange> = Map::new("pending_ranges");
pub const RANGE_SUBMITTER_ADMIN: Item<Addr> = Item::new("range_submitter_admin");
pub const RANGE_EXECUTOR_ADMIN: Item<Addr> = Item::new("range_executor_admin");
