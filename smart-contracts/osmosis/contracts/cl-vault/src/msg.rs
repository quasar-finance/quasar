use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use quasar_types::cw_vault_multi_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

#[cfg(not(target_arch = "wasm32"))]
use crate::query::{
    AssetsBalanceResponse, PoolResponse, PositionResponse, RangeAdminResponse,
    UserSharesBalanceResponse, VerifyTickCacheResponse,
};
use crate::state::{Metadata, VaultConfig};

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionExecuteMsg {
    /// Execute Admin operations.
    Admin(AdminExtensionExecuteMsg),
    /// Rebalance our liquidity range based on an off-chain message
    /// given to us by RANGE_ADMIN
    ModifyRange(ModifyRangeMsg),
    /// provides a fungify callback interface for the contract to use
    Merge(MergePositionMsg),
    /// provides an entry point for autocompounding idle funds to current position
    Autocompound {},
    /// Distribute any rewards over all users
    CollectRewards {},
    /// SwapNonVaultFunds
    SwapNonVaultFunds {
        swap_operations: Vec<SwapOperation>,
        twap_window_seconds: Option<u64>,
    },
}

/// Apollo extension messages define functionality that is part of all apollo
/// vaults, but not part of the standard.
#[cw_serde]
pub enum AdminExtensionExecuteMsg {
    /// Update the vault admin.
    UpdateAdmin {
        /// The new admin address.
        address: String,
    },
    /// Update the range adming,
    UpdateRangeAdmin {
        /// the new range admin
        address: String,
    },
    /// Update the configuration of the vault.
    UpdateConfig {
        /// The config updates.
        updates: VaultConfig,
    },
    UpdateMetadata {
        /// The metadata updates.
        updates: Metadata,
    },
    /// Build tick exponent cache
    BuildTickCache {},
    /// Auto claim endpoint
    AutoWithdraw { users: Vec<(String, Uint128)> },
}

#[cw_serde]
pub struct ModifyRangeMsg {
    /// The new lower bound of the range, this is converted to an 18 precision digit decimal
    pub lower_price: Decimal,
    /// The new upper bound of the range, this is converted to an 18 precision digit decimal
    pub upper_price: Decimal,
    /// max position slippage
    pub max_slippage: Decimal,
    /// desired percent of funds to use during the swap step
    pub ratio_of_swappable_funds_to_use: Decimal,
    /// twap window to use in seconds
    pub twap_window_seconds: u64,
    /// forced swap route to take
    pub forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    /// claim_after optional field, if we off chain computed that incentives have some forfeit duration. this will be persisted in POSITION state
    pub claim_after: Option<u64>,
}

#[cw_serde]
pub struct MergePositionMsg {
    pub position_ids: Vec<u64>,
}

// struct used by swap.rs on swap non vault funds
#[cw_serde]
pub struct SwapOperation {
    pub token_in_denom: String,
    pub pool_id_base: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub pool_id_quote: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub forced_swap_route_base: Option<Vec<SwapAmountInRoute>>,
    pub forced_swap_route_quote: Option<Vec<SwapAmountInRoute>>,
}

/// Extension query messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionQueryMsg {
    /// Metadata surrounding the vault
    Metadata {},
    /// Queries related to the lockup extension.
    Balances(UserBalanceQueryMsg),
    /// Queries related to Concentrated Liquidity
    ConcentratedLiquidity(ClQueryMsg),
    /// Query the DexRouter address
    DexRouter {},
    /// Query users
    Users {
        start_bound_exclusive: Option<String>,
        /// Limit for the search
        limit: u64,
    },
}

/// Extension query messages for user balance related queries
#[cw_serde]
#[derive(QueryResponses)]
pub enum UserBalanceQueryMsg {
    #[returns(UserSharesBalanceResponse)]
    UserSharesBalance { user: String },
    #[returns(AssetsBalanceResponse)]
    UserAssetsBalance { user: String },
}

/// Extension query messages for related concentrated liquidity
#[cw_serde]
#[derive(QueryResponses)]
pub enum ClQueryMsg {
    /// Get the underlying pool of the vault
    #[returns(PoolResponse)]
    Pool {},
    #[returns(PositionResponse)]
    Position {},
    #[returns(RangeAdminResponse)]
    RangeAdmin {},
    #[returns(VerifyTickCacheResponse)]
    VerifyTickCache,
}

/// ExecuteMsg for an Autocompounding Vault.
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

/// QueryMsg for an Autocompounding Vault.
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

#[cw_serde]
pub struct InstantiateMsg {
    /// The general thesis of the vault
    pub thesis: String,
    /// the name of the vault
    pub name: String,
    /// Address that is allowed to update config.
    pub admin: String,
    /// Address that is allowed to update range.
    pub range_admin: String,
    /// The ID of the pool that this vault will autocompound.
    pub pool_id: u64,
    /// Configurable parameters for the contract.
    pub config: VaultConfig,
    /// The subdenom that will be used for the native vault token, e.g.
    /// the denom of the vault token will be:
    /// "factory/{vault_contract}/{vault_token_subdenom}".
    pub vault_token_subdenom: String,
    // create a position upon initialization
    pub initial_lower_tick: i64,
    pub initial_upper_tick: i64,
}

#[cw_serde]
pub struct MigrateMsg {}
