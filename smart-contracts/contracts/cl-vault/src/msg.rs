use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, Env, StdResult, Uint128, WasmMsg};
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

use crate::state::VaultConfig;

/// Extension execute messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionExecuteMsg {
    /// Execute a callback message.
    Callback(CallbackMsg),
    /// Execute a an Apollo vault specific message.
    Admin(AdminExtensionExecuteMsg),
    /// Execute a message from the lockup extension.
    #[cfg(feature = "lockup")]
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

/// Apollo extension queries define functionality that is part of all apollo
/// vaults, but not part of the standard.
#[cw_serde]
pub enum ApolloExtensionQueryMsg {
    /// Query the current state of the vault.
    State {},
}

/// Extension query messages for an apollo autocompounding vault
#[cw_serde]
pub enum ExtensionQueryMsg {
    /// Queries related to the lockup extension.
    #[cfg(feature = "lockup")]
    Lockup(LockupQueryMsg),
    /// Apollo extension queries.
    Apollo(ApolloExtensionQueryMsg),
}

/// Callback messages for the autocompounding vault `Callback` extension
#[cw_serde]
pub enum CallbackMsg {
    /// Sell all the rewards in the contract to the underlying tokens of the
    /// pool.
    SellRewards {},
    /// Provide liquidity with all the underlying tokens of the pool currently
    /// in the contract.
    ProvideLiquidity {},
    /// Stake all base tokens in the contract.
    Stake {
        /// Contract base token balance before this transaction started. E.g. if
        /// funds were sent to the contract as part of the `info.funds` or
        /// received as cw20s in a previous message they must be deducted from
        /// the current contract balance.
        base_token_balance_before: Uint128,
    },
    /// Mint vault tokens
    MintVaultToken {
        /// The amount of base tokens to deposit.
        amount: Uint128,
        /// The recipient of the vault token.
        recipient: Addr,
    },
    /// Redeem vault tokens for base tokens.
    #[cfg(feature = "redeem")]
    Redeem {
        /// The address which should receive the withdrawn base tokens.
        recipient: Addr,
        /// The amount of vault tokens sent to the contract. In the case that
        /// the vault token is a Cosmos native denom, we of course have this
        /// information in the info.funds, but if the vault implements the
        /// Cw4626 API, then we need this argument. We figured it's
        /// better to have one API for both types of vaults, so we
        /// require this argument.
        amount: Uint128,
    },
    /// Burn vault tokens and start the unlocking process.
    #[cfg(feature = "lockup")]
    Unlock {
        /// The address that will be the owner of the unlocking position.
        owner: Addr,
        /// The amount of vault tokens to burn.
        vault_token_amount: Uint128,
    },
    /// Save the currently pending claim to the `claims` storage.
    #[cfg(feature = "lockup")]
    SaveClaim {},
}

impl CallbackMsg {
    /// Convert the callback message to a [`CosmosMsg`]. The message will be
    /// formatted as a `Callback` extension in a [`VaultStandardExecuteMsg`],
    /// accordning to the
    /// [CosmWasm Vault Standard](https://docs.rs/cosmwasm-vault-standard/0.1.0/cosmwasm_vault_standard/#how-to-use-extensions).
    pub fn into_cosmos_msg(&self, env: &Env) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&VaultStandardExecuteMsg::VaultExtension(
                ExtensionExecuteMsg::Callback(self.clone()),
            ))?,
            funds: vec![],
        }))
    }
}

/// ExecuteMsg for an Autocompounding Vault.
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

/// QueryMsg for an Autocompounding Vault.
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

#[cw_serde]
pub struct InstantiateMsg {
    /// Address that is allowed to update config.
    pub admin: String,
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
