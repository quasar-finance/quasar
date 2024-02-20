use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{state::CLAIMED_INCENTIVES, ContractError};

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
pub enum IncentivesExecuteMsg {
    /// Submit a range to the range middleware
    Claim {
        address: String,
        coins: CoinVec,
        proof: String,
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
            address,
            coins,
            proof,
        } => execute_claim(deps, env, info, address, coins, proof),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    address: String,
    coins: CoinVec,
    proof: String,
) -> Result<Response, ContractError> {
    let address_validated = deps.api.addr_validate(&address)?;

    let this_claim = is_valid_claim(deps.as_ref(), address_validated.clone(), &coins, proof)?;

    // bank sends for all coins in this_claim
    let msgs = this_claim.into_bank_sends(&address.to_string())?;

    CLAIMED_INCENTIVES.save(deps.storage, address_validated, &coins)?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "claim")
        .add_attribute("result", "success")
        .add_attribute("address", address)
        .add_attribute("claimed_amount", this_claim.to_string()))
}
