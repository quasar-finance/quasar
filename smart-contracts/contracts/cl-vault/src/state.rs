use crate::{
    helpers::coinlist::CoinList,
    vault::{merge::CurrentMergeWithdraw, range::move_position::SwapDirection},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};
use cw_storage_plus::{Deque, Item, Map};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

/// metadata useful for display purposes
#[cw_serde]
pub struct Metadata {
    /// the underlying thesis of the vault's positions, eg aggresive
    pub thesis: String,
    /// the name of the vault
    pub name: String,
}

pub const METADATA: Item<Metadata> = Item::new("metadata");

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address");
pub const RANGE_ADMIN: Item<Addr> = Item::new("range_admin");

/// VAULT_CONFIG: Base config struct for the contract.
#[cw_serde]
pub struct VaultConfig {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
    /// swap max slippage
    pub swap_max_slippage: Decimal,
    /// Dex router address
    pub dex_router: Addr,
}

/// OLD_VAULT_CONFIG: Base config struct for the contract (pre-autocompound implementation).
#[cw_serde]
pub struct OldVaultConfig {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
    /// swap max slippage
    pub swap_max_slippage: Decimal,
}

pub const OLD_VAULT_CONFIG: Item<OldVaultConfig> = Item::new("vault_config");
pub const VAULT_CONFIG: Item<VaultConfig> = Item::new("vault_config_v2");
pub const VAULT_DENOM: Item<String> = Item::new("vault_denom");

/// MIGRATION_STATUS: Is a temporary state we need to paginate the migration process for the auto-compounding upgrade // TODO: Deprecate!
#[cw_serde]
pub enum MigrationStatus {
    Open,
    Closed,
}
pub const MIGRATION_STATUS: Item<MigrationStatus> = Item::new("migration_status");

/// POOL_CONFIG
#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
    pub token0: String,
    pub token1: String,
}

impl PoolConfig {
    pub fn pool_contains_token(&self, token: impl Into<String>) -> bool {
        [&self.token0, &self.token1].contains(&&token.into())
    }
}

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

/// POSITION
#[cw_serde]
pub struct OldPosition {
    pub position_id: u64,
}

#[cw_serde]
pub struct Position {
    pub position_id: u64,
    pub join_time: u64, // env block time at time of creation, or taken by osmosis protocol response
    pub claim_after: Option<u64>, // this should be off chain computed and set in order to avoid forfeiting incentives
}

// positions in the contract, the key should be the same as the position's id in Osmosis
pub const POSITIONS: Map<u64, Position> = Map::new("positions");
pub const MAIN_POSITION_ID: Item<u64> = Item::new("main_position_id");
pub const CURRENT_POSITION_ID: Item<u64> = Item::new("current_position_id");
pub const CURRENT_CLAIM_AFTER_SECS: Item<Option<u64>> = Item::new("current_claim_after_secs");
pub const SHARES: Map<Addr, Uint128> = Map::new("shares");

/// boolean signifying whether the current executing merge is for the main position
pub const MERGE_MAIN_POSITION: Item<bool> = Item::new("merge_main_position");

/// The queue of positions currently being merged
pub const CURRENT_MERGE: Deque<CurrentMergeWithdraw> = Deque::new("current_merge");

#[cw_serde]
pub struct CurrentMergePosition {
    pub lower_tick: i64,
    pub upper_tick: i64,
    pub claim_after_secs: Option<u64>,
}

pub const CURRENT_MERGE_POSITION: Item<CurrentMergePosition> = Item::new("current_merge_position");

#[cw_serde]
pub struct CurrentDeposit {
    pub token0_in: Uint128,
    pub token1_in: Uint128,
    pub sender: Addr,
}

pub const CURRENT_DEPOSIT: Item<CurrentDeposit> = Item::new("current_deposit");

#[cw_serde]
pub enum RewardsStatus {
    Ready,
    Collecting,
    Distributing,
}

/// REWARDS: Current rewards are the rewards being gathered, these can be both spread rewards as well as incentives
#[deprecated]
pub const STRATEGIST_REWARDS: Item<CoinList> = Item::new("strategist_rewards");

/// Shared collection+distribution states
#[deprecated]
pub const USER_REWARDS: Map<Addr, CoinList> = Map::new("user_rewards");

/// Swap helper states
pub const CURRENT_BALANCE: Item<(Uint128, Uint128)> = Item::new("current_balance"); // CURRENT_BALANCE is intended as CURRENT_SWAP_BALANCE
pub const CURRENT_SWAP: Item<(SwapDirection, Uint128)> = Item::new("current_swap");
pub const CURRENT_SWAP_ANY_DEPOSIT: Item<(SwapDirection, Uint128, Addr, (Uint128, Uint128))> =
    Item::new("current_swap_any_deposit");

/// DEX_ROUTER: The address of the dex router contract
pub const DEX_ROUTER: Item<Addr> = Item::new("dex_router");

#[cw_serde]
pub struct ModifyRangeState {
    // pre-withdraw state items
    pub lower_tick: i64,
    pub upper_tick: i64,
    // the max slippage for modifying the range
    pub max_slippage: Decimal,
    // pre-deposit state items
    pub new_range_position_ids: Vec<u64>,
    // the percent of funds to try for the next swap
    pub ratio_of_swappable_funds_to_use: Decimal,
    // the twap window to use for the swap in seconds
    pub twap_window_seconds: u64,
    // the recommended path to take for the swap
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    // only claim the created position after claim_after amount of seconds
    pub claim_after_secs: u64,
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
