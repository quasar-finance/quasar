use std::ops::Sub;

use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, Env, Event, MessageInfo, Order,
    QuerierWrapper, Response, StdError, Storage, SubMsg, SubMsgResult, Uint128, WasmMsg,
};
use cw_dex_router::{
    msg::{BestPathForPairResponse, ExecuteMsg as ApolloExecuteMsg, QueryMsg as ApolloQueryMsg},
    operations::SwapOperationsListUnchecked,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards,
    MsgCollectSpreadRewardsResponse,
};
use osmosis_std::{
    try_proto_to_cosmwasm_coins,
    types::cosmos::bank::v1beta1::{Input, MsgMultiSend, Output},
};

use crate::state::{
    MigrationStatus, AUTO_COMPOUND_ADMIN, MIGRATION_STATUS, POOL_CONFIG, USER_REWARDS,
};
use crate::{
    msg::AutoCompoundAsset,
    reply::Replies,
    state::{POSITION, STRATEGIST_REWARDS, VAULT_CONFIG},
    ContractError,
};

use super::helpers::CoinList;

pub fn execute_migration_step(
    deps: DepsMut,
    env: Env,
    amount_of_users: Uint128,
) -> Result<Response, ContractError> {
    let mut migration_status = MIGRATION_STATUS.load(deps.storage)?;

    if matches!(migration_status, MigrationStatus::Closed) {
        return Err(ContractError::Unauthorized {});
    }

    let mut outputs = Vec::new();
    let mut addresses = Vec::new();
    let mut total_amount = CoinList::new();

    // Iterate user rewards ina paginated fashion
    for item in USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .take(amount_of_users.u128() as usize)
    {
        let (address, rewards) = item?;
        addresses.push(address.clone());
        outputs.push(Output {
            address: address.to_string(),
            coins: rewards.osmo_coin_from_coin_list(),
        });
        total_amount.add(rewards)?;
    }

    let send_message = MsgMultiSend {
        inputs: vec![Input {
            address: env.contract.address.to_string(),
            coins: total_amount.osmo_coin_from_coin_list(),
        }],
        outputs,
    };

    // Remove processed rewards in a separate iteration.
    for addr in addresses {
        USER_REWARDS.remove(deps.storage, addr);
    }

    // Check if this is the last execution.
    let is_last_execution = USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .next()
        .is_none();
    if is_last_execution {
        migration_status = MigrationStatus::Closed;
        MIGRATION_STATUS.save(deps.storage, &migration_status)?;
    }

    Ok(Response::new()
        .add_message(send_message)
        .add_attribute("migration_status", format!("{:?}", migration_status))
        .add_attribute("is_last_execution", is_last_execution.to_string()))
}

/// claim_rewards claims rewards from Osmosis and update the rewards map to reflect each users rewards
pub fn execute_collect_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let msg = get_collect_incentives_msg(deps.as_ref(), env)?;

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "collect_rewards")
        .add_submessage(SubMsg::reply_on_success(
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
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_incentives,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee: CoinList = response_coin_list.sub_ratio(vault_config.performance_fee)?;
    STRATEGIST_REWARDS.update(deps.storage, |mut old: CoinList| {
        old.add(strategist_fee)?;
        Ok::<CoinList, ContractError>(old)
    })?;

    // collect the spread rewards
    let msg = get_collect_spread_rewards_msg(deps.as_ref(), env)?;
    Ok(Response::new()
        .add_event(Event::new("cl_collect_incentive"))
        .add_attribute(
            "collected_incentives",
            format!("{:?}", response.clone().collected_incentives),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_incentives")
        .add_submessage(SubMsg::reply_on_success(
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
    let mut response_coin_list = CoinList::new();
    response_coin_list.merge(try_proto_to_cosmwasm_coins(
        response.clone().collected_spread_rewards,
    )?)?;

    // calculate the strategist fee and remove the share at source
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let strategist_fee = response_coin_list.sub_ratio(vault_config.performance_fee)?;
    STRATEGIST_REWARDS.update(deps.storage, |mut old: CoinList| {
        old.add(strategist_fee)?;
        Ok::<CoinList, ContractError>(old)
    })?;

    // TODO add a nice response
    Ok(Response::new()
        .add_event(Event::new("cl_collect_spread_rewards"))
        .add_attribute(
            "collected_spread_rewards",
            format!("{:?}", response.clone().collected_spread_rewards),
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_collect_spread_rewards"))
}

pub fn execute_auto_compound_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    force_swap_route: bool,
    swap_routes: Vec<AutoCompoundAsset>,
) -> Result<Response, ContractError> {
    // auto compound admin
    assert_auto_compound_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.may_load(deps.storage)?;
    let dex_router = vault_config.unwrap().dex_router;
    if swap_routes.is_empty() {
        return Err(ContractError::EmptyCompoundAssetList {});
    }

    let mut swap_msgs: Vec<SubMsg> = vec![];
    for current_swap_route in swap_routes {
        // sanity check on the amount of tokens
        let strategist_rewards = STRATEGIST_REWARDS.load(deps.storage)?;
        let balance_in_contract = deps.querier.query_balance(
            env.clone().contract.address,
            current_swap_route.clone().token_in_denom,
        )?;
        let balance_remaining_for_swap = balance_in_contract.amount.sub(
            strategist_rewards
                .find_coin(current_swap_route.clone().token_in_denom)
                .amount,
        );
        if balance_remaining_for_swap <= Uint128::zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: balance_in_contract.amount,
                needed: strategist_rewards
                    .find_coin(current_swap_route.clone().token_in_denom)
                    .amount,
            });
        }

        let pool_config = POOL_CONFIG.load(deps.storage)?;

        let part_1_amount = balance_remaining_for_swap
            .checked_div(Uint128::new(2))
            .unwrap();
        let part_2_amount = balance_remaining_for_swap
            .checked_add(Uint128::new(1))
            .unwrap()
            .checked_div(Uint128::new(2))
            .unwrap();

        swap_msgs.push(SubMsg::new(generate_swap_message(
            deps.querier,
            Some(dex_router.clone()),
            current_swap_route.clone().recommended_swap_route_token_0,
            current_swap_route.clone().token_in_denom,
            part_1_amount,
            pool_config.token0,
            force_swap_route,
        )?));
        swap_msgs.push(SubMsg::new(generate_swap_message(
            deps.querier,
            Some(dex_router.clone()),
            current_swap_route.clone().recommended_swap_route_token_1,
            current_swap_route.clone().token_in_denom,
            part_2_amount,
            pool_config.token1,
            force_swap_route,
        )?));
    }

    Ok(Response::new()
        .add_submessages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "auto_compund_swap"))
}

fn generate_swap_message(
    querier: QuerierWrapper,
    dex_router: Option<Addr>,
    current_swap_route: Option<SwapOperationsListUnchecked>,
    token_in_denom: String,
    token_in_amount: Uint128,
    token_out_denom: String,
    force_swap_route: bool,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg = match dex_router {
        Some(ref dex_router_address) => {
            let offer_asset = AssetInfo::Native(token_in_denom.clone());
            let ask_asset = AssetInfo::Native(token_out_denom);

            let recommended_out: Uint128 = match current_swap_route.clone() {
                Some(operations) => querier.query_wasm_smart(
                    dex_router_address.to_string(),
                    &ApolloQueryMsg::SimulateSwapOperations {
                        offer_amount: token_in_amount,
                        operations,
                    },
                )?,
                None => 0u128.into(),
            };
            let best_path: Option<BestPathForPairResponse> = querier.query_wasm_smart(
                dex_router_address.to_string(),
                &ApolloQueryMsg::BestPathForPair {
                    offer_asset: offer_asset.into(),
                    ask_asset: ask_asset.into(),
                    exclude_paths: None,
                    offer_amount: token_in_amount,
                },
            )?;
            let best_outcome = best_path
                .as_ref()
                .map_or(Uint128::zero(), |path| path.return_amount);

            // Determine the route to use
            let route = if force_swap_route {
                current_swap_route
                    .clone()
                    .ok_or(ContractError::TryForceRouteWithoutRecommendedSwapRoute {})?
            } else if best_outcome >= recommended_out {
                best_path.expect("Expected a best path").operations.into()
            } else {
                current_swap_route
                    .clone()
                    .expect("Expected a recommended route")
            };

            // Execute swap operations once with the determined route
            get_execute_swap_operations_msg(
                dex_router_address.clone(),
                route,
                Uint128::zero(),
                &token_in_denom.clone(),
                token_in_amount,
            )
        }
        None => {
            return Err(ContractError::InvalidDexRouterAddress {});
        }
    };

    swap_msg
}

fn get_execute_swap_operations_msg(
    dex_router_address: Addr,
    operations: SwapOperationsListUnchecked,
    token_out_min_amount: Uint128,
    token_in_denom: &String,
    token_in_amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&ApolloExecuteMsg::ExecuteSwapOperations {
            operations,
            minimum_receive: Some(token_out_min_amount),
            to: None,
            offer_amount: None,
        })?,
        funds: vec![Coin {
            denom: token_in_denom.to_string(),
            amount: token_in_amount,
        }],
    }
    .into();

    Ok(swap_msg)
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

fn assert_auto_compound_admin(
    storage: &mut dyn Storage,
    sender: &Addr,
) -> Result<(), ContractError> {
    let admin = AUTO_COMPOUND_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
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
