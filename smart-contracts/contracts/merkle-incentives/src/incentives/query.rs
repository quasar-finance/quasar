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
        address: String,
        coins: CoinVec,
        proof: String,
    },
}

pub fn query_incentives(deps: Deps, _env: Env, query_msg: IncentivesQueryMsg) -> StdResult<Binary> {
    match query_msg {
        IncentivesQueryMsg::GetMerkleRoot {} => get_merkle_root(deps),
        IncentivesQueryMsg::IsValidClaim {
            address,
            coins,
            proof,
        } => is_valid_claim(deps, address, coins, proof),
    }
}

pub fn get_merkle_root(deps: Deps) -> StdResult<Binary> {
    let merkle_root = MERKLE_ROOT.may_load(deps.storage)?;

    to_json_binary(&merkle_root)
}

pub fn is_valid_claim(
    deps: Deps,
    address: String,
    coins: CoinVec,
    proof: String,
) -> StdResult<Binary> {
    let address_validated = deps.api.addr_validate(&address)?;
    match super::helpers::is_valid_claim(deps, address_validated, &coins, proof) {
        Ok(_coins) => to_json_binary(&true),
        Err(_err) => to_json_binary(&false),
    }
}