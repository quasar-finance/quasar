use cl_vault::ContractError;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, Coin, Deps, DepsMut, Env, StdResult};

#[cw_serde]
#[derive(QueryResponses)]
pub enum IncentivesQueryMsg {
    // Get the pending ranges
    #[returns(String)]
    GetMerkleRoot {},
    // Get the pending ranges for a specific contract
    #[returns(bool)]
    IsValidClaim {
        for_user: String,
        claim_coins: Vec<Coin>,
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
    todo!()
}

pub fn is_valid_claim(
    deps: Deps,
    for_user: String,
    claim_coins: Vec<Coin>,
    proof_str: String,
) -> StdResult<Binary> {
    todo!()
}
