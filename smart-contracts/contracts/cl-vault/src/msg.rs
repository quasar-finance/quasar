use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Decimal256, Uint128};
use cw_vault_multi_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

use crate::{
    query::{FullPositionResponse, PoolResponse, PositionResponse, RangeAdminResponse},
    state::VaultConfig,
};

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionExecuteMsg {
    /// Execute Admin operations.
    Admin(AdminExtensionExecuteMsg),
    /// Rebalance our liquidity range based on an off-chain message
    /// given to us by RANGE_ADMIN
    ModifyRange(ModifyRange),
    /// handle any callback messages of the contract
    CallbackExecuteMsg(CallbackExecuteMsg),
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
    ClaimStrategistRewards {},
}

/// Callback messages
#[cw_serde]
pub enum CallbackExecuteMsg {
    /// distribute any already gathered from the contract state to all share holders
    DistributeRewards(),
    /// provides a fungify callback interface for the contract to use
    Merge(MergePositionMsg),
    /// mint shares after all positions of the user deposit are done
    MintUserDeposit {},
}

/// ModifyRange represents the 3 options we have to change the ranges of the vault, namely moving a current position
/// increasing or decreasing the relative percentage a position has in the vault and creating and deleting a position.
/// Decreasing the percentage of a position to 0 is not allowed. DeletePosition should be used there.
#[cw_serde]
pub enum ModifyRange {
    /// Move the range of a current position
    MovePosition {
        old_position_id: u64,
        /// The new lower bound of the range, this is converted to an 18 precision digit decimal
        new_lower_price: Decimal,
        /// The new upper bound of the range, this is converted to an 18 precision digit decimal
        new_upper_price: Decimal,
        /// max position movement slippage
        max_slippage: Decimal,
    },
    /// Increase the ratio a position has within the total positions of the vault
    AddRatio {
        /// the position_id of the position to change percentage of
        position_id: u64,
        /// The old percentage at which the position was set, this might be different from the actual current percentage
        /// of that position due to IL
        old_ratio: Uint128,
        /// The new percentage to set the position at, Increasing requires free balance in the contract.
        /// Decreasing generates free balance in the contract
        new_ratio: Uint128,
    },
    /// Increase or Decrease which percentage of the vault a range is
    LowerRatio {
        /// the position_id of the position to change percentage of
        position_id: u64,
        /// The old percentage at which the position was set, this might be different from the actual current percentage
        /// of that position due to IL
        old_ratio: Uint128,
        /// The new percentage to set the position at, Increasing requires free balance in the contract.
        /// Decreasing generates free balance in the contract
        new_ratio: Uint128,
    },
    IncreaseFunds {
        position_id: u64,
        token0: Coin,
        token1: Coin,
    },
    DecreaseFunds {
        position_id: u64,
        liquidity: Decimal256,
    },
    /// Create a new position. This consumes all free balance up to max_percentage current free balance
    CreatePosition {
        /// The lower price of the new position
        lower_price: Decimal,
        /// The upper price of the new position
        upper_price: Decimal,
        /// the ratio that this new position can take
        ratio: Uint128,
    },
    DeletePosition {
        /// delete the position under position_id
        position_id: u64,
    },
    /// Rebalance the vaults assets over all positions according to the positions ratios
    Rebalance {},
}

#[cw_serde]
pub struct MergePositionMsg {
    pub position_ids: Vec<u64>,
    pub ratio: Uint128,
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
pub enum UserBalanceQueryMsg {
    UserSharesBalance { user: String },
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
    Positions {},
    #[returns(RangeAdminResponse)]
    RangeAdmin {},
    #[returns(FullPositionResponse)]
    FullPosition {},
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
