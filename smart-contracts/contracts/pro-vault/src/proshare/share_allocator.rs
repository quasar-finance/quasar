use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdResult, Attribute};
use crate::error::ContractError;
use cw_storage_plus::Map;

// Define the map for allocated shares
pub const ALLOCATED_SHARES: Map<&str, Uint128> = Map::new("allocated_shares");

#[derive(Clone, Debug, PartialEq)]
pub enum ShareType {
    Number,
    Cw20Token,
    TokenFactory,
}

pub struct ShareConfig {
    pub share_type: ShareType,
    pub share_denom: String, // TODO - Only a representation during test phases for Number type.
}

impl ShareConfig {
    // TODO - The new method share_denom to be enhanced to reflect real scenario 
    // Currently it is only for representations during intial testing phase.
    pub fn new(share_type: ShareType, deposit_denom: &str) -> Self {
        Self {
            share_type,
            share_denom: format!("pro{}", deposit_denom),
        }
    }
}

// TODO - This is a dummy implementation now till we have oracle in place.
// Assuming that provault will accept only one denom type and proshare will be equal 
// to the denom in value. Right now we are only returning a number as shares in equal 
// amount as deposited amount.
pub fn calculate_shares(amount: Uint128, config: &ShareConfig) -> Uint128 {
    match config.share_type {
        ShareType::Number => amount, // 1:1 ratio for simplicity of initial testing.
        ShareType::Cw20Token => amount, // This will be updated when CW20 token logic is added ( Less likely)
        ShareType::TokenFactory => amount, // This will be updated when TokenFactory logic is added ( Most likely)
    }
}


pub fn allocate_shares(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    amount: Uint128,
    config: &ShareConfig,
) -> Result<Response, ContractError> {
    let shares = calculate_shares(amount, config);
    let recipient_address = recipient.unwrap_or(info.sender.to_string());
    
    // For now during intial testing, attribute to the response indicating shares is added.
    // In a real implementation, we will mint TokenFactory logic here.

    // TODO - Actual share minting using token factory. 
    // And allocating those shares to info.sender or recipient.

    // Update the map with allocated shares
    let current_shares = ALLOCATED_SHARES.may_load(deps.storage, &recipient_address)?.unwrap_or_default();
    let new_shares = current_shares + shares;
    ALLOCATED_SHARES.save(deps.storage, &recipient_address, &new_shares)?;
    
  
    Ok(Response::new()
        .add_attribute("method", "allocate_shares")
        .add_attribute("amount", amount.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("recipient", recipient_address))
}
