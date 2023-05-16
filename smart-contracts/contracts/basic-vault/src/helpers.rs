use cosmwasm_std::{wasm_execute, Addr, Deps, Env, QuerierWrapper, Storage, WasmMsg};
use lp_strategy::msg::UnbondingClaimResponse;
use quasar_types::types::ItemShouldLoad;
use vault_rewards::msg::{ExecuteMsg as VaultRewardsExecuteMsg, VaultExecuteMsg};

use crate::state::VAULT_REWARDS;
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
    if let Some(unbond) = unbonding_claim.unbond {
        match unbond.attempted {
            true => Ok(false),
            false => Ok(unbond.unlock_time < env.block.time),
        }
    } else {
        Ok(true)
    }
}

pub fn update_user_reward_index(
    storage: &dyn Storage,
    user: &Addr,
) -> Result<WasmMsg, ContractError> {
    Ok(wasm_execute(
        VAULT_REWARDS.should_load(storage)?,
        &VaultRewardsExecuteMsg::Vault(VaultExecuteMsg::UpdateUserRewardIndex(user.to_string())),
        vec![],
    )?)
}

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
