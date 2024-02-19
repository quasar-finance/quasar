use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::state::MERKLE_ROOT;

use super::CoinVec;

#[cw_serde]
#[derive(QueryResponses)]
pub enum IncentivesQueryMsg {
    // Get the pending ranges
    #[returns(Option<String>)]
    GetMerkleRoot {},
    // Get the pending ranges for a specific contract
    #[returns(bool)]
    IsValidClaim {
        for_user: String,
        claim_coins: CoinVec,
        proof_str: String,
    },
}

pub fn query_incentives(deps: Deps, _env: Env, query_msg: IncentivesQueryMsg) -> StdResult<Binary> {
    match query_msg {
        IncentivesQueryMsg::GetMerkleRoot {} => get_merkle_root(deps),
        IncentivesQueryMsg::IsValidClaim {
            for_user,
            claim_coins,
            proof_str,
        } => is_valid_claim(deps, for_user, claim_coins, proof_str),
    }
}

pub fn get_merkle_root(deps: Deps) -> StdResult<Binary> {
    let merkle_root = MERKLE_ROOT.may_load(deps.storage)?;

    to_json_binary(&merkle_root)
}

pub fn is_valid_claim(
    deps: Deps,
    for_user: String,
    claim_coins: CoinVec,
    proof_str: String,
) -> StdResult<Binary> {
    let for_user_addr = deps.api.addr_validate(&for_user)?;
    match super::helpers::is_valid_claim(deps, for_user_addr, &claim_coins, proof_str) {
        Ok(_claim_coins) => to_json_binary(&true),
        Err(_err) => to_json_binary(&false),
    }
}
