use cosmwasm_schema::cw_serde;
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use cosmwasm_std::Uint128;
// use crate::vault::{provault, config};
use crate::vault::provault::VaultRunningState;
use crate::vault::config::Config;
use crate::strategy::strategy::{Strategy, StrategyKey};
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;

// TODO - Complex instantiation support to be added so provault can be fully orchastreted 
// using an elegant json file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub thesis: String,      // The general thesis of the vault
    pub name: String,        // The name of the vault
    pub provault_config: Config, // Config parameters for the vault
}


#[cw_serde]
pub enum QueryMsg {
    GetAllStrategies {},
    GetVaultConfig {},    
    GetVaultRunningState {},

}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ProExtensionExecuteMsg {
    MyVariant1 {
        /// The amount of base tokens to deposit.
        amount: Uint128,
        /// The optional recipient of the vault token. If not set, the caller
        /// address will be used instead.
        recipient: Option<String>,
    },

    // TODO - Combine all vault related enums as Vault Actions or VaultAdmin Action.
    UpdateRunningState {
        new_state: VaultRunningState,
        // Placeholder for running state details
    },

    UpdateVaultOwner {
        // Placeholder for vault owner details
    },


    UpdateStrategyOwner {
        // Placeholder for strategy owner details
    },

    // TODO - Adding adaptors, configuring adaptors, adding Strategy Control Owner, Adaptor Control Owner
    CreateStrategy { name: String, description: String },  



    ExecStrategyActions { 
        action : StrategyAction,
    },
}

#[cw_serde]
pub enum StrategyAction {
    DistributeFundWithPresetAdaptorRatio, // Distributing funds across adaptors as per preset ratios
    DistributeFundWithCustomAdaptorRatios { custom_ratios: String }, // CustomAdaptorRatio (A1:R1, A2:R2, A3:R3)
    RemoveAdaptor { adaptor: String }, // Remove Adaptor Ai
    AddNewAdaptor { adaptor: String }, // Add a new adaptor of type Ai. Should fail if already one is present of type A1.
    UpdateStrategyParams ,
    //{ // Placeholder for updating strategy parameters // e.g., update ratio, remove adaptor, enable/disable strategy or adaptor
    //},
    UpdateAdaptorRunningState { adaptor: String },
    UpdateStrategyRunningState,
}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    ProExtension(ProExtensionExecuteMsg),
}

/// ExecuteMsg
pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;


// QUERY RESPONSES 
// TODO - Structure to code to the right module and files .
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StrategyInfoResponse {
    pub strategies: Vec<Strategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultConfigResponse {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultRunningStateResponse {
    pub state: VaultRunningState,
    pub last_statechange_bh: u64,
}
