use crate::helpers::{get_user_reward_index, update_reward_index};
use crate::state::{
    Config, DistributionSchedule, RewardIndex, UserBalance, UserRewardIndex, CONFIG, REWARD_INDEX,
    USER_REWARD_INDEX,
};
use crate::VaultRewardsError;
use cosmwasm_std::{Addr, Deps, DepsMut, Env, Order, Response, StdResult, Uint128};
use cw20::Cw20Contract;
use cw_asset::{Asset, AssetInfo};
use cw_storage_plus::Bound;

pub fn execute_claim(deps: DepsMut, env: &Env, user: Addr) -> Result<Response, VaultRewardsError> {
    let mut config = CONFIG.load(deps.storage)?;
    let cur_block_height = env.block.height;
    let mut user_reward_index = get_user_reward_index(deps.storage, &user);
    let user_vault_token_balance =
        AssetInfo::cw20(config.vault_token.clone()).query_balance(&deps.querier, &user)?;
    // if previous balance, move to history and start new balance index
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
    // update global reward index before calculating user claim amount
    update_reward_index(deps.storage, &deps.querier, env)?;
    let claim_amount = get_claim_amount(deps.as_ref(), env, &config, &user_reward_index)?;

    // double check we have enough balance to cover this
    let contract_reward_token_balance = config
        .reward_token
        .query_balance(&deps.querier, &env.contract.address)?;
    if contract_reward_token_balance < claim_amount {
        return Err(VaultRewardsError::InsufficientFunds {
            contract_balance: contract_reward_token_balance,
            claim_amount,
        });
    }

    let claim = Asset::new(config.reward_token.clone(), claim_amount).transfer_msg(&user)?;
    user_reward_index.history = vec![];
    USER_REWARD_INDEX.save(deps.storage, user.clone(), &user_reward_index)?;
    config.total_claimed += claim_amount;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_message(claim).add_attributes(vec![
        ("action", "claim"),
        ("user", user.as_ref()),
        ("amount", &claim_amount.to_string()),
    ]))
}

pub fn get_claim_amount(
    deps: Deps,
    env: &Env,
    config: &Config,
    user_reward_index: &UserRewardIndex,
) -> Result<Uint128, VaultRewardsError> {
    let mut user_reward_index_history = user_reward_index.history.clone();
    // if we don't have the index history until current block height
    if (user_reward_index_history.is_empty()
        || user_reward_index_history.last().unwrap().end < env.block.height)
        && user_reward_index.balance.is_some()
    {
        user_reward_index_history.push(DistributionSchedule {
            start: user_reward_index.balance.clone().unwrap().reward_index + 1,
            end: env.block.height,
            amount: user_reward_index.balance.clone().unwrap().balance,
        });
    }

    let vault_supply = Cw20Contract(CONFIG.load(deps.storage)?.vault_token)
        .meta(&deps.querier)?
        .total_supply;

    let mut claim_amount = user_reward_index_history
        .iter()
        .map(|d| {
            let mut cur_height = d.start;
            let mut reward_indexes = REWARD_INDEX
                .range(
                    deps.storage,
                    Some(Bound::inclusive(d.start - 1)),
                    Some(Bound::inclusive(d.end)),
                    Order::Ascending,
                )
                .collect::<StdResult<Vec<(u64, RewardIndex)>>>()
                .unwrap();

            if let Some(value) = reward_indexes.last() {
                if value.0 != env.block.height && d.end == env.block.height {
                    reward_indexes.push((env.block.height, RewardIndex { vault_supply }));
                }
            }
            // iterate over reward indexes 2 at a time to calculate reward for each period
            reward_indexes
                .iter()
                .zip(reward_indexes.iter().skip(1))
                .map(|(start, end)| {
                    let (_, reward_index_start) = start;
                    let (height_end, _) = end;
                    let mut period_claim_amount = Uint128::zero();
                    while cur_height <= *height_end {
                        let block_reward = config.get_distribution_rate_at_height(cur_height);
                        // calculate reward for user based on their share of the vault supply
                        period_claim_amount +=
                            block_reward * d.amount / reward_index_start.vault_supply;
                        cur_height += 1;
                    }
                    period_claim_amount
                })
                .sum::<Uint128>()
        })
        .sum::<Uint128>();
    // this accounts for edge case where final user withdraws their claim (ends up being ~1% less than expected due to rounding)
    claim_amount = claim_amount.min(config.get_total_distribution_amount() - config.total_claimed);
    if claim_amount.is_zero() {
        return Err(VaultRewardsError::NoRewardsToClaim {});
    }
    Ok(claim_amount)
}

#[cfg(test)]
mod tests {
    use crate::execute::mock_querier::{mock_dependencies, WasmMockQuerier};
    use crate::execute::user::{execute_claim, get_claim_amount};
    use crate::execute::vault::execute_update_user_reward_index;
    use crate::helpers::{get_user_reward_index};
    use crate::state::{Config, DistributionSchedule, CONFIG};
    use crate::VaultRewardsError;
    use cosmwasm_std::testing::{mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{attr, Addr, Coin, Env, OwnedDeps, Uint128};
    use cw_asset::AssetInfo;

    #[test]
    fn test_execute_claim() {
        let mut deps = mock_dependencies(&[]);
        let mut env = mock_env();
        let mut total_claim_amount = Uint128::zero();
        let config = Config {
            vault_token: Addr::unchecked("vault_token"),
            reward_token: AssetInfo::native("reward_token"),
            distribution_schedules: vec![
                DistributionSchedule {
                    start: 100,
                    end: 1000,
                    amount: Uint128::new(900000000),
                },
                DistributionSchedule {
                    start: 500,
                    end: 1500,
                    amount: Uint128::new(1000000000),
                },
            ],
            total_claimed: Uint128::zero(),
        };
        env.block.height = 1;
        CONFIG.save(deps.as_mut().storage, &config).unwrap();

        let user1 = Addr::unchecked("user1");
        let user2 = Addr::unchecked("user2");
        let user3 = Addr::unchecked("user3");
        let user4 = Addr::unchecked("user4");
        let user5 = Addr::unchecked("user5");

        deps.querier
            .with_token_balance(user1.as_ref(), &Uint128::new(100));
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone()).unwrap();

        let res = execute_claim(deps.as_mut(), &env, user1.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});

        env.block.height = 100;

        // should error since no funds in contract (shouldn't happen tho since it's checked for when adding/updating distribution schedules)
        let res = execute_claim(deps.as_mut(), &env, user1.clone());
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            VaultRewardsError::InsufficientFunds {
                contract_balance: Uint128::zero(),
                claim_amount: Uint128::new(1000000)
            }
        );

        let mut contract_reward_balance = config
            .distribution_schedules
            .iter()
            .fold(Uint128::zero(), |acc, s| acc + s.amount);

        deps.querier.with_bank_balance(
            MOCK_CONTRACT_ADDR,
            vec![Coin {
                denom: "reward_token".to_string(),
                amount: contract_reward_balance,
            }],
        );

        execute_claim_helper(
            &mut deps,
            &env,
            &user1,
            Uint128::new(1000000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );

        // claim in same block should error
        let res = execute_claim(deps.as_mut(), &env, user1.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});

        env.block.height = 200;

        execute_claim_helper(
            &mut deps,
            &env,
            &user1,
            Uint128::new(100000000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );

        env.block.height = 251;

        deps.querier
            .with_token_balance(user2.as_ref(), &Uint128::new(100));
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone()).unwrap();

        env.block.height = 300;

        // user1 should have same rate until block 250, then shares reward with user2
        execute_claim_helper(
            &mut deps,
            &env,
            &user1,
            Uint128::new(75500000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );
        execute_claim_helper(
            &mut deps,
            &env,
            &user2,
            Uint128::new(25000000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );

        env.block.height = 1000;

        // transfer balance from user1 to user2
        deps.querier
            .with_token_balance(user1.as_ref(), &Uint128::zero());
        deps.querier
            .with_token_balance(user2.as_ref(), &Uint128::new(200));
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone()).unwrap();
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone()).unwrap();

        execute_claim_helper(
            &mut deps,
            &env,
            &user1,
            Uint128::new(600000000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );
        execute_claim_helper(
            &mut deps,
            &env,
            &user2,
            Uint128::new(600000000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );

        env.block.height = 1250;

        deps.querier
            .with_token_balance(user3.as_ref(), &Uint128::new(200));
        deps.querier
            .with_token_balance(user4.as_ref(), &Uint128::new(400));
        deps.querier
            .with_token_balance(user5.as_ref(), &Uint128::new(800));
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user3.clone()).unwrap();
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user4.clone()).unwrap();
        execute_update_user_reward_index(deps.as_mut(), env.clone(), user5.clone()).unwrap();

        env.block.height = 2000;

        let res = execute_claim(deps.as_mut(), &env, user1.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        execute_claim_helper(
            &mut deps,
            &env,
            &user2,
            Uint128::new(281125000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );
        execute_claim_helper(
            &mut deps,
            &env,
            &user3,
            Uint128::new(31250000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );
        execute_claim_helper(
            &mut deps,
            &env,
            &user4,
            Uint128::new(62500000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );
        execute_claim_helper(
            &mut deps,
            &env,
            &user5,
            Uint128::new(123625000),
            &mut total_claim_amount,
            &mut contract_reward_balance,
        );

        env.block.height = 3000;

        let res = execute_claim(deps.as_mut(), &env, user1.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        let res = execute_claim(deps.as_mut(), &env, user2.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        let res = execute_claim(deps.as_mut(), &env, user3.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        let res = execute_claim(deps.as_mut(), &env, user4.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        let res = execute_claim(deps.as_mut(), &env, user5.clone());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});

        let expected_claim_amount = config
            .distribution_schedules
            .iter()
            .fold(Uint128::zero(), |acc, s| acc + s.amount);
        assert_eq!(total_claim_amount, expected_claim_amount);
    }

    fn execute_claim_helper(
        deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
        env: &Env,
        user: &Addr,
        expected_claim_amount: Uint128,
        total_claim_amount: &mut Uint128,
        contract_reward_balance: &mut Uint128,
    ) {
        let res = execute_claim(deps.as_mut(), env, user.clone());
        if res.is_err() {
            println!("res: {res:?}");
        }
        assert!(res.is_ok());
        let res = res.unwrap();
        let claim_amount = res
            .attributes
            .iter()
            .find(|a| a.key == "amount")
            .unwrap()
            .value
            .parse::<Uint128>()
            .unwrap();
        *total_claim_amount += claim_amount;
        *contract_reward_balance -= claim_amount;
        deps.querier.with_bank_balance(
            MOCK_CONTRACT_ADDR,
            vec![Coin {
                denom: "reward_token".to_string(),
                amount: *contract_reward_balance,
            }],
        );
        assert_eq!(
            res.attributes,
            vec![
                attr("action", "claim"),
                attr("user", user.to_string()),
                attr("amount", expected_claim_amount.to_string()),
            ]
        );
    }

    #[test]
    fn execute_get_claim_amount() {
        let mut deps = mock_dependencies(&[]);
        let mut env = mock_env();
        let config = Config {
            vault_token: Addr::unchecked("vault_token"),
            reward_token: AssetInfo::native("reward_token"),
            distribution_schedules: vec![DistributionSchedule {
                start: 2,
                end: 14,
                amount: Uint128::new(900000000),
            }],
            total_claimed: Uint128::zero(),
        };
        env.block.height = 1;
        CONFIG.save(deps.as_mut().storage, &config).unwrap();

        let user1 = Addr::unchecked("user1");
        let user2 = Addr::unchecked("user2");

        deps.querier
            .with_token_balance(user1.as_ref(), &Uint128::new(100));
        deps.querier
            .with_token_balance(user2.as_ref(), &Uint128::new(200));

        let user_reward_index = get_user_reward_index(&deps.storage, &user1);
        let res = get_claim_amount(deps.as_ref(), &env, &config, &user_reward_index);

        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone());
        let res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone());
        assert!(res.is_ok());

        env.block.height += 10;

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone());

        env.block.height += 10;

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone());

        let user1_reward_index = get_user_reward_index(&deps.storage, &user1);
        let user2_reward_index = get_user_reward_index(&deps.storage, &user2);
        let res1 = get_claim_amount(deps.as_ref(), &env, &config, &user1_reward_index);
        let res2 = get_claim_amount(deps.as_ref(), &env, &config, &user2_reward_index);

        assert!(res1.is_ok());
        assert!(res2.is_ok());
        assert_eq!(res1.unwrap(), Uint128::new(300000000)); // user1 holds 1/3 of the vaulth
        assert_eq!(res2.unwrap(), Uint128::new(600000000)); // user2 holds 2/3 of the vaulth
    }

    #[test]
    fn execute_get_claim_amount_without_distribution_schedule() {
        let mut deps = mock_dependencies(&[]);
        let mut env = mock_env();
        let config = Config {
            vault_token: Addr::unchecked("vault_token"),
            reward_token: AssetInfo::native("reward_token"),
            distribution_schedules: vec![],
            total_claimed: Uint128::zero(),
        };
        env.block.height = 1;
        CONFIG.save(deps.as_mut().storage, &config).unwrap();

        let user1 = Addr::unchecked("user1");
        let user2 = Addr::unchecked("user2");

        deps.querier
            .with_token_balance(user1.as_ref(), &Uint128::new(100));
        deps.querier
            .with_token_balance(user2.as_ref(), &Uint128::new(200));

        let user_reward_index = get_user_reward_index(&deps.storage, &user1);
        let res = get_claim_amount(deps.as_ref(), &env, &config, &user_reward_index);

        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone());
        let res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone());
        assert!(res.is_ok());

        env.block.height += 10;

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user1.clone());

        env.block.height += 10;

        let _res = execute_update_user_reward_index(deps.as_mut(), env.clone(), user2.clone());

        let user1_reward_index = get_user_reward_index(&deps.storage, &user1);
        let user2_reward_index = get_user_reward_index(&deps.storage, &user2);
        let res1 = get_claim_amount(deps.as_ref(), &env, &config, &user1_reward_index);
        let res2 = get_claim_amount(deps.as_ref(), &env, &config, &user2_reward_index);

        assert!(res1.is_err());
        assert_eq!(res1.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
        assert!(res2.is_err());
        assert_eq!(res2.err().unwrap(), VaultRewardsError::NoRewardsToClaim {});
    }
}
