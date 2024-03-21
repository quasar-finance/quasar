use crate::{
    state::{CLAIMED_INCENTIVES, CONFIG},
    ContractError,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use cosmwasm_schema::{
    cw_serde,
    serde::{self, Deserialize, Deserializer, Serializer},
};
use cosmwasm_std::{DepsMut, Env, Response};

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

pub(crate) fn as_base64<S>(array_of_bytes: &[[u8; 32]], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let base64_strings: Vec<String> = array_of_bytes
        .iter()
        .map(|bytes| STANDARD.encode(bytes))
        .collect();

    serializer.serialize_some(&base64_strings)
}

pub(crate) fn from_base64<'de, D>(deserializer: D) -> Result<Vec<[u8; 32]>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec_of_strings = Vec::<String>::deserialize(deserializer)?;
    let mut bytes_vec = Vec::new();

    for s in vec_of_strings {
        let decoded = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;
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
    env: Env,
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
            env,
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
    env: Env,
    coins: CoinVec,
    proof_hashes: Vec<[u8; 32]>,
    leaf_index: usize,
    total_leaves_count: usize,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if env.block.height >= config.expiration_block {
        return Err(ContractError::ExpirationHeightReached);
    }

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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies_with_balance, mock_env},
        Addr,
    };
    use rs_merkle::{algorithms, Hasher};

    use crate::state::Config;

    use super::*;

    #[test]
    fn claiming_after_expiration_fails() {
        let mut deps = mock_dependencies_with_balance(&[coin(1000, "ugauge")]);
        let mut env = mock_env();

        // we mock a gauge of 100 blocks, where the creator can clawback after expiration of the gauge
        // in this example, 1000 out of the 10_000 tokens are unclaimed, and thus can be clawed back
        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    clawback_address: Addr::unchecked("bob"),
                    start_block: 1,
                    end_block: 100,
                    expiration_block: 1000,
                    total_incentives: vec![coin(10000, "ugauge")],
                },
            )
            .unwrap();

        env.block.height = 1000;

        let err = execute_claim(
            deps.as_mut(),
            env,
            CoinVec(vec![coin(1000, "ugauge")]),
            get_leaves(),
            1,
            1,
            "should_fail_doesn't_matter".to_string(),
        )
        .unwrap_err();

        assert_eq!(err, ContractError::ExpirationHeightReached)
    }

    fn get_leaves() -> Vec<[u8; 32]> {
        let leaves_str = vec![
            format!("{}900000000ugauge", "a").to_string(),
            format!("{}9000000000ugauge", "b").to_string(),
            format!("{}90000000000ugauge", "c").to_string(),
            format!("{}900000000000ugauge", "d").to_string(),
            format!("{}9000000000000ugauge", "e").to_string(),
            format!("{}90000000000000ugauge", "f").to_string(),
            format!("{}900000000000000ugauge", "g").to_string(),
            format!("{}9000000000900000ugauge", "h").to_string(),
            format!("{}90000000009000000ugauge", "i").to_string(),
            format!("{}900000000090000000ugauge", "j").to_string(),
        ];

        // , accounts[0].address().to_string()which seems to generate this root: 0hGvbH+l9pdPgOmJY6wZuwjsrvtPsuslgTURavrUP6I=

        // create leave hashes from above strings
        let leaves = leaves_str
            .iter()
            .map(|x| algorithms::Sha256::hash(x.as_bytes()))
            .collect::<Vec<[u8; 32]>>();
        leaves
    }
}
