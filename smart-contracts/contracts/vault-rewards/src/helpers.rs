use crate::state::{RewardIndex, UserRewardIndex, CONFIG, REWARD_INDEX, USER_REWARD_INDEX};
use crate::VaultRewardsError;
use cosmwasm_std::{Addr, Env, QuerierWrapper, Storage};
use cw20::Cw20Contract;

pub fn update_reward_index(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
) -> Result<RewardIndex, VaultRewardsError> {
    let cur_block_height = env.block.height;
    let mut reward_index = REWARD_INDEX
        .load(storage, cur_block_height)
        .unwrap_or_default();
    reward_index.vault_supply = Cw20Contract(CONFIG.load(storage)?.vault_token)
        .meta(querier)?
        .total_supply;
    REWARD_INDEX.save(storage, cur_block_height, &reward_index)?;
    Ok(reward_index)
}

pub fn get_user_reward_index(storage: &dyn Storage, user: &Addr) -> UserRewardIndex {
    USER_REWARD_INDEX
        .load(storage, user.clone())
        .unwrap_or_else(|_| UserRewardIndex {
            balance: None,
            history: vec![],
        })
}

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), VaultRewardsError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != sus_admin.to_string() {
            return Err(VaultRewardsError::Unauthorized {});
        }
    } else {
        return Err(VaultRewardsError::Unauthorized {});
    }
    Ok(())
}
