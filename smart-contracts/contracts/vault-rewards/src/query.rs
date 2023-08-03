use std::ops::Deref;
use crate::helpers::{get_user_reward_index, update_reward_index};
use crate::msg::ConfigResponse;
use crate::state::{CONFIG, DistributionSchedule, REWARD_INDEX, UserBalance};
use crate::VaultRewardsError;
use crate::{execute::user::get_claim_amount, state::UserRewardIndex};
use cosmwasm_std::{Addr, Deps, DepsMut, Env, Uint128};
use cw20::Cw20Contract;
use cw_asset::AssetInfo;

pub fn query_config(deps: Deps, env: Env) -> Result<ConfigResponse, VaultRewardsError> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        reward_token: config.reward_token.clone(),
        contract_balance: config
            .reward_token
            .query_balance(&deps.querier, env.contract.address)?,
        total_claimed: config.total_claimed,
        distribution_schedules: config
            .distribution_schedules
            .iter()
            .enumerate()
            .map(|(idx, s)| s.to_response(idx))
            .collect(),
        current_distribution_rate_per_block: config
            .get_distribution_rate_at_height(env.block.height),
    })
}

pub fn query_pending_rewards(
    deps: Deps,
    env: Env,
    user: Addr,
) -> Result<Uint128, VaultRewardsError> {
    let config = CONFIG.load(deps.storage)?;
    let cur_block_height = env.block.height;
    let mut user_reward_index = get_user_reward_index(deps.storage, &user);
    let user_vault_token_balance =
        AssetInfo::cw20(config.vault_token.clone()).query_balance(&deps.querier, &user)?;
    if let Some(prev_balance) = user_reward_index.balance {
        user_reward_index.history.push(DistributionSchedule {
            start: prev_balance.reward_index,
            end: cur_block_height,
            amount: prev_balance.balance,
        });
        user_reward_index.balance = if !user_vault_token_balance.is_zero() {
            Some(UserBalance {
                reward_index: cur_block_height + 1,
                balance: user_vault_token_balance,
            })
        } else {
            None
        };
    }
    get_claim_amount(deps, &env, &config, &user_reward_index)
}

pub fn query_user_rewards_index(
    deps: Deps,
    user: Addr,
) -> Result<UserRewardIndex, VaultRewardsError> {
    let user_reward_index = get_user_reward_index(deps.storage, &user);
    Ok(user_reward_index)
}

#[cfg(test)]
mod tests {}
