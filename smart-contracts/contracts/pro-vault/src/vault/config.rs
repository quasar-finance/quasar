use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::Item;
use cosmwasm_std::{Uint128, Uint64};

pub const VAULT_CONFIG: Item<Config> = Item::new("vault_config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultConfigResponse {
    pub config: Config,
}
 

// Pro vault level config parameters. 
// Config will be created during the initialisation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub max_deposit_cap: Uint128,
    pub deposit_denom: String,
    pub share_denom: String,
    pub max_strategy_inst: Uint64,
    pub admin: String,
}



