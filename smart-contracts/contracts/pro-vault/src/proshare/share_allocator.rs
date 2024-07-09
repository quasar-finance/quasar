use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdResult, Attribute};
use crate::error::ContractError;
use cw_storage_plus::{Map, Item};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use cosmwasm_schema::cw_serde;

// Map to store allocated shares per user.
pub const ALLOCATED_SHARES: Map<&str, Uint128> = Map::new("allocated_shares");
pub const TOTAL_SHARES: Item<Uint128> = Item::new("total_shares");
pub const TOTAL_ASSETS: Item<Uint128> = Item::new("total_assets");

// #[cw_serde]
// TODO - Recheck serde serialization for the enum.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ShareType {
    Number,
    Cw20Token,
    TokenFactory,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
 

// Function to calculate the number of shares based on the amount deposited.
pub fn calculate_shares(amount: Uint128, total_shares: Uint128, total_assets: Uint128) -> Uint128 {
    if total_shares.is_zero() {
        amount
    } else {
        amount * total_shares / total_assets
    }
}

// TODO - Below codes are reduntant, added for some testing scenario for now.
pub fn allocate_shares(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
    amount: Uint128,
    config: &ShareConfig,
) -> Result<Response, ContractError> {
    let recipient_address = recipient.unwrap_or(info.sender.to_string());
    
    // Load total shares and total assets from storage.
    let total_shares = TOTAL_SHARES.load(deps.storage).unwrap_or(Uint128::zero());
    let total_assets = TOTAL_ASSETS.load(deps.storage).unwrap_or(Uint128::zero());
    
    // Calculate the new shares to be allocated.
    let shares = calculate_shares(amount, total_shares, total_assets);
    
    // Update the allocated shares for the recipient.
    let current_shares = ALLOCATED_SHARES.may_load(deps.storage, &recipient_address)?.unwrap_or_default();
    let new_shares = current_shares + shares;
    ALLOCATED_SHARES.save(deps.storage, &recipient_address, &new_shares)?;
    
    // Update the total shares and total assets in the vault.
    let updated_total_shares = total_shares + shares;
    let updated_total_assets = total_assets + amount;
    TOTAL_SHARES.save(deps.storage, &updated_total_shares)?;
    TOTAL_ASSETS.save(deps.storage, &updated_total_assets)?;
    
    Ok(Response::new()
        .add_attribute("method", "allocate_shares")
        .add_attribute("amount", amount.to_string())
        .add_attribute("shares", shares.to_string())
        .add_attribute("recipient", recipient_address)
        .add_attribute("total_shares", updated_total_shares.to_string())
        .add_attribute("total_assets", updated_total_assets.to_string()))
}


// Function to withdraw shares and corresponding assets for a user.
pub fn withdraw_shares(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    withdraw_share_amount: Uint128,
    config: &ShareConfig,
) -> Result<Response, ContractError> {
    let sender_address = info.sender.to_string();
    
    // Load total shares and total assets from storage.
    let total_shares = TOTAL_SHARES.load(deps.storage).unwrap_or(Uint128::zero());
    let total_assets = TOTAL_ASSETS.load(deps.storage).unwrap_or(Uint128::zero());
    
    // Check for zero total shares to prevent division by zero.
    if total_shares.is_zero() {
        return Err(ContractError::InsufficientShares {});
    }

    // Calculate the amount of assets to withdraw based on the shares.
    let withdraw_amount = withdraw_share_amount * total_assets / total_shares;
    
    // Update the allocated shares for the sender.
    let current_shares = ALLOCATED_SHARES.load(deps.storage, &sender_address)?;
    if current_shares < withdraw_share_amount {
        return Err(ContractError::InsufficientShares {});
    }
    let remaining_user_shares = current_shares - withdraw_share_amount;
    ALLOCATED_SHARES.save(deps.storage, &sender_address, &remaining_user_shares)?;
    
    // Update the total shares and total assets in the vault.
    let remaining_total_shares = total_shares - withdraw_share_amount;
    let remaining_total_assets = total_assets - withdraw_amount;
    TOTAL_SHARES.save(deps.storage, &remaining_total_shares)?;
    TOTAL_ASSETS.save(deps.storage, &remaining_total_assets)?;
    
    Ok(Response::new()
        .add_attribute("method", "withdraw_shares")
        .add_attribute("share_amount", withdraw_share_amount.to_string())
        .add_attribute("withdraw_amount", withdraw_amount.to_string())
        .add_attribute("sender", sender_address)
        .add_attribute("total_shares", remaining_total_shares.to_string())
        .add_attribute("total_assets", remaining_total_assets.to_string()))
}
