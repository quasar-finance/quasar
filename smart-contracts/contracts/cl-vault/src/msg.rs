use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Decimal256, Uint128};
use cw_dex_router::operations::SwapOperationsListUnchecked;
use cw_vault_multi_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    query::{
        AssetsBalanceResponse, MainPositionResponse, PoolResponse, PositionsResponse,
        RangeAdminResponse, UserSharesBalanceResponse, VerifyTickCacheResponse,
    },
    state::{Metadata, VaultConfig},
};

/// ExecuteMsg for an Autocompounding Vault.
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

impl From<ExtensionExecuteMsg> for ExecuteMsg {
    fn from(msg: ExtensionExecuteMsg) -> Self {
        ExecuteMsg::VaultExtension(msg)
    }
}

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExtensionExecuteMsg {
    /// Execute Admin operations.
    Admin(AdminExtensionExecuteMsg),
    /// An interface of certain vault interaction with forced values for authz
    Authz(AuthzExtension),
    /// Rebalance our liquidity range based on an off-chain message
    /// given to us by RANGE_ADMIN
    ModifyRange(ModifyRange),
    /// provides a fungify callback interface for the contract to use
    Merge(MergePositionMsg),
    /// provides an entry point for autocompounding idle funds to current position
    Autocompound {},
    /// Distribute any rewards over all users
    CollectRewards {},
    /// MigrationStep
    MigrationStep { amount_of_users: Uint128 },
    /// SwapNonVaultFunds
    SwapNonVaultFunds { swap_operations: Vec<SwapOperation> },
}

/// Extension messages for Authz. This interface basically reexports certain vault functionality
/// but sets recipient forcibly to None
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AuthzExtension {
    ExactDeposit {},
    AnyDeposit { max_slippage: Decimal },
    Redeem { amount: Uint128 },
}

impl From<AuthzExtension> for ExtensionExecuteMsg {
    fn from(msg: AuthzExtension) -> Self {
        ExtensionExecuteMsg::Authz(msg)
    }
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
    /// Update the dex router address.
    UpdateDexRouter {
        /// The new dex router address.
        address: Option<String>,
    },
    /// Build tick exponent cache
    BuildTickCache {},
}

impl From<AdminExtensionExecuteMsg> for ExtensionExecuteMsg {
    fn from(msg: AdminExtensionExecuteMsg) -> Self {
        ExtensionExecuteMsg::Admin(msg)
    }
}

/// ModifyRange represents the 3 options we have to change the ranges of the vault, namely moving a current position
/// increasing or decreasing the relative percentage a position has in the vault and creating and deleting a position.
/// Decreasing the percentage of a position to 0 is not allowed. DeletePosition should be used there.
#[cw_serde]
pub enum ModifyRange {
    /// Move the range of a current position
    MovePosition(MovePosition),
    IncreaseFunds(IncreaseFunds),
    DecreaseFunds(DecreaseFunds),
    /// Create a new position. This consumes all free balance up to max_percentage current free balance
    CreatePosition(CreatePosition),
    DeletePosition(DeletePosition),
}

#[cw_serde]
pub struct IncreaseFunds {
    pub position_id: u64,
    pub token0: Coin,
    pub token1: Coin,
}

#[cw_serde]
pub struct DecreaseFunds {
    pub position_id: u64,
    pub liquidity: Decimal256,
}

#[cw_serde]
pub struct CreatePosition {
    /// The lower price of the new position
    pub lower_price: Decimal,
    /// The upper price of the new position
    pub upper_price: Decimal,
    /// claim_after optional field, if we off chain computed that incentives have some forfeit duration. this will be persisted in POSITION state
    pub claim_after: Option<u64>,
}

#[cw_serde]
pub struct DeletePosition {
    /// delete the position under position_id
    pub position_id: u64,
}

#[cw_serde]
pub struct MovePosition {
    /// the id of the position to move
    pub position_id: u64,
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
    pub main_position: bool,
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

impl From<UserBalanceQueryMsg> for ExtensionQueryMsg {
    fn from(msg: UserBalanceQueryMsg) -> Self {
        ExtensionQueryMsg::Balances(msg)
    }
}

/// Extension query messages for related concentrated liquidity
#[cw_serde]
#[derive(QueryResponses)]
pub enum ClQueryMsg {
    /// Get the underlying pool of the vault
    #[returns(PoolResponse)]
    Pool {},
    #[returns(PositionsResponse)]
    Positions {},
    #[returns(RangeAdminResponse)]
    RangeAdmin {},
    #[returns(VerifyTickCacheResponse)]
    VerifyTickCache,
    #[returns(MainPositionResponse)]
    MainPosition,
}

impl From<ClQueryMsg> for ExtensionQueryMsg {
    fn from(msg: ClQueryMsg) -> Self {
        ExtensionQueryMsg::ConcentratedLiquidity(msg)
    }
}

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
pub struct MigrateMsg {
    pub dex_router: Addr,
}
