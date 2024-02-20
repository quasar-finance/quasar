use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{state::CLAIMED_INCENTIVES, ContractError};

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
pub enum IncentivesExecuteMsg {
    /// Submit a range to the range middleware
    Claim {
        for_user: String,
        claim_coins: CoinVec,
        proof_str: String,
    },
}

pub fn execute_incentives_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    incentives_msg: IncentivesExecuteMsg,
) -> Result<Response, ContractError> {
    match incentives_msg {
        IncentivesExecuteMsg::Claim {
            for_user,
            claim_coins,
            proof_str,
        } => execute_claim(deps, env, info, for_user, claim_coins, proof_str),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    for_user: String,
    claim_coins: CoinVec,
    proof_str: String,
) -> Result<Response, ContractError> {
    let for_user_addr = deps.api.addr_validate(&for_user)?;

    let this_claim = is_valid_claim(
        deps.as_ref(),
        for_user_addr.clone(),
        &claim_coins,
        proof_str,
    )?;

    // bank sends for all coins in this_claim
    let msgs = this_claim.into_bank_sends(&for_user.to_string())?;

    CLAIMED_INCENTIVES.save(deps.storage, for_user_addr, &claim_coins)?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "claim")
        .add_attribute("result", "success")
        .add_attribute("for_user", for_user)
        .add_attribute("claimed_amount", this_claim.to_string()))
}
