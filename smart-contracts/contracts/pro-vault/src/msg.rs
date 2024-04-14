use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use cw_dex_router::operations::SwapOperationsListUnchecked;
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use vaultenator::msg::{VaultenatorExtensionExecuteMsg, VaultenatorExtensionQueryMsg};

use crate::{
    adapters::generic_vault::{VaultAction, VaultAdapters},
    query::{
        AssetsBalanceResponse, PositionResponse, RangeAdminResponse, UserRewardsResponse,
        UserSharesBalanceResponse, VerifyTickCacheResponse,
    },
    state::VaultConfig,
};

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionExecuteMsg {
    Vaultenator(VaultenatorExtensionExecuteMsg),
    Adapter(),
}

#[cw_serde]
pub enum AdapterExtensionMsg {
    Vault {
        address: String,
        action: VaultAction,
    },
    Debt(),
    Swap(),
}

/// Extension query messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionQueryMsg {
    Vaultenator(VaultenatorExtensionQueryMsg),
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
    pub strategist: String,
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
