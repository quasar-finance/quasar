use apollo_cw_asset::AssetInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

use crate::rewards::Rewards;

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address"); // aliceaddress
pub const VAULT_CONFIG: Item<Config> = Item::new("vault_config");
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
}

pub const POSITION: Item<Position> = Item::new("position");

/// current rewards are the rewards being gathered, these can be both spread rewards aswell as incentives
pub const CURRENT_REWARDS: Item<Rewards> = Item::new("rewards");

pub const USER_REWARDS: Map<Addr, Rewards> = Map::new("user_rewards");

pub const STRATEGIST_REWARDS: Item<Rewards> = Item::new("strategist_rewards");

pub const LOCKUP_DURATION: Item<cw_utils::Duration> = Item::new("lockup_duration");

pub const LOCKED_TOKENS: Map<Addr, Uint128> = Map::new("locked_tokens");
pub const LOCKED_TOTAL: Item<Uint128> = Item::new("locked_total");

#[cw_serde]
pub struct Position {
    pub position_id: u64,
}

/// Base config struct for the contract.
#[cw_serde]
pub struct Config {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
}
