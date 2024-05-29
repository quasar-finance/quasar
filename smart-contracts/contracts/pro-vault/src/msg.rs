use cosmwasm_schema::cw_serde;
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use cosmwasm_std::Uint128;
use crate::vault::provault::{VaultRunningState, VaultAction};
use crate::vault::config::Config;
use crate::strategy::strategy::{Strategy, StrategyKey, StrategyAction}; 
use serde::{Serialize, Deserialize};
use crate::vault::query::VaultQueryMsg; 

use schemars::JsonSchema;

// Pro vault instantiate message structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub thesis: String,      // The general thesis of the vault
    pub name: String,        // The name of the vault
    pub provault_config: Config, // Config parameters for the vault
}


// Pro vault query message enums types.
#[cw_serde]
pub enum QueryMsg {
    GetAllStrategies {},
    VaultQuery(VaultQueryMsg), // Use VaultQueryMsg for vault-related queries
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ProExtensionExecuteMsg {
    MyVariant1 {
        amount: Uint128,
        recipient: Option<String>,
    },
    /*
    UpdateRunningState {
        new_state: VaultRunningState,
    },
    UpdateVaultOwner {},
    UpdateStrategyOwner {},
    CreateStrategy {
        name: String,
        description: String,
    },
    */
    ExecVaultActions {
        action: VaultAction,
    }, 
    ExecStrategyActions {
        action: StrategyAction,
    },
}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    ProExtension(ProExtensionExecuteMsg),
}

/// ExecuteMsg
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;


 
