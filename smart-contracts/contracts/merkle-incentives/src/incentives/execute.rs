use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Response};

use crate::{state::CLAIMED_INCENTIVES, ContractError};

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
pub enum IncentivesExecuteMsg {
    Claim {
        coins: CoinVec,
        proof_hashes: Vec<[u8; 32]>,
        leaf_index: usize,
        total_leaves_count: usize,
        address: String,
    },
}

pub fn handle_execute_incentives(
    deps: DepsMut,
    incentives_msg: IncentivesExecuteMsg,
) -> Result<Response, ContractError> {
    match incentives_msg {
        IncentivesExecuteMsg::Claim {
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
            address,
        } => execute_claim(
            deps,
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
            address,
        ),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    coins: CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
    address: String,
) -> Result<Response, ContractError> {
    let address_validated = deps.api.addr_validate(&address)?;

    let claim_amount = is_valid_claim(
        deps.as_ref(),
        &address_validated,
        &coins,
        proof_hashes,
        leaf_index,
        total_leaves_count,
    )?;

    // bank sends for all coins in this_claim
    let bank_msgs = claim_amount
        .into_bank_sends(deps.api.addr_validate(address_validated.as_str())?.as_str())?;

    CLAIMED_INCENTIVES.save(deps.storage, address_validated, &coins)?;

    Ok(Response::new()
        .add_messages(bank_msgs)
        .add_attribute("action", "claim")
        .add_attribute("result", "success")
        .add_attribute("address", address)
        .add_attribute("claimed_amount", claim_amount.to_string()))
}
