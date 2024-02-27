use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};

use crate::{state::CLAIMED_INCENTIVES, ContractError};

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
pub enum IncentivesExecuteMsg {
    Claim {
        coins: CoinVec,
        proof_hashes: Vec<[u8; 32]>,
        leaf_index: usize,
        total_leaves_count: usize,
        destination_address: Addr,
    },
}

pub fn handle_execute_incentives(
    deps: DepsMut,
    info: MessageInfo,
    incentives_msg: IncentivesExecuteMsg,
) -> Result<Response, ContractError> {
    match incentives_msg {
        IncentivesExecuteMsg::Claim {
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
            destination_address,
        } => execute_claim(
            deps,
            info,
            coins,
            proof_hashes,
            leaf_index,
            total_leaves_count,
            destination_address,
        ),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    info: MessageInfo,
    coins: CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
    destination_address: Addr,
) -> Result<Response, ContractError> {
    let address_validated = deps.api.addr_validate(destination_address.as_str())?;

    let claim_amount = is_valid_claim(
        deps.as_ref(),
        address_validated,
        &coins,
        proof_hashes,
        leaf_index,
        total_leaves_count,
    )?;

    // bank sends for all coins in this_claim
    let bank_msgs = claim_amount.into_bank_sends(address_validated.as_str())?;

    CLAIMED_INCENTIVES.save(deps.storage, address_validated, &coins)?;

    Ok(Response::new()
        .add_messages(bank_msgs)
        .add_attribute("action", "claim")
        .add_attribute("result", "success")
        .add_attribute("address", info.sender.to_string())
        .add_attribute("claimed_amount", claim_amount.to_string()))
}
