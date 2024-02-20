use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::incentives::CoinVec;

pub const CLAIMED_INCENTIVES: Map<Addr, CoinVec> = Map::new("claimed_incentives");

pub const MERKLE_ROOT: Item<String> = Item::new("merkle_root");
pub const INCENTIVES_ADMIN: Item<Addr> = Item::new("incentives_admin");

#[cw_serde]
pub struct MerkleProof {
    pub is_left_sibling: String,
    pub hash: String,
}

#[cw_serde]
pub struct ClaimAccount {
    pub seed: String,
    pub proofs: Vec<MerkleProof>,
    pub coins: CoinVec,
}
