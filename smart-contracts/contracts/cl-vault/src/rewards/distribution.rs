use cosmwasm_std::{
    Addr, Deps, DepsMut, Env, Fraction, Order, Response, SubMsg, SubMsgResult, Uint128,
};

use crate::{
    debug,
    error::ContractResult,
    reply::Replies,
    state::{
        CURRENT_REWARDS, SHARES, POSITION, STRATEGIST_REWARDS, USER_REWARDS, VAULT_CONFIG,
        VAULT_DENOM,
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

use super::rewards::Rewards;

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn execute_distribute_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    CURRENT_REWARDS.save(deps.storage, &Rewards::new())?;
    let msg = collect_incentives(deps.as_ref(), env)?;

    debug!(deps, "here1", msg);
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
    debug!(deps, "here2", data);
    let response: MsgCollectIncentivesResponse = data.try_into()?;
    CURRENT_REWARDS.update(
        deps.storage,
        |mut rewards| -> Result<Rewards, ContractError> {
            rewards.update_rewards(response.collected_incentives)?;
            Ok(rewards)
        },
    )?;

    debug!(deps, "here4", "");
    // collect the spread rewards
    let msg = collect_spread_rewards(deps.as_ref(), env)?;
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
    debug!(deps, "here3", "");
    // after we have collected both the spread rewards and the incentives, we can distribute them over the share holders
    // we don't need to save the rewards here again, just pass it to update rewards
    let response: MsgCollectSpreadRewardsResponse = data.try_into()?;
    let mut rewards = CURRENT_REWARDS.load(deps.storage)?;
    rewards.update_rewards(response.collected_spread_rewards)?;

    // update the rewards map against all user's locked up vault shares
    distribute_rewards(deps, rewards)?;

    // TODO add a nice response
    Ok(Response::new())
}

fn distribute_rewards(mut deps: DepsMut, mut rewards: Rewards) -> Result<(), ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    // calculate the strategist fee
    let strategist_fee = rewards.sub_percentage(
        vault_config.performance_fee.numerator(),
        vault_config.performance_fee.denominator(),
    )?;
    STRATEGIST_REWARDS.update(deps.storage, |old| old.add(strategist_fee))?;

    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    let total_shares: Uint128 = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();

    // for each user with locked tokens, we distribute some part of the rewards to them
    let user_rewards: Result<Vec<(Addr, Rewards)>, ContractError> = SHARES
        .range(deps.branch().storage, None, None, Order::Ascending)
        .map(|v| -> Result<(Addr, Rewards), ContractError> {
            let (address, user_shares) = v?;
            // calculate the amount of each asset the user should get in rewards
            // we need to always round down here, so we never expect more rewards than we have
            let user_rewards = rewards.percentage(total_shares, user_shares);
            Ok((address, user_rewards))
        })
        .collect();

    user_rewards?
        .into_iter()
        .try_for_each(|(addr, reward)| -> ContractResult<()> {
            USER_REWARDS.update(deps.storage, addr, |old| -> ContractResult<Rewards> {
                if let Some(old_user_rewards) = old {
                    Ok(reward.add(old_user_rewards)?)
                } else {
                    Ok(reward)
                }
            })?;
            Ok(())
        })?;

    Ok(())
}

fn collect_incentives(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

fn collect_spread_rewards(deps: Deps, env: Env) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::{
//         coin,
//         testing::{mock_dependencies, mock_env},
//         Decimal, SubMsgResponse, Uint128,
//     };

//     use crate::state::{Position, VaultConfig};
//     use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;

//     use super::*;

//     #[test]
//     fn test_claim_rewards() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();
//         let position = Position { position_id: 1 };
//         POSITION.save(deps.as_mut().storage, &position).unwrap();

//         let resp = claim_rewards(deps.as_mut(), env.clone()).unwrap();
//         assert_eq!(
//             resp.messages[0].msg,
//             collect_incentives(deps.as_ref(), env).unwrap().into()
//         )
//     }

//     #[test]
//     fn test_handle_collect_rewards() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();
//         let position = Position { position_id: 1 };
//         POSITION.save(deps.as_mut().storage, &position).unwrap();

//         CURRENT_REWARDS
//             .save(deps.as_mut().storage, &Rewards::new())
//             .unwrap();

//         let msg: Binary = MsgCollectIncentivesResponse {
//             collected_incentives: vec![
//                 OsmoCoin {
//                     denom: "uosmo".into(),
//                     amount: "1234".into(),
//                 },
//                 OsmoCoin {
//                     denom: "uqsr".into(),
//                     amount: "2345".into(),
//                 },
//             ],
//             forfeited_incentives: vec![],
//         }
//         .try_into()
//         .unwrap();

//         let resp = handle_collect_incentives_reply(
//             deps.as_mut(),
//             env.clone(),
//             SubMsgResult::Ok(SubMsgResponse {
//                 events: vec![],
//                 data: Some(msg),
//             }),
//         )
//         .unwrap();

//         assert_eq!(
//             resp.messages[0].msg,
//             collect_spread_rewards(deps.as_ref(), env.clone())
//                 .unwrap()
//                 .into()
//         );

//         let msg: Binary = MsgCollectSpreadRewardsResponse {
//             collected_spread_rewards: vec![OsmoCoin {
//                 denom: "uatom".into(),
//                 amount: "3456".into(),
//             }],
//         }
//         .try_into()
//         .unwrap();

//         // we need a vault config to distribute the rewards in the vault config
//         let vault_config = VaultConfig {
//             performance_fee: Decimal::percent(20),
//             treasury: Addr::unchecked("strategy_man"),
//             create_position_max_slippage: Decimal::from_ratio(1u128, 100u128),
//             swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
//         };
//         VAULT_CONFIG
//             .save(deps.as_mut().storage, &vault_config)
//             .unwrap();

//         // mock a vec of user shares
//         let user_shares = vec![(Addr::unchecked("user1"), Uint128::new(1000))];
//         let total = user_shares
//             .iter()
//             .fold(Uint128::zero(), |acc, (_, shares)| acc + shares);
//         LOCKED_TOTAL.save(deps.as_mut().storage, &total).unwrap();
//         user_shares.into_iter().for_each(|(addr, shares)| {
//             SHARES
//                 .save(deps.as_mut().storage, addr, &shares)
//                 .unwrap()
//         });

//         // mock some previous rewards
//         let strategist_rewards = Rewards::from_coins(vec![coin(50, "uosmo")]);
//         STRATEGIST_REWARDS
//             .save(deps.as_mut().storage, &strategist_rewards)
//             .unwrap();

//         let _resp = handle_collect_spread_rewards_reply(
//             deps.as_mut(),
//             env,
//             SubMsgResult::Ok(SubMsgResponse {
//                 events: vec![],
//                 data: Some(msg),
//             }),
//         )
//         .unwrap();

//         // we have collected vec![coin(1234, "uosmo"), coin(2345, "uqsr"), coin(3456, "uatom")] at this point
//         let rewards = Rewards::from_coins(vec![
//             coin(1234, "uosmo"),
//             coin(2345, "uqsr"),
//             coin(3456, "uatom"),
//         ]);

//         assert_eq!(
//             STRATEGIST_REWARDS.load(deps.as_ref().storage).unwrap(),
//             strategist_rewards
//                 .add(
//                     rewards
//                         .clone()
//                         .sub_percentage(
//                             vault_config.performance_fee.numerator(),
//                             vault_config.performance_fee.denominator()
//                         )
//                         .unwrap()
//                 )
//                 .unwrap()
//         );

//         // verify that the distributed rewards make sense
//         let strategist_fee_percentage = VAULT_CONFIG
//             .load(deps.as_ref().storage)
//             .unwrap()
//             .performance_fee;
//         let total_shares = LOCKED_TOTAL.load(deps.as_ref().storage).unwrap();

//         USER_REWARDS
//             .range(deps.as_ref().storage, None, None, Order::Ascending)
//             .for_each(|val| {
//                 let (user, user_rewards) = val.unwrap();
//                 let user_shares = SHARES.load(deps.as_ref().storage, user).unwrap();
//                 let mut tmp_rewards = rewards.clone();

//                 tmp_rewards
//                     .sub_percentage(
//                         strategist_fee_percentage.numerator(),
//                         strategist_fee_percentage.denominator(),
//                     )
//                     .unwrap();

//                 assert_eq!(
//                     user_rewards,
//                     tmp_rewards.percentage(user_shares, total_shares)
//                 )
//             })
//     }

//     #[test]
//     fn distribute_rewards_works() {
//         let mut deps = mock_dependencies();

//         // we need a vault config to distribute the rewards in the vault config
//         VAULT_CONFIG
//             .save(
//                 deps.as_mut().storage,
//                 &VaultConfig {
//                     performance_fee: Decimal::percent(20),
//                     treasury: Addr::unchecked("strategy_man"),
//                     create_position_max_slippage: Decimal::from_ratio(1u128, 100u128),
//                     swap_max_slippage: Decimal::from_ratio(1u128, 100u128),
//                 },
//             )
//             .unwrap();

//         // mock a vec of user shares
//         let user_shares = vec![(Addr::unchecked("user1"), Uint128::new(1000))];
//         let total = user_shares
//             .iter()
//             .fold(Uint128::zero(), |acc, (_, shares)| acc + shares);
//         LOCKED_TOTAL.save(deps.as_mut().storage, &total).unwrap();
//         user_shares.into_iter().for_each(|(addr, shares)| {
//             SHARES
//                 .save(deps.as_mut().storage, addr, &shares)
//                 .unwrap()
//         });

//         let strategist_rewards = Rewards::from_coins(vec![coin(50, "uosmo")]);
//         STRATEGIST_REWARDS
//             .save(deps.as_mut().storage, &strategist_rewards)
//             .unwrap();

//         let rewards = Rewards::from_coins(vec![coin(10000, "uosmo"), coin(1000000, "uatom")]);
//         distribute_rewards(deps.as_mut(), rewards.clone()).unwrap();

//         // each entry in USER_REWARDS should be equal to rewards.sub_percentage(strategist_fee_percentage).percentage(user_shares, total_shares)
//         // we can get the user shares from SHARES
//         let strategist_fee_percentage = VAULT_CONFIG
//             .load(deps.as_ref().storage)
//             .unwrap()
//             .performance_fee;
//         let total_shares = LOCKED_TOTAL.load(deps.as_ref().storage).unwrap();

//         assert_eq!(
//             STRATEGIST_REWARDS.load(deps.as_ref().storage).unwrap(),
//             strategist_rewards
//                 .add(
//                     rewards
//                         .clone()
//                         .sub_percentage(
//                             strategist_fee_percentage.numerator(),
//                             strategist_fee_percentage.denominator()
//                         )
//                         .unwrap()
//                 )
//                 .unwrap()
//         );

//         USER_REWARDS
//             .range(deps.as_ref().storage, None, None, Order::Ascending)
//             .for_each(|val| {
//                 let (user, user_rewards) = val.unwrap();
//                 let user_shares = SHARES.load(deps.as_ref().storage, user).unwrap();
//                 let mut tmp_rewards = rewards.clone();

//                 tmp_rewards
//                     .sub_percentage(
//                         strategist_fee_percentage.numerator(),
//                         strategist_fee_percentage.denominator(),
//                     )
//                     .unwrap();

//                 assert_eq!(
//                     user_rewards,
//                     tmp_rewards.percentage(user_shares, total_shares)
//                 )
//             })
//     }

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
// }
