use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Map;
use mars_owner::Owner;

#[cw_serde]
pub struct NewRange {
    pub cl_vault_address: String,
    pub lower_price: Decimal,
    pub upper_price: Decimal,
}

pub const PENDING_RANGES: Map<Addr, NewRange> = Map::new("pending_ranges");

pub const OWNER: Owner = Owner::new("owner");
pub const RANGE_SUBMITTER_OWNER: Owner = Owner::new("range_submitter_owner");
pub const RANGE_EXECUTOR_OWNER: Owner = Owner::new("range_executor_owner");
