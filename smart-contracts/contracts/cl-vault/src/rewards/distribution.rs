use cosmwasm_std::{BankMsg, DepsMut, Env, Event, Response, StdError, SubMsg, SubMsgResult};
use osmosis_std::try_proto_to_cosmwasm_coins;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCollectIncentivesResponse, MsgCollectSpreadRewardsResponse,
};

use crate::helpers::sort_tokens;
use crate::state::POSITION;
use crate::{reply::Replies, state::VAULT_CONFIG, ContractError};

use super::helpers::CoinList;
use super::{get_collect_incentives_msg, get_collect_spread_rewards_msg};

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn execute_collect_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let msg = get_collect_spread_rewards_msg(deps.as_ref(), env)?;

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "collect_rewards")
        .add_submessage(SubMsg::reply_on_success(
            msg,
            Replies::CollectSpreadRewards as u64,
        )))
}

pub fn handle_collect_spread_rewards_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let data: Result<MsgCollectSpreadRewardsResponse, ContractError> = data
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .map(|b| Ok(b.try_into()?))
        .unwrap_or(Ok(MsgCollectSpreadRewardsResponse {
            collected_spread_rewards: vec![],
        }));

    let response: MsgCollectSpreadRewardsResponse = data?;
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_spread_rewards,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee = response_coin_list.sub_ratio(vault_config.performance_fee)?;

    let mut response = Response::new()
        .add_event(Event::new("cl_collect_spread_rewards"))
        .add_attribute(
            "collected_spread_rewards",
            format!("{:?}", response.clone().collected_spread_rewards),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_spread_rewards");

    // Conditionally add a bank send message if the strategist fee is not empty
    if !strategist_fee.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: vault_config.treasury.to_string(),
            amount: sort_tokens(strategist_fee.coins()),
        };
        response = response.add_message(bank_send_msg);
    }

    // Collect the incentives rewards optional workflow
    let position_state = POSITION.load(deps.storage)?;
    let claim_timestamp = position_state.join_time + position_state.claim_after.unwrap_or_default();

    // If claim_after period expired
    if env.block.time.seconds() > claim_timestamp {
        let msg = get_collect_incentives_msg(deps.as_ref(), env)?;
        // Here, directly update the response without cloning it unnecessarily
        response = response.add_submessage(SubMsg::reply_on_success(
            msg,
            Replies::CollectIncentives as u64,
        ));
    }

    Ok(response)
}

pub fn handle_collect_incentives_reply(
    deps: DepsMut,
    _env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
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
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_incentives,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee: CoinList = response_coin_list.sub_ratio(vault_config.performance_fee)?;

    // Create the base response object
    let mut response = Response::new()
        .add_event(Event::new("cl_collect_incentive"))
        .add_attribute(
            "collected_incentives",
            format!("{:?}", response.clone().collected_incentives),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_incentives");

    // Conditionally add a bank send message if the strategist fee is not empty
    if !strategist_fee.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: vault_config.treasury.to_string(),
            amount: sort_tokens(strategist_fee.coins()),
        };
        response = response.add_message(bank_send_msg);
    }

    Ok(response)
}

// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::{
//         coin,
//         testing::{mock_dependencies, mock_env},
//     };

//     use crate::{state::Position, test_helpers::QuasarQuerier};
//     use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
//         FullPositionBreakdown, Position as OsmoPosition,
//     };

//     use super::*;

// #[test]
// fn test_claim_rewards() {
//     let position_id = 2;
//     let mut deps = mock_dependencies();
//     let _qq = QuasarQuerier::new(
//         FullPositionBreakdown {
//             position: Some(OsmoPosition {
//                 position_id,
//                 address: "bob".to_string(),
//                 pool_id: 1,
//                 lower_tick: 1,
//                 upper_tick: 100,
//                 join_time: None,
//                 liquidity: "123.214".to_string(),
//             }),
//             asset0: Some(coin(1000, "uosmo").into()),
//             asset1: Some(coin(1000, "uosmo").into()),
//             claimable_spread_rewards: vec![coin(1000, "uosmo").into()],
//             claimable_incentives: vec![coin(123, "uatom").into()],
//             forfeited_incentives: vec![],
//         },
//         100,
//     );
//     let env = mock_env();
//     let position = Position { position_id };
//     POSITION.save(deps.as_mut().storage, &position).unwrap();

//     // TODO: implement execute_collect_rewards
//     let collect_resp = execute_collect_rewards(deps.as_mut(), env.clone()).unwrap();
//     assert_eq!(
//         collect_resp.messages[0].msg,
//         get_collect_incentives_msg(deps.as_ref(), env.clone())
//             .unwrap()
//             .into()
//     );

//     let _distribute_resp =
//         execute_distribute_rewards(deps.as_mut(), env, Uint128::new(1u128)).unwrap();

//     // TODO: query is_distributing and assert it is false, and user_rewards empty
// }

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
// }
