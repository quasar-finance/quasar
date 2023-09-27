use cosmwasm_std::{Addr, DepsMut, Env, Response};
use cw_asset::Asset;

use crate::helpers::add_reply;
use crate::state::{AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

// Define a function to process airdrop claims for a user
pub fn execute_claim(deps: DepsMut, env: Env, user: Addr) -> Result<Response, AirdropErrors> {
    // Load the current airdrop configuration from storage
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;

    // Check if the airdrop window is open and the user is eligible to claim
    if current_airdrop_config.start_height == 0
        || current_airdrop_config.end_height == 0
        || env.block.height > current_airdrop_config.end_height
        || env.block.height < current_airdrop_config.start_height
    {
        return Err(AirdropErrors::InvalidClaim {});
    }

    // Load the user's airdrop information from storage
    let user_info = USER_INFO.load(deps.storage, user.to_string())?;

    // Check if the user has already claimed the airdrop
    if user_info.get_claimed_flag() {
        return Err(AirdropErrors::AlreadyClaimed {});
    }

    // Get the admin address of the contract
    let current_airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;
    let contract_balance = current_airdrop_config
        .airdrop_asset
        .query_balance(&deps.querier, &env.contract.address)?;

    // Check if the user's claimable amount exceeds the contract's balance
    if user_info.get_claimable_amount() > contract_balance {
        return Err(AirdropErrors::InsufficientFundsInContractAccount {});
    }

    // Transfer the airdrop asset to the withdrawal address
    let claim = Asset::new(
        current_airdrop_config.airdrop_asset,
        user_info.claimable_amount,
    )
    .transfer_msg(user.clone())?;

    // Return a default response if all checks pass
    Ok(Response::new()
        .add_submessage(add_reply(deps.storage, claim, user.clone())?)
        .add_attributes(vec![
            ("action", "claim"),
            ("user", user.as_ref()),
            ("amount", &user_info.claimable_amount.to_string()),
        ]))
}
