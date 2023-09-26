use cosmwasm_std::{DepsMut, Response};

use crate::state::{AIRDROP_CONFIG, USER_INFO};
use crate::AirdropErrors;

pub fn handle_reply(deps: DepsMut, user: String) -> Result<Response, AirdropErrors> {
    let mut user_info = USER_INFO.load(deps.storage, user.clone())?;
    // update the user info as claimed
    user_info.claimed_flag = true;
    USER_INFO.save(deps.storage, user, &user_info)?;

    // update airdrop config by increasing the claimed amount
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
