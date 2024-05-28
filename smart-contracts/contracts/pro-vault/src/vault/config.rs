use crate::msg::InstantiateMsg;
use cosmwasm_schema::cw_serde;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::Item;
use cosmwasm_std::{Deps, StdResult, Binary, to_json_binary};
use cosmwasm_std::{DepsMut, Uint128, Uint64};

pub const VAULT_CONFIG: Item<Config> = Item::new("vault_config");

// use crate::msg::VaultConfigResponse;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultConfigResponse {
    pub config: Config,
}
/* 
pub fn query_vault_config(deps: Deps) -> StdResult<Binary> {
    let config = VAULT_CONFIG.load(deps.storage)?;
    to_json_binary(&VaultConfigResponse {
        config,
    })
}
*/

// Pro vault level config parameters. 
// Config will be created during the initialisation.
// #[cw_serde] // TODO 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub max_deposit_cap: Uint128,
    pub deposit_denom: String,
    pub share_denom: String,
    pub max_strategy_inst: Uint64,
    pub admin: String,
}



