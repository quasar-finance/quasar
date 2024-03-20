use crate::incentives::CoinVec;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

pub const CLAIMED_INCENTIVES: Map<Addr, CoinVec> = Map::new("claimed_incentives");
pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const INCENTIVES_ADMIN: Item<Addr> = Item::new("incentives_admin");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct ClaimAccount {
    pub proof: Vec<Vec<u8>>,
    pub coins: CoinVec,
}

#[cw_serde]
pub struct Config {
    pub clawback_address: Addr,
    pub start_block: u64,
    pub end_block: u64,
    pub expiration_block: u64,
    pub fee: Option<Fee>,
    pub total_incentives: Vec<Coin>
}

#[cw_serde]
pub struct Fee {
    pub fee_address: Addr,
    pub fee: Decimal,
}
