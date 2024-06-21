use std::collections::VecDeque;

use cl_vault::msg::{CreatePosition, DecreaseFunds, DeletePosition, IncreaseFunds};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct NewRange {
    pub position_id: u64,
    pub lower_price: Decimal,
    pub upper_price: Decimal,
}


#[cw_serde]
pub struct RangeUpdates {
    pub cl_vault_address: String,
    pub updates: VecDeque<UpdateActions>,
}

#[cw_serde]
pub enum UpdateActions {
    CreatePosition(CreatePosition),
    DeletePosition(DeletePosition),
    DecreaseFunds(DecreaseFunds),
    IncreaseFunds(IncreaseFunds),
    NewRange(NewRange)
}

pub const PENDING_RANGES: Map<Addr, RangeUpdates> = Map::new("pending_ranges");
pub const RANGE_SUBMITTER_ADMIN: Item<Addr> = Item::new("range_submitter_admin");
pub const RANGE_EXECUTOR_ADMIN: Item<Addr> = Item::new("range_executor_admin");
