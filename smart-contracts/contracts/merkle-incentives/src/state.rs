use crate::incentives::CoinVec;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const CLAIMED_INCENTIVES: Map<Addr, CoinVec> = Map::new("claimed_incentives");
pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const INCENTIVES_ADMIN: Item<Addr> = Item::new("incentives_admin");

#[cw_serde]
pub struct ClaimAccount {
    pub proof: Vec<Vec<u8>>,
    pub coins: CoinVec,
}
