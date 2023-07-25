use apollo_cw_asset::AssetInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Decimal256, Uint128};

use cw_storage_plus::{Item, Map};

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address"); // aliceaddress
pub const VAULT_CONFIG: Item<Config> = Item::new("vault_config");
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
}

pub const POSITION: Item<Position> = Item::new("position");

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

// TODO: this could be done using normal hashmaps maybe?
#[cw_serde]
pub struct TickExpIndexData {
    pub initial_price: Decimal256,
    pub max_price: Decimal256,
    pub additive_increment_per_tick: Decimal256,
    pub initial_tick: i64,
}

pub const TICK_EXP_CACHE: Map<i64, TickExpIndexData> = Map::new("tick_exp_cache");

// this is the info that the contract needs to know about the pool. Saved during instantiation
#[cw_serde]
pub struct Investment {
    /// Owner created the contract and takes a cut
    pub owner: Addr,
    /// Each vault has one specific token that is used for deposits, withdrawals and accounting
    pub base_denom: String,
    /// The quote denom used by the vault (asset1)
    pub quote_denom: String,
    /// the osmosis pool id used by the vault
    pub pool_id: u64,
}

pub const INVESTMENT: Item<Investment> = Item::new("investment");

#[cw_serde]
pub struct Strategy {
    pub lower_tick: i64,
    pub upper_tick: i64,
    // slippage tolerance in basis points (1bps = 0.01%)
    pub slippage_tolerance: Uint128,
}

pub const STRATEGY: Item<Strategy> = Item::new("strategy");

pub const USER_BALANCE: Map<Addr, Uint128> = Map::new("user_balance");

pub const USER_REWARDS: Map<Addr, Vec<Coin>> = Map::new("user_rewards");

#[cw_serde]
pub enum Replies {
    Swap { user_addr: Addr, amount0: Uint128 },
    CreatePosition { user_addr: Addr },
}

// TODO: can we use one map for all replies?
pub const REPLIES: Map<u64, Replies> = Map::new("replies");
