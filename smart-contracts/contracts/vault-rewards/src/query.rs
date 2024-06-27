use crate::helpers::get_user_reward_index;
use crate::msg::{ConfigResponse, QueryAllUsersResponse};
use crate::state::{DistributionSchedule, UserBalance, CONFIG, USER_REWARD_INDEX};
use crate::VaultRewardsError;
use crate::{execute::user::get_claim_amount, state::UserRewardIndex};
use cosmwasm_std::{Addr, Deps, Env, Order, Uint128};
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

pub fn query_all_users(
    deps: Deps,
    start_after: Option<usize>,
    limit: Option<usize>,
) -> Result<QueryAllUsersResponse, VaultRewardsError> {
    let mut users: Vec<Addr> = vec![];

    let items = USER_REWARD_INDEX
        .range(deps.storage, None, None, Order::Ascending)
        .skip(start_after.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Result<Vec<_>, _>>()?;

    for item in items {
        users.push(item.0.clone());
    }

    Ok(QueryAllUsersResponse { users })
}

#[cfg(test)]
mod tests {
    use crate::query::query_all_users;
    use crate::state::{DistributionSchedule, UserBalance, UserRewardIndex, USER_REWARD_INDEX};
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Addr, Uint128};

    #[test]
    fn test_query_all_users_and_rewards_pagination() {
        let mut deps = mock_dependencies();

        // Populate the USER_REWARD_INDEX with sample data
        let user1 = Addr::unchecked("user1");
        let user2 = Addr::unchecked("user2");
        let user3 = Addr::unchecked("user3");
        let user4 = Addr::unchecked("user4");
        let user5 = Addr::unchecked("user5");
        let user6 = Addr::unchecked("user6");

        let user_data1 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 1,
                balance: Uint128::new(100),
            }),
            history: vec![DistributionSchedule {
                start: 1,
                end: 10,
                amount: Uint128::new(1000),
            }],
        };

        let user_data2 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 2,
                balance: Uint128::new(200),
            }),
            history: vec![DistributionSchedule {
                start: 11,
                end: 20,
                amount: Uint128::new(2000),
            }],
        };

        let user_data3 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 3,
                balance: Uint128::new(300),
            }),
            history: vec![DistributionSchedule {
                start: 21,
                end: 30,
                amount: Uint128::new(3000),
            }],
        };

        let user_data4 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 4,
                balance: Uint128::new(300),
            }),
            history: vec![DistributionSchedule {
                start: 22,
                end: 30,
                amount: Uint128::new(3000),
            }],
        };

        let user_data5 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 5,
                balance: Uint128::new(300),
            }),
            history: vec![DistributionSchedule {
                start: 23,
                end: 30,
                amount: Uint128::new(3000),
            }],
        };

        let user_data6 = UserRewardIndex {
            balance: Some(UserBalance {
                reward_index: 6,
                balance: Uint128::new(300),
            }),
            history: vec![DistributionSchedule {
                start: 24,
                end: 30,
                amount: Uint128::new(3000),
            }],
        };

        USER_REWARD_INDEX
            .save(&mut deps.storage, user1.clone(), &user_data1)
            .unwrap();
        USER_REWARD_INDEX
            .save(&mut deps.storage, user2.clone(), &user_data2)
            .unwrap();
        USER_REWARD_INDEX
            .save(&mut deps.storage, user3.clone(), &user_data3)
            .unwrap();
        USER_REWARD_INDEX
            .save(&mut deps.storage, user4.clone(), &user_data4)
            .unwrap();
        USER_REWARD_INDEX
            .save(&mut deps.storage, user5.clone(), &user_data5)
            .unwrap();
        USER_REWARD_INDEX
            .save(&mut deps.storage, user6.clone(), &user_data6)
            .unwrap();

        // Perform the query with pagination
        let start_after = Some(0);
        let limit = Some(2);

        let response = query_all_users(deps.as_ref(), start_after, limit).unwrap();

        // Expected users in the response
        let expected_users = vec![user1, user2];

        // Check the results
        assert_eq!(response.users, expected_users);

        let start_after = Some(2);
        let limit = Some(10);

        let response = query_all_users(deps.as_ref(), start_after, limit).unwrap();

        // Expected users in the response
        let expected_users = vec![user3, user4, user5, user6];

        // Check the results
        assert_eq!(response.users, expected_users);
    }
}
