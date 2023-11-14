use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use cw_vault_multi_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

use crate::{
    query::{
        AssetsBalanceResponse, PoolResponse, PositionResponse, RangeAdminResponse,
        UserRewardsResponse, UserSharesBalanceResponse,
    },
    state::{Metadata, VaultConfig},
};

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
    /// Distribute any rewards over all users
    DistributeRewards {},
    /// Claim rewards belonging to a single user
    ClaimRewards {},
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
    ClaimStrategistRewards {},
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
}

#[cw_serde]
pub struct MergePositionMsg {
    pub position_ids: Vec<u64>,
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
}

/// Extension query messages for user balance related queries
#[cw_serde]
#[derive(QueryResponses)]
pub enum UserBalanceQueryMsg {
    #[returns(UserSharesBalanceResponse)]
    UserSharesBalance { user: String },
    #[returns(AssetsBalanceResponse)]
    UserAssetsBalance { user: String },
    #[returns(UserRewardsResponse)]
    UserRewards { user: String },
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
