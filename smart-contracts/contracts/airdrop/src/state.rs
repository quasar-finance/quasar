use std::string::String;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_asset::AssetInfo;
use cw_storage_plus::{Item, Map};

pub const AIRDROP_CONFIG: Item<AirdropConfig> = Item::new("airdrop_config");
pub const USER_INFO: Map<String, UserInfo> = Map::new("user_info");
pub const REPLY_MAP: Map<u64, String> = Map::new("reply_map");

//----------------------------------------------------------------------------------------
// Storage types
//----------------------------------------------------------------------------------------

#[cw_serde]
pub struct AirdropConfig {
    /// every airdrop contains a description of it
    pub airdrop_description: String,
    /// token amount to be airdropped
    pub airdrop_amount: Uint128,
    /// token denom to be airdropped
    pub airdrop_asset: AssetInfo,
    /// total claimed amount, zero initially
    pub total_claimed: Uint128,
    /// starting time from which users can claim airdrop
    pub start_height: u64,
    /// end time after which users cannot claim airdrop
    pub end_height: u64,
}

#[cw_serde]
pub struct UserInfo {
    /// total airdrop tokens claimable by the user
    pub claimable_amount: Uint128,
    /// boolean value indicating if the user has withdrawn the remaining tokens
    pub claimed_flag: bool,
}

impl AirdropConfig {
    pub fn get_start_height(&self) -> u64 {
        self.start_height
    }
    pub fn get_end_heights(&self) -> u64 {
        self.start_height
    }
    pub fn get_config(self) -> AirdropConfig {
        self
    }
}

impl UserInfo {
    pub fn get_claimable_amount(&self) -> Uint128 {
        self.claimable_amount
    }
    pub fn get_claimed_flag(&self) -> bool {
        self.claimed_flag
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            claimable_amount: Uint128::zero(),
            claimed_flag: false,
        }
    }
}
