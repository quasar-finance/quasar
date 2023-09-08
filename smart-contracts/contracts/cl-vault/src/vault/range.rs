use cosmwasm_schema::cw_serde;
use std::str::FromStr;

use cosmwasm_std::{
    to_binary, Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction, MessageInfo,
    QuerierWrapper, Response, Storage, SubMsg, SubMsgResult, Uint128,
};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
        concentratedliquidity::v1beta1::{
            MsgCreatePosition, MsgCreatePositionResponse, MsgWithdrawPosition,
            MsgWithdrawPositionResponse,
        },
        gamm::v1beta1::MsgSwapExactAmountInResponse,
    },
};

use crate::helpers::round_up_to_nearest_multiple;
use crate::state::CURRENT_SWAP;
use crate::vault::concentrated_liquidity::create_position;
use crate::{
    debug,
    msg::{ExecuteMsg, MergePositionMsg},
};
use crate::{
    helpers::get_spot_price,
    math::tick::price_to_tick,
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, MODIFY_RANGE_STATE, POOL_CONFIG,
        POSITION, RANGE_ADMIN, SWAP_DEPOSIT_MERGE_STATE, VAULT_CONFIG,
    },
    swap::swap,
    vault::concentrated_liquidity::get_position,
    vault::merge::MergeResponse,
    ContractError,
};
use crate::{
    helpers::{
        get_single_sided_deposit_0_to_1_swap_amount, get_single_sided_deposit_1_to_0_swap_amount,
    },
    state::CURRENT_BALANCE,
};

use super::concentrated_liquidity::get_cl_pool_info;

fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

pub fn execute_update_range(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lower_price: Decimal,
    upper_price: Decimal,
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    let storage = deps.storage;
    let querier = deps.querier;

    let lower_tick = price_to_tick(storage, Decimal256::from(lower_price))?;
    let upper_tick = price_to_tick(storage, Decimal256::from(upper_price))?;

    execute_update_range_ticks(
        storage,
        &querier,
        env,
        info,
        lower_tick.try_into().unwrap(),
        upper_tick.try_into().unwrap(),
        max_slippage,
    )
}

/// This function is the entrypoint into the dsm routine that will go through the following steps
/// * how much liq do we have in current range
/// * so how much of each asset given liq would we have at current price
/// * how much of each asset do we need to move to get to new range
/// * deposit up to max liq we can right now, then swap remaining over and deposit again
pub fn execute_update_range_ticks(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    lower_tick: i64,
    upper_tick: i64,
    max_slippage: Decimal,
) -> Result<Response, ContractError> {
    assert_range_admin(storage, &info.sender)?;

    // todo: prevent re-entrancy by checking if we have anything in MODIFY_RANGE_STATE (redundant check but whatever)

    // this will error if we dont have a position anyway
    let position_breakdown = get_position(storage, querier, &env)?;
    let position = position_breakdown.position.unwrap();

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id,
        sender: env.contract.address.to_string(),
        liquidity_amount: Decimal::from_str(position.liquidity.as_str())?
            .atomics()
            .to_string(),
    };

    MODIFY_RANGE_STATE.save(
        storage,
        // todo: should ModifyRangeState be an enum?
        &Some(ModifyRangeState {
            lower_tick,
            upper_tick,
            new_range_position_ids: vec![],
            max_slippage,
        }),
    )?;

    Ok(Response::default()
        .add_submessage(SubMsg::reply_on_success(
            withdraw_msg,
            Replies::WithdrawPosition.into(),
        ))
        .add_attribute("action", "modify_range")
        .add_attribute("method", "withdraw_position")
        .add_attribute("position_id", position.position_id.to_string())
        .add_attribute("liquidity_amount", position.liquidity))
}

// do create new position
pub fn handle_withdraw_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let msg: MsgWithdrawPositionResponse = data.try_into()?;

    // let msg: MsgWithdrawPositionResponse = data.into_result().unwrap().data.unwrap().try_into()?;

    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    debug!(deps, "withdraw_response", msg);
    // what about funds sent to the vault via banksend, what about airdrops/other ways this would not be the total deposited balance
    // todo: Test that one-sided withdraw wouldn't error here (it shouldn't)
    let amount0: Uint128 = msg.amount0.parse()?;
    let amount1: Uint128 = msg.amount1.parse()?;
    debug!(deps, "amounts", vec![amount0, amount1]);

    CURRENT_BALANCE.save(deps.storage, &(amount0, amount1))?;

    let mut tokens_provided = vec![];
    if !amount0.is_zero() {
        tokens_provided.push(OsmoCoin {
            denom: pool_config.token0.clone(),
            amount: amount0.to_string(),
        })
    }
    if !amount1.is_zero() {
        tokens_provided.push(OsmoCoin {
            denom: pool_config.token1.clone(),
            amount: amount1.to_string(),
        })
    }

    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    // if only one token is being deposited, and we are moving into a position where any amount of the other token is needed,
    // creating the position here will fail because liquidityNeeded is calculated as 0 on chain level
    // we can fix this by going straight into a swap-deposit-merge before creating any positions

    // todo: Check if needs LTE or just LT
    if (amount0.is_zero() && modify_range_state.lower_tick < pool_details.current_tick)
        || (amount1.is_zero() && modify_range_state.upper_tick > pool_details.current_tick)
    {
        do_swap_deposit_merge(
            deps,
            env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            (amount0, amount1),
            None, // we just withdrew our only position
        )
    } else {
        // we can naively re-deposit up to however much keeps the proportion of tokens the same. Then swap & re-deposit the proper ratio with the remaining tokens
        let create_position_msg = MsgCreatePosition {
            pool_id: pool_config.pool_id,
            sender: env.contract.address.to_string(),
            // round our lower tick and upper tick up to the nearest pool_details.tick_spacing
            lower_tick: round_up_to_nearest_multiple(
                modify_range_state.lower_tick,
                pool_details
                    .tick_spacing
                    .try_into()
                    .expect("tick spacing is too big to fit into u64"),
            ),
            upper_tick: round_up_to_nearest_multiple(
                modify_range_state.upper_tick,
                pool_details
                    .tick_spacing
                    .try_into()
                    .expect("tick spacing is too big to fit into u64"),
            ),
            tokens_provided,
            // passing 0 is ok here because currently no swap is done on osmosis side, so we don't actually need to worry about slippage impact
            token_min_amount0: "0".to_string(),
            token_min_amount1: "0".to_string(),
        };

        debug!(deps, "create_pos", create_position_msg);

        Ok(Response::new()
            .add_submessage(SubMsg::reply_on_success(
                create_position_msg,
                Replies::RangeInitialCreatePosition.into(),
            ))
            .add_attribute("action", "modify_range")
            .add_attribute("method", "create_position")
            .add_attribute("lower_tick", format!("{:?}", modify_range_state.lower_tick))
            .add_attribute("upper_tick", format!("{:?}", modify_range_state.upper_tick))
            .add_attribute("token0", format!("{:?}{:?}", amount0, pool_config.token0))
            .add_attribute("token1", format!("{:?}{:?}", amount1, pool_config.token1)))
    }
}

// do swap
pub fn handle_initial_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    debug!(deps, "create_pos_response", create_position_message);

    // target range for our imminent swap
    // taking from response message is important because they may differ from the ones in our request
    let target_lower_tick = create_position_message.lower_tick;
    let target_upper_tick = create_position_message.upper_tick;

    // get refunded amounts
    let current_balance = CURRENT_BALANCE.load(deps.storage)?;
    let refunded_amounts = (
        current_balance
            .0
            .checked_sub(Uint128::from_str(&create_position_message.amount0)?)?,
        current_balance
            .1
            .checked_sub(Uint128::from_str(&create_position_message.amount1)?)?,
    );

    do_swap_deposit_merge(
        deps,
        env,
        target_lower_tick,
        target_upper_tick,
        refunded_amounts,
        Some(create_position_message.position_id),
    )
}

/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
pub fn do_swap_deposit_merge(
    mut deps: DepsMut,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
    refunded_amounts: (Uint128, Uint128),
    position_id: Option<u64>,
) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)?;
    if swap_deposit_merge_state.is_some() {
        return Err(ContractError::SwapInProgress {});
    }

    debug!(deps, "oh no", "down bad");

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_config = VAULT_CONFIG.load(deps.storage)?;

    SWAP_DEPOSIT_MERGE_STATE.save(
        deps.storage,
        &SwapDepositMergeState {
            target_lower_tick,
            target_upper_tick,
            target_range_position_ids: if position_id.is_some() {
                vec![position_id.unwrap()]
            } else {
                vec![]
            },
        },
    )?;

    // start swap workflow
    // get pool
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    // get remaining balance in contract for each token (one of these should be zero i think)
    // @notice: this actually works if this function (do_swap_deposit_merge) is called by
    // handle_initial_create_position_reply, double check if implementing it somewhere else
    let (balance0, balance1) = refunded_amounts;

    //TODO: further optimizations can be made by increasing the swap amount by half of our expected slippage,
    // to reduce the total number of non-deposited tokens that we will then need to refund
    let (swap_amount, swap_direction) = if !balance0.is_zero() {
        {
            (
                if pool_details.current_tick > target_upper_tick {
                    balance0
                } else {
                    get_single_sided_deposit_0_to_1_swap_amount(
                        deps.branch(),
                        balance0,
                        target_lower_tick,
                        pool_details.current_tick,
                        target_upper_tick,
                    )?
                },
                SwapDirection::ZeroToOne,
            )
        }
    } else if !balance1.is_zero() {
        (
            if pool_details.current_tick < target_lower_tick {
                balance1
            } else {
                get_single_sided_deposit_1_to_0_swap_amount(
                    deps.storage,
                    &deps.querier,
                    balance1,
                    target_lower_tick,
                    pool_details.current_tick,
                    target_upper_tick,
                )?
            },
            SwapDirection::OneToZero,
        )
    } else {
        // if we have not tokens to swap, that means all tokens we correctly used in the create position
        // this means we can save the position id of the first create_position
        POSITION.save(
            deps.storage,
            &Position {
                // if position not found, then we should panic here anyway ?
                position_id: position_id.expect("position id should be set if no swap is needed"),
            },
        )?;
        return Ok(Response::new()
            .add_attribute("action", "swap_deposit_merge")
            .add_attribute("method", "no_swap")
            .add_attribute("new_position", position_id.unwrap().to_string()));
    };

    // todo check that this math is right with spot price (numerators, denominators) if taken by legacy gamm module instead of poolmanager
    let spot_price = get_spot_price(deps.storage, &deps.querier)?;
    let (token_in_denom, token_out_ideal_amount, left_over_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            pool_config.token0,
            swap_amount.checked_multiply_ratio(spot_price.numerator(), spot_price.denominator()),
            balance0.checked_sub(swap_amount)?,
        ),
        SwapDirection::OneToZero => (
            pool_config.token1,
            swap_amount.checked_multiply_ratio(spot_price.denominator(), spot_price.numerator()),
            balance1.checked_sub(swap_amount)?,
        ),
    };

    CURRENT_SWAP.save(deps.storage, &(swap_direction, left_over_amount))?;

    let token_out_min_amount = token_out_ideal_amount?.checked_multiply_ratio(
        vault_config.swap_max_slippage.numerator(),
        vault_config.swap_max_slippage.denominator(),
    )?;

    let swap_msg = swap(
        &deps.querier,
        deps.storage,
        &env,
        swap_amount,
        &token_in_denom,
        token_out_min_amount,
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_always(swap_msg, Replies::Swap.into()))
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "swap")
        .add_attribute("token_in", format!("{:?}{:?}", swap_amount, token_in_denom))
        .add_attribute("token_out_min", format!("{:?}", token_out_min_amount)))
}

// do deposit
pub fn handle_swap_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // TODO: remove clone
    match data.clone() {
        SubMsgResult::Ok(_msg) => handle_swap_success(deps, env, data.try_into()?),
        SubMsgResult::Err(msg) => Err(ContractError::SwapFailed { message: msg }),
    }
}

fn handle_swap_success(
    deps: DepsMut,
    env: Env,
    resp: MsgSwapExactAmountInResponse,
) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = match SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)? {
        Some(swap_deposit_merge) => swap_deposit_merge,
        None => return Err(ContractError::SwapDepositMergeStateNotFound {}),
    };
    let (swap_direction, left_over_amount) = CURRENT_SWAP.load(deps.storage)?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let _modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();

    // get post swap balances to create positions with

    let (balance0, balance1): (Uint128, Uint128) = match swap_direction {
        SwapDirection::ZeroToOne => (
            left_over_amount,
            Uint128::new(resp.token_out_amount.parse()?),
        ),
        SwapDirection::OneToZero => (
            Uint128::new(resp.token_out_amount.parse()?),
            left_over_amount,
        ),
    };

    // Create the position after swapped the leftovers based on swap direction
    let create_position_msg = create_position(
        deps.storage,
        &env,
        swap_deposit_merge_state.target_lower_tick,
        swap_deposit_merge_state.target_upper_tick,
        vec![
            Coin {
                denom: pool_config.token0.clone(),
                amount: balance0,
            },
            Coin {
                denom: pool_config.token1.clone(),
                amount: balance1,
            },
        ],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_always(
            create_position_msg,
            Replies::RangeIterationCreatePosition.into(),
        ))
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "create_position2")
        .add_attribute(
            "lower_tick",
            format!("{:?}", swap_deposit_merge_state.target_lower_tick),
        )
        .add_attribute(
            "upper_tick",
            format!("{:?}", swap_deposit_merge_state.target_upper_tick),
        )
        .add_attribute("token0", format!("{:?}{:?}", balance0, pool_config.token0))
        .add_attribute("token1", format!("{:?}{:?}", balance1, pool_config.token1)))
}

// do merge position & exit
pub fn handle_iteration_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    let mut swap_deposit_merge_state = match SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)? {
        Some(swap_deposit_merge) => swap_deposit_merge,
        None => return Err(ContractError::SwapDepositMergeStateNotFound {}),
    };

    // add the position id to the ones we need to merge
    swap_deposit_merge_state
        .target_range_position_ids
        .push(create_position_message.position_id);

    // call merge
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: swap_deposit_merge_state.target_range_position_ids.clone(),
        }));
    // merge our position with the main position
    let merge_submsg = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    // clear state to allow for new liquidity movement operations
    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);

    Ok(Response::new()
        .add_submessage(merge_submsg)
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions")
        .add_attribute(
            "position_ids",
            format!("{:?}", swap_deposit_merge_state.target_range_position_ids),
        ))
}

// store new position id and exit
pub fn handle_merge_response(deps: DepsMut, data: SubMsgResult) -> Result<Response, ContractError> {
    let merge_response: MergeResponse = data.try_into()?;

    POSITION.save(
        deps.storage,
        &Position {
            position_id: merge_response.new_position_id,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "swap_deposit_merge")
        .add_attribute("method", "fungify_positions_success")
        .add_attribute("swap_deposit_merge_status", "success")
        .add_attribute("status", "success"))
}

#[cw_serde]
pub enum SwapDirection {
    ZeroToOne,
    OneToZero,
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, str::FromStr};

    use cosmwasm_std::{
        testing::{
            mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
        },
        Addr, Decimal, Empty, MessageInfo, OwnedDeps, SubMsgResponse, SubMsgResult,
    };
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as OsmoCoin,
        osmosis::concentratedliquidity::v1beta1::{
            FullPositionBreakdown, MsgWithdrawPositionResponse, Position as OsmoPosition,
        },
    };

    use crate::{
        state::{
            PoolConfig, VaultConfig, MODIFY_RANGE_STATE, POOL_CONFIG, POSITION, RANGE_ADMIN,
            VAULT_CONFIG,
        },
        test_helpers::QuasarQuerier,
    };

    fn mock_deps_with_querier(
        info: &MessageInfo,
    ) -> OwnedDeps<MockStorage, MockApi, QuasarQuerier, Empty> {
        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: QuasarQuerier::new(
                FullPositionBreakdown {
                    position: Some(OsmoPosition {
                        position_id: 1,
                        address: MOCK_CONTRACT_ADDR.to_string(),
                        pool_id: 1,
                        lower_tick: 100,
                        upper_tick: 1000,
                        join_time: None,
                        liquidity: "1000000.1".to_string(),
                    }),
                    asset0: Some(OsmoCoin {
                        denom: "token0".to_string(),
                        amount: "1000000".to_string(),
                    }),
                    asset1: Some(OsmoCoin {
                        denom: "token1".to_string(),
                        amount: "1000000".to_string(),
                    }),
                    claimable_spread_rewards: vec![
                        OsmoCoin {
                            denom: "token0".to_string(),
                            amount: "100".to_string(),
                        },
                        OsmoCoin {
                            denom: "token1".to_string(),
                            amount: "100".to_string(),
                        },
                    ],
                    claimable_incentives: vec![],
                    forfeited_incentives: vec![],
                },
                500,
            ),
            custom_query_type: PhantomData,
        };

        let storage = &mut deps.storage;

        RANGE_ADMIN.save(storage, &info.sender).unwrap();
        POOL_CONFIG
            .save(
                storage,
                &PoolConfig {
                    pool_id: 1,
                    token0: "token0".to_string(),
                    token1: "token1".to_string(),
                },
            )
            .unwrap();
        VAULT_CONFIG
            .save(
                storage,
                &VaultConfig {
                    performance_fee: Decimal::zero(),
                    treasury: Addr::unchecked("treasure"),
                    swap_max_slippage: Decimal::from_ratio(1u128, 20u128),
                },
            )
            .unwrap();
        POSITION
            .save(storage, &crate::state::Position { position_id: 1 })
            .unwrap();

        deps
    }

    #[test]
    fn test_assert_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        super::assert_range_admin(&mut deps.storage, &info.sender).unwrap();

        let info = mock_info("addr0001", &[]);
        super::assert_range_admin(&mut deps.storage, &info.sender).unwrap_err();

        let info = mock_info("addr0000", &[]);
        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        super::assert_range_admin(&mut deps.storage, &Addr::unchecked("someoneelse")).unwrap_err();
    }

    #[test]
    fn test_get_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        assert_eq!(super::get_range_admin(deps.as_ref()).unwrap(), info.sender);
    }

    #[test]
    fn test_execute_update_range() {
        let info = mock_info("addr0000", &[]);
        let mut deps = mock_deps_with_querier(&info);

        let env = mock_env();
        let lower_price = Decimal::from_str("100").unwrap();
        let upper_price = Decimal::from_str("100.20").unwrap();
        let max_slippage = Decimal::from_str("0.5").unwrap();

        let res = super::execute_update_range(
            deps.as_mut(),
            env,
            info,
            lower_price,
            upper_price,
            max_slippage,
        )
        .unwrap();

        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "modify_range");
        assert_eq!(res.attributes[1].value, "withdraw_position");
        assert_eq!(res.attributes[2].value, "1");
        assert_eq!(res.attributes[3].value, "1000000.1");
    }

    #[test]
    fn test_handle_withdraw_position_reply_selects_correct_next_step_for_new_range() {
        let info = mock_info("addr0000", &[]);
        let mut deps = mock_deps_with_querier(&info);

        // moving into a range
        MODIFY_RANGE_STATE
            .save(
                deps.as_mut().storage,
                &Some(crate::state::ModifyRangeState {
                    lower_tick: 100,
                    upper_tick: 1000, // since both times we are moving into range and in the quasarquerier we configured the current_tick as 500, this would mean we are trying to move into range
                    new_range_position_ids: vec![],
                    max_slippage: Decimal::zero(),
                }),
            )
            .unwrap();

        // Reply
        let env = mock_env();
        //first test fully one-sided withdraw
        let data = SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(
                MsgWithdrawPositionResponse {
                    amount0: "0".to_string(),
                    amount1: "10000".to_string(),
                }
                .try_into()
                .unwrap(),
            ),
        });

        let res = super::handle_withdraw_position_reply(deps.as_mut(), env.clone(), data).unwrap();

        // verify that we went straight to swap_deposit_merge
        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "swap_deposit_merge");
        assert_eq!(res.attributes[1].value, "swap");

        // now test two-sided withdraw
        let data = SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(
                MsgWithdrawPositionResponse {
                    amount0: "10000".to_string(),
                    amount1: "10000".to_string(),
                }
                .try_into()
                .unwrap(),
            ),
        });

        let res = super::handle_withdraw_position_reply(deps.as_mut(), env, data).unwrap();

        // verify that we did create_position first
        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "modify_range");
        assert_eq!(res.attributes[1].value, "create_position");
    }
}
