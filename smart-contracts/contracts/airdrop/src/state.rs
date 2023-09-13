use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

pub const AIRDROP_CONFIG: Item<AirdropConfig> = Item::new("airdrop_config");
pub const USER_INFO: Map<String, Vec<UserInfo>> = Map::new("user_info");

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
    pub airdrop_denom: Uint128,
    /// total claimed amount, zero initially
    pub total_claimed: Uint128,
    /// starting time from which users can claim airdrop
    pub start_height: u64,
    /// end time after which users cannot claim airdrop
    pub end_height: u64,
    /// flag to enable and disable claims for the given airdrop in case of any emergency
    pub claim_enabled: bool,
    /// total amount of unclaimed tokens, equal to airdrop_tokens_amount
    pub unclaimed_tokens: Uint128,
}

#[cw_serde]
pub struct UserInfo {
    /// total airdrop tokens claimable by the user
    pub claimable_amount: Uint128,
    /// boolean value indicating if the user has withdrawn the remaining tokens
    pub claimed_flag: bool,
}

impl AirdropConfig {
    pub fn get_start_and_end_heights(&self) -> (u64, u64) {
        (self.start_height, self.end_height)
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
