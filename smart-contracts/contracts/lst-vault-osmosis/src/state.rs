use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;
use quasar_types::denoms::LstDenom;

#[cw_serde]
pub struct Config {
    pub lst_adapter: Addr,
    pub dex_adapter: Addr,
    pub lst_denom: LstDenom,
    pub denom: String,
    pub unbonding_time_seconds: u64,
}

#[cw_serde]
pub struct Claim {
    pub amount: Uint128,
    pub expiration: Timestamp,
}

pub const OWNER: Owner = Owner::new("owner");
pub const CONFIG: Item<Config> = Item::new("state");
pub const PENDING_CLAIMS: Map<Addr, Vec<Claim>> = Map::new("pending");
