use cosmwasm_std::{wasm_execute, Addr, Deps, Env, StdResult, Storage, WasmMsg};
use lp_strategy::msg::UnbondingClaimResponse;
use vault_rewards::msg::{ExecuteMsg as VaultRewardsExecuteMsg, VaultExecuteMsg};

use crate::state::{INVESTMENT, VAULT_REWARDS};
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

pub fn update_user_reward_index(storage: &dyn Storage, user: &Addr) -> StdResult<WasmMsg> {
    wasm_execute(
        VAULT_REWARDS.load(storage)?,
        &VaultRewardsExecuteMsg::Vault(VaultExecuteMsg::UpdateUserRewardIndex(user.to_string())),
        vec![],
    )
}

pub fn is_contract_owner(deps: &Deps, sus_owner: &Addr) -> Result<(), ContractError> {
    let info = INVESTMENT.load(deps.storage)?;
    if info.owner.as_str() != sus_owner.as_str() {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
