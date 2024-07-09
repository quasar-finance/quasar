use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub lst_adapter: Addr,
    pub dex_adapter: Addr,
    pub deposit_denom: String,
    pub lst_denom: String,
    pub denom: String,
    pub unbonding_time_seconds: u64,
}

#[cw_serde]
pub struct Claim {
    pub amount: Uint128,
    pub expiration: Timestamp,
}

pub const STATE: Item<Config> = Item::new("state");
pub const PENDING: Map<Addr, Vec<Claim>> = Map::new("pending");
