use cosmwasm_std::{DepsMut, Response};

use crate::state::{AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

/// Handles a reply from a user indicating that they have successfully claimed their airdrop.
///
/// # Arguments
///
/// * `deps` - DepsMut is a struct providing mutable access to the contract's dependencies like storage.
/// * `user` - The address of the user who claimed the airdrop.
///
/// # Returns
///
/// Returns a `Response` indicating the success of the claim operation and updated airdrop statistics.
pub fn handle_reply(deps: DepsMut, user: String) -> Result<Response, AirdropErrors> {
    // Load the user's information from storage
    let mut user_info = USER_INFO.load(deps.storage, user.clone())?;

    // Mark the user as claimed
    user_info.claimed_flag = true;
    USER_INFO.save(deps.storage, user, &user_info)?;

    // Update the airdrop configuration by increasing the total claimed amount
    let mut airdrop_config = AIRDROP_CONFIG.load(deps.storage)?;
    airdrop_config.total_claimed += user_info.claimable_amount;
    AIRDROP_CONFIG.save(deps.storage, &airdrop_config)?;

    Ok(Response::default()
        .add_attribute(
            "user_claimed_amount",
            user_info.claimable_amount.to_string(),
        )
        .add_attribute(
            "total_claimed_amount",
            airdrop_config.total_claimed.to_string(),
        ))
}
