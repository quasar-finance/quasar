use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};
use cw_storage_plus::{Item, Map};

use crate::rewards::Rewards;

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address"); // aliceaddress
pub const VAULT_CONFIG: Item<VaultConfig> = Item::new("vault_config");
pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
    pub token0: String, // todo: Verify in instantiate message
    pub token1: String, // todo: Verify in instantiate message
}

impl PoolConfig {
    pub fn pool_contains_token(&self, token: impl Into<String>) -> bool {
        vec![&self.token0, &self.token1].contains(&&token.into())
    }
}

pub const POSITION: Item<Position> = Item::new("position");

#[cw_serde]
pub struct Position {
    pub position_id: u64,
}

/// Base config struct for the contract.
#[cw_serde]
pub struct VaultConfig {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
}

pub const VAULT_DENOM: Item<String> = Item::new("vault_denom");

/// current rewards are the rewards being gathered, these can be both spread rewards aswell as incentives
pub const CURRENT_REWARDS: Item<Rewards> = Item::new("rewards");

pub const USER_REWARDS: Map<Addr, Rewards> = Map::new("user_rewards");

pub const STRATEGIST_REWARDS: Item<Rewards> = Item::new("strategist_rewards");

pub const LOCKUP_DURATION: Item<cw_utils::Duration> = Item::new("lockup_duration");

pub const LOCKED_SHARES: Map<Addr, Uint128> = Map::new("locked_tokens");
pub const LOCKED_TOTAL: Item<Uint128> = Item::new("locked_total");

#[cw_serde]
pub struct TickExpIndexData {
    pub initial_price: Decimal256,
    pub max_price: Decimal256,
    pub additive_increment_per_tick: Decimal256,
    pub initial_tick: i64,
}

pub const TICK_EXP_CACHE: Map<i64, TickExpIndexData> = Map::new("tick_exp_cache");
pub const CURRENT_WITHDRAWER: Item<Addr> = Item::new("current_withdrawer");

#[cfg(test)]
mod tests {
    use super::PoolConfig;

    #[test]
    fn test_pool_contains_token() {
        let pool_config = PoolConfig {
            pool_id: 1,
            token0: "token1".to_string(),
            token1: "token2".to_string(),
        };

        assert!(pool_config.pool_contains_token("token1"));
        assert!(pool_config.pool_contains_token("token2"));
        assert!(!pool_config.pool_contains_token("token3"));
    }
}
