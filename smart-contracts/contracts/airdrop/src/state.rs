use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_asset::AssetInfo;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");
pub const AIRDROP_ID: Item<Uint128> = Item::new("airdrop_id");
pub const AIRDROP_CONFIGS: Map<Uint128, AirdropConfig> = Map::new("airdrop_configs");
pub const USER_INFO: Map<Addr, Vec<UserInfo>> = Map::new("user_info");
pub const AIRDROP_INFO: Map<Uint128, Map<Addr, AirdropInfo>> = Map::new("airdrop_info");

//----------------------------------------------------------------------------------------
// Storage types
//----------------------------------------------------------------------------------------

#[cw_serde]
pub struct Config {
    /// Account who can update config
    pub owner: Addr,
    ///  QUASAR token address for funding and refund
    pub quasar_funding_address: Addr,
}

#[cw_serde]
pub struct AirdropConfig {
    /// each airdrop contains a unique airdrop ID
    pub airdrop_id: Uint128,
    /// every airdrop contains a description of it
    pub airdrop_description: String,
    /// tokens to be airdropped
    pub airdrop_token: AssetInfo,
    /// total claimed amount, zero initially
    pub total_claimed: Uint128,
    /// starting time from which users can claim airdrop
    pub from_timestamp: u64,
    /// end time after which users cannot claim airdrop
    pub to_timestamp: u64,
    /// flag to enable and disable claims for the given airdrop in case of any emergency
    pub claim_enabled: bool,
    /// total amount of unclaimed tokens, equal to airdrop_tokens_amount
    pub unclaimed_tokens: Uint128,
}

#[cw_serde]
pub struct UserInfo {
    /// airdrop id users info is attached to
    pub airdrop_id: Uint128,
    /// total airdrop tokens claimable by the user
    pub claimable_amount: Uint128,
    /// boolean value indicating if the user has withdrawn the remaining tokens
    pub claimed_flag: bool,
}

#[cw_serde]
pub struct AirdropInfo {
    /// total claimable amount for the user
    pub claimable_amount: Uint128,
    /// boolean value indicating if the user has withdrawn the remaining tokens
    pub claimed_flag: Uint128,
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            airdrop_id: Uint128::zero(),
            claimable_amount: Uint128::zero(),
            claimed_flag: false,
        }
    }
}
