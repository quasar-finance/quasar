use cosmwasm_std::{from_binary, Deps, Env, Uint128};
use lp_strategy::msg::UnbondingClaimResponse;

use crate::{state::UnbondingStub, ContractError};

pub fn can_unbond_from_primitive(
    deps: Deps,
    env: &Env,
    unbond_id: &str,
    stub: &UnbondingStub,
) -> Result<bool, ContractError> {
    // only attempt if we already know we passed unlock time.
    if !stub
        .unlock_time
        .map_or(false, |unlock_time| unlock_time < env.block.time)
    {
        return Ok(false);
    }

    let unbonding_claim_query = lp_strategy::msg::QueryMsg::UnbondingClaim {
        addr: env.contract.address.clone(),
        id: unbond_id.to_string(),
    };
    let unbonding_claim: UnbondingClaimResponse = deps
        .querier
        .query_wasm_smart(stub.address.clone(), &unbonding_claim_query)?;

    // if we attempted to unbond, don't attempt again
    match unbonding_claim.unbond.attempted {
        true => Ok(false),
        false => Ok(unbonding_claim.unbond.unlock_time < env.block.time),
    }
}
