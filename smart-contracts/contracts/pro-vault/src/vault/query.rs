use cosmwasm_std::{Deps, StdResult, Binary, to_json_binary};
use crate::vault::config::{Config, VAULT_CONFIG, VaultConfigResponse};
use crate::vault::provault::{VaultRunningState, VAULT_STATE};
use cosmwasm_schema::cw_serde;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use crate::strategy::strategy::{Strategy, STRATEGY};
 
// TODO - Recheck the serialization
#[cw_serde]
pub enum VaultQueryMsg {
    GetVaultConfig {},
    GetVaultRunningState {},
}
 
pub fn query_vault_config(deps: Deps) -> StdResult<Binary> {
    let config = VAULT_CONFIG.load(deps.storage)?;
    to_json_binary(&VaultConfigResponse {
        config,
    })
}


// query to return the running state of the pro Vault
pub fn query_vault_running_state(deps: Deps) -> StdResult<Binary> {
    let vault = VAULT_STATE.load(deps.storage)?;
    to_json_binary(&VaultRunningStateResponse {
        state: vault.state,
        last_statechange_bh: vault.last_statechange_bh,
    })
}
// Ideally it should be zero or one for the initial phase.
// Vec is added for a potential design enhancement for the future.
pub fn query_all_strategies(deps: Deps) -> StdResult<Binary> {
 
    let strategies: StdResult<Vec<Strategy>> = STRATEGY
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.map(|(_, strategy)| strategy))
        .collect();
        to_json_binary(&StrategyInfoResponse {
        strategies: strategies?,
    })
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultRunningStateResponse {
    pub state: VaultRunningState,
    pub last_statechange_bh: u64,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StrategyInfoResponse {
    pub strategies: Vec<Strategy>,
}
