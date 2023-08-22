# CHANGELOG

## [0.1.1] - 2023-07-19

### Added
- Nothing

### Changed
- Updated the query to fetch the claimable amount for give user address.
  
Earlier it was not changing the user reward index, the push of current balance to history while querying fixed it :
```rust
pub fn query_pending_rewards(
    deps: Deps,
    env: Env,
    user: Addr,
) -> Result<Uint128, VaultRewardsError> {
    let config = CONFIG.load(deps.storage)?;
    let user_reward_index = get_user_reward_index(deps.storage, &user);
    get_claim_amount(deps, &env, &config, &user_reward_index)
}
```
Changed to :
```rust
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
```

### Deprecated
- Nothing

### Removed
- Nothing

### Fixed
- Fixed issue in fn `get_claim_amount()` where previously unwrap was happening on a nil vector.

Old code 
```rust
if reward_indexes.last().unwrap().0 != env.block.height && d.end == env.block.height {
    reward_indexes.push((env.block.height, RewardIndex { vault_supply }));
}
```
New code
```rust
if let Some(value) = reward_indexes.last() {
    if value.0 != env.block.height & & d.end == env.block.height {
        reward_indexes.push((env.block.height, RewardIndex { vault_supply }));
    }
}
```

### Security
- Nothing




