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
    // Whitelist denoms
    pub whitelisted_denoms: Vec<String>
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

/// Execute Msg
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;


/////// QUERY ////////

// Pro vault extension query messages
#[cw_serde]
pub enum ProExtensionQueryMsg {
    Metadata {},
    GetAllStrategies {},
    // Use VaultQueryMsg for vault-related queries
    VaultQuery(VaultQueryMsg), 
}

// Extending the vault standard query message
#[cw_serde]
pub enum ExtensionQueryMsg {
    ProExtension(ProExtensionQueryMsg),
}

/// Query Msg
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;
