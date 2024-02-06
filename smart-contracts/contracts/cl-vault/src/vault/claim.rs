use std::fmt;

use cosmwasm_std::{Addr, Coin, DepsMut, Env, Response, Uint128};

use crate::{helpers::sort_tokens, state::USER_REWARDS, ContractError};

pub fn execute_claim_user_rewards(
    deps: DepsMut,
    recipient: &str,
) -> Result<Response, ContractError> {
    // addr unchecked is safe here because we will chekc addresses on save into this map
    let mut user_rewards =
        match USER_REWARDS.may_load(deps.storage, deps.api.addr_validate(recipient)?)? {
            Some(user_rewards) => user_rewards,
            None => {
                return Ok(Response::default()
                    .add_attribute("action", "claim_user_rewards")
                    .add_attribute("result", "no_rewards"))
            }
        };

    let send_rewards_msg = user_rewards.claim(recipient)?;

    // todo: check if user rewards are claimed correctly
    USER_REWARDS.save(deps.storage, Addr::unchecked(recipient), &user_rewards)?;

    Ok(Response::new()
        .add_message(send_rewards_msg)
        .add_attribute("action", "claim_user_rewards")
        .add_attribute("result", "success")
        .add_attribute("recipient", recipient)
        .add_attributes(user_rewards.into_attributes()))
}
