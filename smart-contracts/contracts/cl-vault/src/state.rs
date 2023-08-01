use apollo_cw_asset::AssetInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address"); // aliceaddress
pub const VAULT_CONFIG: Item<VaultConfig> = Item::new("vault_config");
// We should move base_token and quote_token into PoolConfig (see struct below)
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
    pub base_token: String,  // todo: Verify in instantiate message
    pub quote_token: String, // todo: Verify in instantiate message
}

impl PoolConfig {
    pub fn pool_contains_token(&self, token: impl Into<String>) -> bool {
        vec![&self.base_token, &self.quote_token].contains(&&token.into())
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

#[cfg(test)]
mod tests {
    use super::PoolConfig;

    #[test]
    fn test_pool_contains_token() {
        let pool_config = PoolConfig {
            pool_id: 1,
            base_token: "token1".to_string(),
            quote_token: "token2".to_string(),
        };

        assert!(pool_config.pool_contains_token("token1"));
        assert!(pool_config.pool_contains_token("token2"));
        assert!(!pool_config.pool_contains_token("token3"));
    }
}
