use crate::vault::merge::CurrentMergeWithdraw;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Decimal256, Uint128};
use cw_storage_plus::{Deque, Item, Map};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

#[cw_serde]
pub struct Metadata {
    /// the underlying thesis of the vault's positions, eg aggresive
    pub thesis: String,
    pub name: String,
}

pub const METADATA: Item<Metadata> = Item::new("metadata");

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address");
pub const RANGE_ADMIN: Item<Addr> = Item::new("range_admin");

#[cw_serde]
pub struct VaultConfig {
    pub performance_fee: Decimal,
    pub treasury: Addr,
    pub swap_max_slippage: Decimal,
    pub dex_router: Addr,
    pub swap_admin: Addr,
}

pub const VAULT_CONFIG: Item<VaultConfig> = Item::new("vault_config");
pub const VAULT_DENOM: Item<String> = Item::new("vault_denom");

#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
    pub token0: String,
    pub token1: String,
}

impl PoolConfig {
    pub fn pool_contains_token(&self, token: &str) -> bool {
        self.token0 == token || self.token1 == token
    }
}

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

#[cw_serde]
pub struct Position {
    pub position_id: u64,
    pub join_time: u64, // env block time at time of creation, or taken by osmosis protocol response
    pub claim_after: Option<u64>, // this should be off chain computed and set in order to avoid forfeiting incentives
}

pub const POSITION: Item<Position> = Item::new("position_v2");

pub const SHARES: Map<Addr, Uint128> = Map::new("shares");

pub const CURRENT_MERGE: Deque<CurrentMergeWithdraw> = Deque::new("current_merge");

#[cw_serde]
pub struct CurrentMergePosition {
    pub lower_tick: i64,
    pub upper_tick: i64,
}

pub const CURRENT_MERGE_POSITION: Item<CurrentMergePosition> = Item::new("current_merge_position");

#[cw_serde]
pub struct CurrentDeposit {
    pub token0_in: Uint128,
    pub token1_in: Uint128,
    pub sender: Addr,
}

pub const CURRENT_SWAP_ANY_DEPOSIT: Item<(Coin, Addr, (Uint128, Uint128))> =
    Item::new("current_swap_any_deposit");

pub const DEX_ROUTER: Item<Addr> = Item::new("dex_router");

#[cw_serde]
pub struct ModifyRangeState {
    pub lower_tick: i64,
    pub upper_tick: i64,
    pub max_slippage: Decimal,
    pub new_range_position_ids: Vec<u64>,
    pub ratio_of_swappable_funds_to_use: Decimal,
    pub twap_window_seconds: u64,
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
}

pub const MODIFY_RANGE_STATE: Item<Option<ModifyRangeState>> = Item::new("modify_range_state");

#[cw_serde]
pub struct SwapDepositMergeState {
    pub target_lower_tick: i64,
    pub target_upper_tick: i64,
    pub target_range_position_ids: Vec<u64>,
}

pub const SWAP_DEPOSIT_MERGE_STATE: Item<SwapDepositMergeState> =
    Item::new("swap_deposit_merge_state");

#[cw_serde]
pub struct TickExpIndexData {
    pub initial_price: Decimal256,
    pub max_price: Decimal256,
    pub additive_increment_per_tick: Decimal256,
    pub initial_tick: i64,
}

pub const TICK_EXP_CACHE: Map<i64, TickExpIndexData> = Map::new("tick_exp_cache");
pub const CURRENT_WITHDRAWER: Item<Addr> = Item::new("current_withdrawer");
pub const CURRENT_WITHDRAWER_DUST: Item<(Uint128, Uint128)> = Item::new("current_withdrawer_dust");

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
