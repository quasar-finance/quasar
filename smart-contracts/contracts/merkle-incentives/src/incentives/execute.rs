use crate::{state::CLAIMED_INCENTIVES, ContractError};
use cosmwasm_schema::{
    cw_serde,
    serde::{self, Deserialize, Deserializer, Serializer},
};
use cosmwasm_std::{DepsMut, Response};

use super::{helpers::is_valid_claim, CoinVec};

#[cw_serde]
pub enum IncentivesExecuteMsg {
    Claim {
        coins: CoinVec,
        #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
        proof_hashes: Vec<[u8; 32]>,
        leaf_index: usize,
        total_leaves_count: usize,
        address: String,
    },
}

fn as_base64<S>(array_of_bytes: &[[u8; 32]], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let base64_strings: Vec<String> = array_of_bytes
        .iter()
        .map(|bytes| base64::encode(bytes))
        .collect();

    serializer.serialize_some(&base64_strings)
}

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<[u8; 32]>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec_of_strings = Vec::<String>::deserialize(deserializer)?;
    let mut bytes_vec = Vec::new();

    for s in vec_of_strings {
        let decoded = base64::decode(&s).map_err(serde::de::Error::custom)?;
        let mut array = [0u8; 32];
        // Ensure the decoded bytes fit into the 32-byte array.
        if decoded.len() != array.len() {
            return Err(serde::de::Error::custom("Invalid byte length"));
        }
        array.copy_from_slice(&decoded);
        bytes_vec.push(array);
    }

    Ok(bytes_vec)
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
