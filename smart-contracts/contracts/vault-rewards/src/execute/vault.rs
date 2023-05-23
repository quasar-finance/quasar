use crate::helpers::{get_user_reward_index, update_reward_index};
use crate::state::{DistributionSchedule, UserBalance, CONFIG, USER_REWARD_INDEX};
use crate::VaultRewardsError;
use cosmwasm_std::{Addr, DepsMut, Env, Response};
use cw_asset::AssetInfo;
use quasar_types::types::ItemShouldLoad;

pub fn execute_update_user_reward_index(
    deps: DepsMut,
    env: Env,
    user: Addr,
) -> Result<Response, VaultRewardsError> {
    let cur_block_height = env.block.height;
    let user_vault_token_balance = AssetInfo::cw20(CONFIG.should_load(deps.storage)?.vault_token)
        .query_balance(&deps.querier, &user)?;
    let mut user_reward_index = get_user_reward_index(deps.storage, &user);
    // if previous balance, then move to history and record new balance
    if let Some(prev_balance) = user_reward_index.balance {
        user_reward_index.history.push(DistributionSchedule {
            start: prev_balance.reward_index,
            end: cur_block_height,
            amount: prev_balance.balance,
        })
    }
    user_reward_index.balance = if !user_vault_token_balance.is_zero() {
        Some(UserBalance {
            reward_index: cur_block_height,
            balance: user_vault_token_balance,
        })
    } else {
        None
    };
    USER_REWARD_INDEX.save(deps.storage, user.clone(), &user_reward_index)?;
    update_reward_index(deps.storage, &deps.querier, &env)?;
    Ok(Response::default().add_attributes(vec![
        ("action", "update_user_index"),
        ("user", user.as_ref()),
        ("vault_token_balance", &user_vault_token_balance.to_string()),
    ]))
}

#[cfg(test)]
mod tests {}
