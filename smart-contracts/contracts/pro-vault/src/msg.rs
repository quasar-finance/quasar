use cosmwasm_schema::cw_serde;
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use crate::vault::provault::VaultAction;
use crate::vault::config::Config;
use crate::strategy::strategy::StrategyAction; 
use serde::{Serialize, Deserialize};
use crate::vault::query::VaultQueryMsg; 

use schemars::JsonSchema;

// Pro vault instantiate message structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // The general thesis of the vault
    pub thesis: String,      
    // The name of the vault
    pub name: String, 
    // Config parameters for the vault       
    pub provault_config: Config, 
}


// Pro vault query message enums types.
// TODO - Extending the Vault standard query message
#[cw_serde]
pub enum QueryMsg {
    GetAllStrategies {},
    VaultQuery(VaultQueryMsg), // Use VaultQueryMsg for vault-related queries
}

#[cw_serde]
pub struct MigrateMsg {}


// Pro vault extension execute messages 
#[cw_serde]
pub enum ProExtensionExecuteMsg {
    ExecVaultActions {
        action: VaultAction,
    }, 
    ExecStrategyActions {
        action: StrategyAction,
    },
}

// Extending the vault standard execute message
#[cw_serde]
pub enum ExtensionExecuteMsg {
    ProExtension(ProExtensionExecuteMsg),
}

/// ExecuteMsg
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;


 
