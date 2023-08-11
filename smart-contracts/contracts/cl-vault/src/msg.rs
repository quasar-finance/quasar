use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw_vault_multi_standard::{
    extensions::lockup::{LockupExecuteMsg, LockupQueryMsg},
    VaultStandardExecuteMsg, VaultStandardQueryMsg,
};

use crate::{query::PoolResponse, state::VaultConfig};

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionExecuteMsg {
    /// Execute Admin operations.
    Admin(AdminExtensionExecuteMsg),
    /// Rebalance our liquidity range based on an off-chain message
    /// given to us by RANGE_ADMIN
    ModifyRange(ModifyRangeMsg),
    /// Execute a message from the lockup extension.
    Lockup(LockupExecuteMsg),
    /// Execute a message from the force unlock extension.
    #[cfg(feature = "force-unlock")]
    ForceUnlock(ForceUnlockExecuteMsg),
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
    /// Update the configuration of the vault.
    UpdateConfig {
        /// The config updates.
        updates: VaultConfig,
    },
}

#[cw_serde]
pub struct ModifyRangeMsg {
    /// The new lower bound of the range
    pub lower_price: Uint128,
    /// The new upper bound of the range
    pub upper_price: Uint128,
}

/// Extension query messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionQueryMsg {
    /// Queries related to the lockup extension.
    Lockup(LockupQueryMsg),
    /// Queries related to Concentrated Liquidity
    ConcentratedLiquidity(ClQueryMsg),
}

///
#[cw_serde]
#[derive(QueryResponses)]
pub enum ClQueryMsg {
    /// Get the underlying pool of the vault
    #[returns(PoolResponse)]
    Pool {},
}

/// ExecuteMsg for an Autocompounding Vault.
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

/// QueryMsg for an Autocompounding Vault.
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

#[cw_serde]
pub struct InstantiateMsg {
    /// Address that is allowed to update config.
    pub admin: String,
    /// Address that is allowed to update range.
    pub range_admin: String,
    /// The ID of the pool that this vault will autocompound.
    pub pool_id: u64,
    /// The lockup duration in seconds that this vault will use when staking
    /// LP tokens.
    pub lockup_duration: u64,
    /// Configurable parameters for the contract.
    pub config: VaultConfig,
    /// The subdenom that will be used for the native vault token, e.g.
    /// the denom of the vault token will be:
    /// "factory/{vault_contract}/{vault_token_subdenom}".
    pub vault_token_subdenom: String,
}

#[cw_serde]
pub struct MigrateMsg {}
