use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::Item;
use cosmwasm_std::{Uint128, Uint64, DepsMut, Env, MessageInfo, StdResult, StdError, Response};
use crate::error::ContractError::Unauthorized;
use crate::error::ContractError;
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
    pub deposit_denom: String, // Single denom deposit for now.
    pub share_denom: Option<String>,
    pub max_strategy_inst: Uint64,
    pub admin: String,
}

pub fn update_max_deposit_cap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_cap: Uint128,
) -> Result<Response, ContractError>  {

    let mut config: Config = VAULT_CONFIG.load(deps.storage)?;
    
    // TODO - Could be checked against VAULT_OWNER
    if info.sender != config.admin {
        return Err(Unauthorized {});
    }

    config.max_deposit_cap = new_cap;
    VAULT_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_max_deposit_cap")
        .add_attribute("new_cap", new_cap.to_string())
        .add_attribute("admin", info.sender.to_string()))
}


