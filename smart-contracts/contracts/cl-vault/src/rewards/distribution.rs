use cosmwasm_std::{
    Addr, Attribute, Decimal, Deps, DepsMut, Env, Order, Response, StdError, StdResult, SubMsg,
    SubMsgResult, Uint128,
};

use crate::{
    error::ContractResult,
    reply::Replies,
    state::{
        CURRENT_REWARDS, CURRENT_REWARDS_INDEX, IS_DISTRIBUTING, POSITION, SHARES,
        STRATEGIST_REWARDS, USER_REWARDS, VAULT_CONFIG, VAULT_DENOM,
    },
    ContractError,
};
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::concentratedliquidity::v1beta1::{
        MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards,
        MsgCollectSpreadRewardsResponse,
    },
};

use super::helpers::CoinList;

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn execute_collect_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    if IS_DISTRIBUTING.load(deps.storage).unwrap() {
        return Err(ContractError::IsDistributing {});
    }
    IS_DISTRIBUTING.save(deps.storage, &true);
    CURRENT_REWARDS.save(deps.storage, &CoinList::new())?;

    let msg = get_collect_incentives_msg(deps.as_ref(), env)?;

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        msg,
        Replies::CollectIncentives as u64,
    )))
}

pub fn handle_collect_incentives_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // save the response from the collect incentives
    // If we do not have data here, we treat this as an empty MsgCollectIncentivesResponse, this seems to be a bug somewhere between cosmwasm and osmosis
    let data: Result<MsgCollectIncentivesResponse, ContractError> = data
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .map(|b| Ok(b.try_into()?))
        .unwrap_or(Ok(MsgCollectIncentivesResponse {
            collected_incentives: vec![],
            forfeited_incentives: vec![],
        }));

    let response: MsgCollectIncentivesResponse = data?;
    CURRENT_REWARDS.update(
        deps.storage,
        |mut rewards| -> Result<CoinList, ContractError> {
            rewards.update_rewards(response.collected_incentives)?;
            Ok(rewards)
        },
    )?;

    // collect the spread rewards
    let msg = get_collect_spread_rewards_msg(deps.as_ref(), env)?;
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        msg,
        Replies::CollectSpreadRewards as u64,
    )))
}

pub fn handle_collect_spread_rewards_reply(
    deps: DepsMut,
    _env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // after we have collected both the spread rewards and the incentives, we can distribute them over the share holders
    // we don't need to save the rewards here again, just pass it to update rewards
    let data: Result<MsgCollectSpreadRewardsResponse, ContractError> = data
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .map(|b| Ok(b.try_into()?))
        .unwrap_or(Ok(MsgCollectSpreadRewardsResponse {
            collected_spread_rewards: vec![],
        }));

    let response: MsgCollectSpreadRewardsResponse = data?;
    let mut rewards = CURRENT_REWARDS.load(deps.storage)?;
    rewards.update_rewards(response.collected_spread_rewards)?;

    CURRENT_REWARDS.save(deps.storage, &rewards)?;

    // TODO add a nice response
    Ok(Response::new())
}

pub fn execute_distribute_rewards(
    deps: DepsMut,
    _env: Env,
    amount_of_users: Uint128,
) -> Result<Response, ContractError> {
    if !IS_DISTRIBUTING.load(deps.storage).unwrap() {
        return Err(ContractError::IsDistributing {});
    }

    distribute_rewards(&mut deps, amount_of_users)?;

    // TODO add a nice response
    Ok(Response::new())
}

// TODO: deprecate this separate function and merge the content ^ in distribute_rewards so we fix bowworing
fn distribute_rewards(
    deps: &mut DepsMut,
    amount_of_users: Uint128,
) -> Result<Vec<Attribute>, ContractError> {
    let mut rewards = CURRENT_REWARDS.load(deps.storage)?;
    if rewards.is_empty() {
        return Ok(vec![Attribute::new("total_rewards_amount", "0")]);
    }
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    // Calculate the strategist fee and update the strategist rewards
    let strategist_fee = rewards.sub_ratio(vault_config.performance_fee)?;
    STRATEGIST_REWARDS.update(deps.storage, |old| -> StdResult<_> {
        Ok(old.add(strategist_fee)?)
    })?;

    // Prepare the bank querier and get the total shares
    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_shares: Uint128 = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    // Get state item for last processed user index as start
    let mut start = CURRENT_REWARDS_INDEX.load(deps.storage)?;
    let end = start + amount_of_users.u128();

    // Create a new CoinList store distributed rewards for later subtraction
    let mut distributed_rewards = CoinList::new();

    // Get all users and their current pre-distribution rewards within the range
    // TODO: We cannot use numbers as min and max, probably we need to just iterate in a sorting way
    //       and save the latest key + 1 so we know the next address to start from. Check that new users would be appended at the end of the list.
    let user_rewards: Result<Vec<(Addr, CoinList)>, ContractError> = SHARES
        .range(deps.storage, Some(todo!()), Some(todo!()), Order::Ascending)
        .take(amount_of_users.u128() as usize) // Process only the specified amount of users
        .map(|item| {
            let (address, user_shares) = item?;
            let user_reward = rewards.mul_ratio(Decimal::from_ratio(user_shares, total_shares));
            distributed_rewards.add(user_reward.clone())?; // Add the reward to the distributed sum
            Ok((address, user_reward))
        })
        .collect();

    // Update the user rewards
    user_rewards?
        .into_iter()
        .try_for_each(|(addr, reward)| -> ContractResult<()> {
            USER_REWARDS.update(deps.storage, addr, |old| -> ContractResult<CoinList> {
                Ok(match old {
                    Some(old_rewards) => reward.add(old_rewards)?,
                    None => reward,
                })
            })?;
            Ok(())
        })?;

    CURRENT_REWARDS_INDEX.save(deps.storage, &end)?; // TODO: +1 needed? so the next start is end+1?

    CURRENT_REWARDS.update(deps.storage, |old_rewards| -> StdResult<_> {
        old_rewards.sub(&distributed_rewards)?
    })?;

    // Determine if this is the last execution
    let is_last_execution = end >= total_shares.u128(); // Assuming total_shares is the upper bound
    if is_last_execution {
        IS_DISTRIBUTING.save(deps.storage, &false)?;
    }

    // Construct a response with appropriate attributes
    let attributes = vec![
        Attribute::new("total_rewards_amount", format!("{:?}", rewards.coins())),
        Attribute::new("last_processed_user_index", end.to_string()),
        Attribute::new("is_last_execution", is_last_execution.to_string()),
    ];
    Ok(attributes)
}

fn get_collect_incentives_msg(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

fn get_collect_spread_rewards_msg(
    deps: Deps,
    env: Env,
) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env},
    };

    use crate::{state::Position, test_helpers::QuasarQuerier};
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        FullPositionBreakdown, Position as OsmoPosition,
    };

    use super::*;

    #[test]
    fn test_claim_rewards() {
        let position_id = 2;
        let mut deps = mock_dependencies();
        let _qq = QuasarQuerier::new(
            FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id,
                    address: "bob".to_string(),
                    pool_id: 1,
                    lower_tick: 1,
                    upper_tick: 100,
                    join_time: None,
                    liquidity: "123.214".to_string(),
                }),
                asset0: Some(coin(1000, "uosmo").into()),
                asset1: Some(coin(1000, "uosmo").into()),
                claimable_spread_rewards: vec![coin(1000, "uosmo").into()],
                claimable_incentives: vec![coin(123, "uatom").into()],
                forfeited_incentives: vec![],
            },
            100,
        );
        let env = mock_env();
        let position = Position { position_id };
        POSITION.save(deps.as_mut().storage, &position).unwrap();

        // TODO: implement execute_collect_rewards

        // TODO: implement amount of users here iterating many times
        let resp = execute_distribute_rewards(deps.as_mut(), env.clone()).unwrap();
        assert_eq!(
            resp.messages[0].msg,
            get_collect_incentives_msg(deps.as_ref(), env).unwrap().into()
        )
    }

    // #[test]
    // fn test_handle_collect_rewards() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let position = Position { position_id: 1 };
    //     POSITION.save(deps.as_mut().storage, &position).unwrap();

    //     CURRENT_REWARDS
    //         .save(deps.as_mut().storage, &Rewards::new())
    //         .unwrap();

    //     let msg: Binary = MsgCollectIncentivesResponse {
    //         collected_incentives: vec![
    //             OsmoCoin {
    //                 denom: "uosmo".into(),
    //                 amount: "1234".into(),
    //             },
    //             OsmoCoin {
    //                 denom: "uqsr".into(),
    //                 amount: "2345".into(),
    //             },
    //         ],
    //         forfeited_incentives: vec![],
    //     }
    //     .try_into()
    //     .unwrap();

    //     let resp = handle_collect_incentives_reply(
    //         deps.as_mut(),
    //         env.clone(),
    //         SubMsgResult::Ok(SubMsgResponse {
    //             events: vec![],
    //             data: Some(msg),
    //         }),
    //     )
    //     .unwrap();

    //     assert_eq!(
    //         resp.messages[0].msg,
    //         collect_spread_rewards(deps.as_ref(), env.clone())
    //             .unwrap()
    //             .into()
    //     );

    //     let msg: Binary = MsgCollectSpreadRewardsResponse {
    //         collected_spread_rewards: vec![OsmoCoin {
    //             denom: "uatom".into(),
    //             amount: "3456".into(),
    //         }],
    //     }
    //     .try_into()
    //     .unwrap();

    //     // we need a vault config to distribute the rewards in the vault config
    //     let vault_config = VaultConfig {
    //         performance_fee: Decimal::percent(20),
    //         treasury: Addr::unchecked("strategy_man"),
    //         swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
    //     };

    //     VAULT_CONFIG
    //         .save(deps.as_mut().storage, &vault_config)
    //         .unwrap();

    //     // mock a vec of user shares
    //     let user_shares = vec![(Addr::unchecked("user1"), Uint128::new(1000))];
    //     let total = user_shares
    //         .iter()
    //         .fold(Uint128::zero(), |acc, (_, shares)| acc + shares);
    //     LOCKED_TOTAL.save(deps.as_mut().storage, &total).unwrap();
    //     user_shares
    //         .into_iter()
    //         .for_each(|(addr, shares)| SHARES.save(deps.as_mut().storage, addr, &shares).unwrap());

    //     // mock some previous rewards
    //     let strategist_rewards = Rewards::from_coins(vec![coin(50, "uosmo")]);
    //     STRATEGIST_REWARDS
    //         .save(deps.as_mut().storage, &strategist_rewards)
    //         .unwrap();

    //     let _resp = handle_collect_spread_rewards_reply(
    //         deps.as_mut(),
    //         env,
    //         SubMsgResult::Ok(SubMsgResponse {
    //             events: vec![],
    //             data: Some(msg),
    //         }),
    //     )
    //     .unwrap();

    //     // we have collected vec![coin(1234, "uosmo"), coin(2345, "uqsr"), coin(3456, "uatom")] at this point
    //     let rewards = Rewards::from_coins(vec![
    //         coin(1234, "uosmo"),
    //         coin(2345, "uqsr"),
    //         coin(3456, "uatom"),
    //     ]);

    //     assert_eq!(
    //         STRATEGIST_REWARDS.load(deps.as_ref().storage).unwrap(),
    //         strategist_rewards
    //             .add(
    //                 rewards
    //                     .clone()
    //                     .sub_percentage(
    //                         vault_config.performance_fee.numerator(),
    //                         vault_config.performance_fee.denominator()
    //                     )
    //                     .unwrap()
    //             )
    //             .unwrap()
    //     );

    //     // verify that the distributed rewards make sense
    //     let strategist_fee_percentage = VAULT_CONFIG
    //         .load(deps.as_ref().storage)
    //         .unwrap()
    //         .performance_fee;
    //     let total_shares = LOCKED_TOTAL.load(deps.as_ref().storage).unwrap();

    //     USER_REWARDS
    //         .range(deps.as_ref().storage, None, None, Order::Ascending)
    //         .for_each(|val| {
    //             let (user, user_rewards) = val.unwrap();
    //             let user_shares = SHARES.load(deps.as_ref().storage, user).unwrap();
    //             let mut tmp_rewards = rewards.clone();

    //             tmp_rewards
    //                 .sub_percentage(
    //                     strategist_fee_percentage.numerator(),
    //                     strategist_fee_percentage.denominator(),
    //                 )
    //                 .unwrap();

    //             assert_eq!(user_rewards, tmp_rewards.ratio(user_shares, total_shares))
    //         })
    // }

    // #[test]
    // fn distribute_rewards_works() {
    //     let mut deps = mock_dependencies();
    //     let mut mut_deps = deps.as_mut();

    //     let qq = QuasarQuerier::new_with_balances(
    //         FullPositionBreakdown {
    //             position: Some(OsmoPosition {
    //                 position_id: 1,
    //                 address: "bob".to_string(),
    //                 pool_id: 1,
    //                 lower_tick: 100,
    //                 upper_tick: 1000,
    //                 join_time: None,
    //                 liquidity: "1000000.2".to_string(),
    //             }),
    //             asset0: Some(OsmoCoin {
    //                 denom: "token0".to_string(),
    //                 amount: "1000000".to_string(),
    //             }),
    //             asset1: Some(OsmoCoin {
    //                 denom: "token1".to_string(),
    //                 amount: "1000000".to_string(),
    //             }),
    //             claimable_spread_rewards: vec![
    //                 OsmoCoin {
    //                     denom: "token0".to_string(),
    //                     amount: "100".to_string(),
    //                 },
    //                 OsmoCoin {
    //                     denom: "token1".to_string(),
    //                     amount: "100".to_string(),
    //                 },
    //             ],
    //             claimable_incentives: vec![],
    //             forfeited_incentives: vec![],
    //         },
    //         500,
    //         &[]
    //     );
    //     mut_deps.querier = QuerierWrapper::new(&qq);
    //     // let qq = QuasarQuerier::new()
    //     // we need a vault config to distribute the rewards in the vault config
    //     VAULT_CONFIG
    //         .save(
    //             mut_deps.storage,
    //             &VaultConfig {
    //                 performance_fee: Decimal::percent(20),
    //                 treasury: Addr::unchecked("strategy_man"),
    //                 swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
    //             },
    //         )
    //         .unwrap();

    //     VAULT_DENOM.save(mut_deps.storage, &"share_denom".to_string()).unwrap();

    //     // mock a vec of user shares
    //     let user_shares = vec![(Addr::unchecked("user1"), Uint128::new(1000))];
    //     let total = user_shares
    //         .iter()
    //         .fold(Uint128::zero(), |acc, (_, shares)| acc + shares);
    //     user_shares.into_iter().for_each(|(addr, shares)| {
    //         SHARES
    //             .save(mut_deps.storage, addr, &shares)
    //             .unwrap()
    //     });

    //     let strategist_rewards = Rewards::from_coins(vec![coin(50, "uosmo")]);
    //     STRATEGIST_REWARDS
    //         .save(mut_deps.storage, &strategist_rewards)
    //         .unwrap();

    //     let rewards = Rewards::from_coins(vec![coin(10000, "uosmo"), coin(1000000, "uatom")]);
    //     distribute_rewards(mut_deps, rewards.clone()).unwrap();

    //     // each entry in USER_REWARDS should be equal to rewards.sub_percentage(strategist_fee_percentage).percentage(user_shares, total_shares)
    //     // we can get the user shares from SHARES
    //     let strategist_fee_percentage = VAULT_CONFIG
    //         .load(mut_deps.storage)
    //         .unwrap()
    //         .performance_fee;

    //     assert_eq!(
    //         STRATEGIST_REWARDS.load(mut_deps.storage).unwrap(),
    //         strategist_rewards
    //             .add(
    //                 rewards
    //                     .clone()
    //                     .sub_ratio(
    //                         strategist_fee_percentage
    //                     )
    //                     .unwrap()
    //             )
    //             .unwrap()
    //     );

    //     USER_REWARDS
    //         .range(mut_deps.branch().storage, None, None, Order::Ascending)
    //         .for_each(|val| {
    //             let (user, user_rewards) = val.unwrap();
    //             let user_shares = SHARES.load(mut_deps.branch().storage, user).unwrap();
    //             let mut tmp_rewards = rewards.clone();

    //             tmp_rewards
    //                 .sub_ratio(
    //                     strategist_fee_percentage
    //                 )
    //                 .unwrap();

    //             assert_eq!(
    //                 user_rewards,
    //                 tmp_rewards.ratio(Decimal::from_ratio(user_shares, total))
    //             )
    //         })
    // }

    //     #[test]
    //     fn test_collect_incentives() {
    //         let mut deps = mock_dependencies();
    //         let position = Position { position_id: 1 };
    //         POSITION.save(deps.as_mut().storage, &position).unwrap();
    //         let env = mock_env();

    //         let res = collect_incentives(deps.as_ref(), env.clone()).unwrap();

    //         // Check that the correct message type is returned
    //         assert_eq!(
    //             res,
    //             MsgCollectIncentives {
    //                 position_ids: vec![1], // Check that the correct position_id is included in the message
    //                 sender: env.contract.address.into(),
    //             }
    //         );
    //     }

    //     #[test]
    //     fn test_collect_spread_rewards() {
    //         let mut deps = mock_dependencies();
    //         let position = Position { position_id: 1 };
    //         POSITION.save(deps.as_mut().storage, &position).unwrap();
    //         let env = mock_env();

    //         let res = collect_spread_rewards(deps.as_ref(), env.clone()).unwrap();

    //         // Check that the correct message type is returned
    //         assert_eq!(
    //             res,
    //             MsgCollectSpreadRewards {
    //                 position_ids: vec![1], // Check that the correct position_id is included in the message
    //                 sender: env.contract.address.into(),
    //             }
    //         );
    //     }
}
