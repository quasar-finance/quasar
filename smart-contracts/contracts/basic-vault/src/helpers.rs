use cosmwasm_std::{from_binary, Deps, Env, Uint128};
use lp_strategy::msg::UnbondingClaimResponse;

use crate::{state::UnbondingStub, ContractError};

pub fn can_unbond_from_primitive(
    deps: Deps,
    env: &Env,
    unbond_id: &str,
    stub: &UnbondingStub,
) -> Result<bool, ContractError> {
    let unbonding_claim_query = lp_strategy::msg::QueryMsg::UnbondingClaim {
        addr: env.contract.address.clone(),
        id: unbond_id.to_string(),
    };
    let unbonding_claim_response = deps
        .querier
        .query_wasm_smart(stub.address.clone(), &unbonding_claim_query)?;
    let unbonding_claim: UnbondingClaimResponse = from_binary(&unbonding_claim_response)?;

    match unbonding_claim.unbond {
        Some(unbond) => Ok(unbond.unlock_time < env.block.time),
        None => Ok(false),
    }
}
