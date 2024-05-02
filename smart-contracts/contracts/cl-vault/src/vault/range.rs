use crate::{
    helpers::{extract_attribute_value_by_ty_and_key, get_twap_price, get_unused_balances},
    helpers::{
        get_single_sided_deposit_0_to_1_swap_amount, get_single_sided_deposit_1_to_0_swap_amount,
    },
    math::tick::price_to_tick,
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    state::CURRENT_BALANCE,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, CURRENT_SWAP, MODIFY_RANGE_STATE,
        POOL_CONFIG, POSITION, RANGE_ADMIN, SWAP_DEPOSIT_MERGE_STATE,
    },
    vault::{
        concentrated_liquidity::{create_position, get_position},
        merge::MergeResponse,
        swap::swap,
    },
    ContractError,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction, MessageInfo,
    Response, Storage, SubMsg, SubMsgResult, Uint128,
};
use cw_dex_router::operations::SwapOperationsListUnchecked;
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::{
        MsgCreatePositionResponse, MsgWithdrawPosition, MsgWithdrawPositionResponse,
    },
    gamm::v1beta1::MsgSwapExactAmountInResponse,
};
use std::str::FromStr;

use super::{concentrated_liquidity::get_cl_pool_info, swap::SwapParams};

fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

/// This function is the entrypoint into the dsm routine that will go through the following steps
/// * how much liq do we have in current range
/// * so how much of each asset given liq would we have at current price
/// * how much of each asset do we need to move to get to new range
/// * deposit up to max liq we can right now, then swap remaining over and deposit again
#[allow(clippy::too_many_arguments)]
pub fn execute_update_range(
    deps: DepsMut,
    env: &Env,
    info: MessageInfo,
    lower_price: Decimal,
    upper_price: Decimal,
    max_slippage: Decimal,
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
    recommended_swap_route: Option<SwapOperationsListUnchecked>,
    force_swap_route: bool,
    claim_after: Option<u64>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    let lower_tick: i64 = price_to_tick(deps.storage, Decimal256::from(lower_price))?
        .try_into()
        .expect("Overflow when converting lower price to tick");
    let upper_tick: i64 = price_to_tick(deps.storage, Decimal256::from(upper_price))?
        .try_into()
        .expect("Overflow when converting upper price to tick");

    // validate ratio of swappable funds to use
    if ratio_of_swappable_funds_to_use > Decimal::one()
        || ratio_of_swappable_funds_to_use <= Decimal::zero()
    {
        return Err(ContractError::InvalidRatioOfSwappableFundsToUse {});
    }

    let modify_range_config = ModifyRangeState {
        lower_tick,
        upper_tick,
        max_slippage,
        new_range_position_ids: vec![],
        ratio_of_swappable_funds_to_use,
        twap_window_seconds,
        recommended_swap_route,
        force_swap_route,
    };

    execute_update_range_ticks(deps, env, info, modify_range_config, claim_after)
}

/// This function is the entrypoint into the dsm routine that will go through the following steps
/// * how much liq do we have in current range
/// * so how much of each asset given liq would we have at current price
/// * how much of each asset do we need to move to get to new range
/// * deposit up to max liq we can right now, then swap remaining over and deposit again
#[allow(clippy::too_many_arguments)]
pub fn execute_update_range_ticks(
    deps: DepsMut,
    env: &Env,
    info: MessageInfo,
    modify_range_config: ModifyRangeState,
    claim_after: Option<u64>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    // todo: prevent re-entrancy by checking if we have anything in MODIFY_RANGE_STATE (redundant check but whatever)

    // this will error if we dont have a position anyway
    let position_breakdown = get_position(deps.storage, &deps.querier)?;
    let position = position_breakdown.position.unwrap();

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id,
        sender: env.contract.address.to_string(),
        liquidity_amount: Decimal256::from_str(position.liquidity.as_str())?
            .atomics()
            .to_string(),
    };

    MODIFY_RANGE_STATE.save(
        deps.storage,
        // todo: should ModifyRangeState be an enum?
        &Some(modify_range_config),
    )?;

    // Load the current Position to set new join_time and claim_after, leaving current position_id unchanged.
    let position_state = POSITION.load(deps.storage)?;
    POSITION.save(
        deps.storage,
        &Position {
            position_id: position_state.position_id,
            join_time: env.block.time.seconds(),
            claim_after,
        },
    )?;

    Ok(Response::default()
        .add_submessage(SubMsg::reply_on_success(
            withdraw_msg,
            Replies::WithdrawPosition.into(),
        ))
        .add_attribute("method", "execute")
        .add_attribute("action", "update_range_ticks")
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

    // TODO: First we should subtract strategist rewrds and send them out. then follow
    // This with autocompound wouldnt create any math issue because we would then compound those, but no performance_fee for treasury.
    // But as we are pre-autocompound this broke the contraact balance (in positive) leaving idle funds of no one till today.

    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    let mut amount0: Uint128 = msg.amount0.parse()?;
    let mut amount1: Uint128 = msg.amount1.parse()?;

    let unused_balances = get_unused_balances(&deps.querier, &env)?;
    let unused_balance0 = unused_balances
        .find_coin(pool_config.token0.clone())
        .amount
        .saturating_sub(amount0);
    let unused_balance1 = unused_balances
        .find_coin(pool_config.token1.clone())
        .amount
        .saturating_sub(amount1);

    amount0 = amount0.checked_add(unused_balance0)?;
    amount1 = amount1.checked_add(unused_balance1)?;

    CURRENT_BALANCE.save(deps.storage, &(amount0, amount1))?;

    let mut tokens_provided = vec![];
    if !amount0.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token0.clone(),
            amount: amount0,
        })
    }
    if !amount1.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token1.clone(),
            amount: amount1,
        })
    }

    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    // if only one token is being deposited, and we are moving into a position where any amount of the other token is needed,
    // creating the position here will fail because liquidityNeeded is calculated as 0 on chain level
    // we can fix this by going straight into a swap-deposit-merge before creating any positions

    // todo: Check if needs LTE or just LT
    // 0 token0 and current_tick > lower_tick
    // 0 token1 and current_tick < upper_tick
    // if (lower < current < upper) && amount0 == 0  || amount1 == 0
    // also onesided but wrong token
    // bad complexity demon, grug no like
    if (amount0.is_zero() && pool_details.current_tick < modify_range_state.upper_tick)
        || (amount1.is_zero() && pool_details.current_tick > modify_range_state.lower_tick)
    {
        do_swap_deposit_merge(
            deps,
            env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            (amount0, amount1),
            None, // we just withdrew our only position
            modify_range_state.ratio_of_swappable_funds_to_use,
            modify_range_state.twap_window_seconds,
        )
    } else {
        // we can naively re-deposit up to however much keeps the proportion of tokens the same. Then swap & re-deposit the proper ratio with the remaining tokens
        let create_position_msg = create_position(
            deps,
            &env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            tokens_provided,
            Uint128::zero(),
            Uint128::zero(),
        )?;

        Ok(Response::new()
            .add_submessage(SubMsg::reply_on_success(
                create_position_msg,
                Replies::RangeInitialCreatePosition.into(),
            ))
            .add_attribute("method", "reply")
            .add_attribute("action", "handle_withdraw_position")
            .add_attribute("lower_tick", format!("{:?}", modify_range_state.lower_tick))
            .add_attribute("upper_tick", format!("{:?}", modify_range_state.upper_tick))
            .add_attribute("token0", format!("{}{}", amount0, pool_config.token0))
            .add_attribute("token1", format!("{}{}", amount1, pool_config.token1)))
    }
}

// do swap
pub fn handle_initial_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;
    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();

    // target range for our imminent swap
    // taking from response message is important because they may differ from the ones in our request
    let target_lower_tick = create_position_message.lower_tick;
    let target_upper_tick = create_position_message.upper_tick;

    // get refunded amounts
    // TODO added saturating sub as work around for https://github.com/osmosis-labs/osmosis/issues/6843
    // should be a checked sub eventually
    let current_balance = CURRENT_BALANCE.load(deps.storage)?;
    let refunded_amounts = (
        current_balance
            .0
            .saturating_sub(Uint128::from_str(&create_position_message.amount0)?),
        current_balance
            .1
            .saturating_sub(Uint128::from_str(&create_position_message.amount1)?),
    );

    do_swap_deposit_merge(
        deps,
        env,
        target_lower_tick,
        target_upper_tick,
        refunded_amounts,
        Some(create_position_message.position_id),
        modify_range_state.ratio_of_swappable_funds_to_use,
        modify_range_state.twap_window_seconds,
    )
}

/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
#[allow(clippy::too_many_arguments)]
pub fn do_swap_deposit_merge(
    deps: DepsMut,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
    refunded_amounts: (Uint128, Uint128),
    position_id: Option<u64>,
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
) -> Result<Response, ContractError> {
    if SWAP_DEPOSIT_MERGE_STATE.may_load(deps.storage)?.is_some() {
        return Err(ContractError::SwapInProgress {});
    }

    let (balance0, balance1) = (
        refunded_amounts.0.checked_multiply_ratio(
            ratio_of_swappable_funds_to_use.numerator(),
            ratio_of_swappable_funds_to_use.denominator(),
        )?,
        refunded_amounts.1.checked_multiply_ratio(
            ratio_of_swappable_funds_to_use.numerator(),
            ratio_of_swappable_funds_to_use.denominator(),
        )?,
    );

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    let mut target_range_position_ids = vec![];
    if let Some(pos_id) = position_id {
        target_range_position_ids.push(pos_id);
    }

    SWAP_DEPOSIT_MERGE_STATE.save(
        deps.storage,
        &SwapDepositMergeState {
            target_lower_tick,
            target_upper_tick,
            target_range_position_ids,
        },
    )?;

    //TODO: further optimizations can be made by increasing the swap amount by half of our expected slippage,
    // to reduce the total number of non-deposited tokens that we will then need to refund
    let (swap_amount, swap_direction) = if !balance0.is_zero() {
        (
            // range is above current tick
            if pool_details.current_tick > target_upper_tick {
                balance0
            } else {
                get_single_sided_deposit_0_to_1_swap_amount(
                    balance0,
                    target_lower_tick,
                    pool_details.current_tick,
                    target_upper_tick,
                )?
            },
            SwapDirection::ZeroToOne,
        )
    } else if !balance1.is_zero() {
        (
            // current tick is above range
            if pool_details.current_tick < target_lower_tick {
                balance1
            } else {
                get_single_sided_deposit_1_to_0_swap_amount(
                    balance1,
                    target_lower_tick,
                    pool_details.current_tick,
                    target_upper_tick,
                )?
            },
            SwapDirection::OneToZero,
        )
    } else {
        // Load the current Position to extract join_time and claim_after which is unchangeable in this context
        let position = POSITION.load(deps.storage)?;

        // if we have not tokens to swap, that means all tokens we correctly used in the create position
        // this means we can save the position id of the first create_position
        POSITION.save(
            deps.storage,
            &Position {
                // if position not found, then we should panic here anyway ?
                position_id: position_id.expect("position id should be set if no swap is needed"),
                join_time: position.join_time,
                claim_after: position.claim_after,
            },
        )?;

        SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);

        return Ok(Response::new()
            .add_attribute("method", "reply")
            .add_attribute("action", "do_swap_deposit_merge")
            .add_attribute("new_position", position_id.unwrap().to_string()));
    };

    // todo check that this math is right with spot price (numerators, denominators) if taken by legacy gamm module instead of poolmanager
    let twap_price = get_twap_price(deps.storage, &deps.querier, &env, twap_window_seconds)?;
    let (token_in_denom, token_out_denom, token_out_ideal_amount, left_over_amount) =
        match swap_direction {
            SwapDirection::ZeroToOne => (
                pool_config.token0,
                pool_config.token1,
                swap_amount
                    .checked_multiply_ratio(twap_price.numerator(), twap_price.denominator()),
                balance0.checked_sub(swap_amount)?,
            ),
            SwapDirection::OneToZero => (
                pool_config.token1,
                pool_config.token0,
                swap_amount
                    .checked_multiply_ratio(twap_price.denominator(), twap_price.numerator()),
                balance1.checked_sub(swap_amount)?,
            ),
        };

    CURRENT_SWAP.save(deps.storage, &(swap_direction, left_over_amount))?;

    let mrs = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let token_out_min_amount = token_out_ideal_amount?
        .checked_multiply_ratio(mrs.max_slippage.numerator(), mrs.max_slippage.denominator())?;

    let swap_params = SwapParams {
        token_in_amount: swap_amount,
        token_out_min_amount,
        token_in_denom,
        token_out_denom,
        recommended_swap_route: mrs.recommended_swap_route,
        force_swap_route: mrs.force_swap_route,
    };

    let token_in_denom = swap_params.token_in_denom.clone();

    let swap_msg = swap(deps, &env, swap_params)?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(swap_msg, Replies::Swap.into()))
        .add_attribute("method", "reply")
        .add_attribute("action", "do_swap_deposit_merge")
        .add_attribute("token_in", format!("{}{}", swap_amount, token_in_denom))
        .add_attribute("token_out_min", format!("{}", token_out_min_amount)))
}

// do deposit
pub fn handle_swap_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    // TODO: Remove handling of data. if we keep reply_on_success in the caller function
    match data.clone() {
        SubMsgResult::Ok(msg) => {
            let resp: Result<MsgSwapExactAmountInResponse, _> = data.try_into();
            let tokens_out: Result<String, ContractError> = match resp {
                Ok(msg) => Ok(msg.token_out_amount),
                Err(_) => {
                    let tokens_out_opt = extract_attribute_value_by_ty_and_key(
                        &msg.events,
                        "token_swapped",
                        "tokens_out",
                    );

                    match tokens_out_opt {
                        Some(tokens_out) => {
                            let token_out_coin = Coin::from_str(&tokens_out);
                            Ok(token_out_coin?.amount.to_string())
                        }
                        None => {
                            return Err(ContractError::SwapFailed {
                                message: "No tokens_out attribute found in swap response"
                                    .to_string(),
                            })
                        }
                    }
                }
            };
            handle_swap_success(deps, env, tokens_out?)
        }
        SubMsgResult::Err(msg) => Err(ContractError::SwapFailed { message: msg }),
    }
}

fn handle_swap_success(
    deps: DepsMut,
    env: Env,
    tokens_out: String,
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
        SwapDirection::ZeroToOne => (left_over_amount, Uint128::new(tokens_out.parse()?)),
        SwapDirection::OneToZero => (Uint128::new(tokens_out.parse()?), left_over_amount),
    };
    // Create the position after swapped the leftovers based on swap direction
    let mut coins_to_send = vec![];
    if !balance0.is_zero() {
        coins_to_send.push(Coin {
            denom: pool_config.token0.clone(),
            amount: balance0,
        });
    }
    if !balance1.is_zero() {
        coins_to_send.push(Coin {
            denom: pool_config.token1.clone(),
            amount: balance1,
        });
    }
    let create_position_msg = create_position(
        deps,
        &env,
        swap_deposit_merge_state.target_lower_tick,
        swap_deposit_merge_state.target_upper_tick,
        coins_to_send,
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::RangeIterationCreatePosition.into(),
        ))
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_swap_success")
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
            msg: to_json_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    // clear state to allow for new liquidity movement operations
    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);

    Ok(Response::new()
        .add_submessage(merge_submsg)
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_iteration_create_position")
        .add_attribute(
            "position_ids",
            format!("{:?}", swap_deposit_merge_state.target_range_position_ids),
        ))
}

// store new position id and exit
pub fn handle_merge_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let merge_response: MergeResponse = data.try_into()?;

    // Load the current Position to extract join_time and claim_after which is unchangeable in this context
    let position = POSITION.load(deps.storage)?;

    POSITION.save(
        deps.storage,
        &Position {
            position_id: merge_response.new_position_id,
            join_time: env.block.time.seconds(),
            claim_after: position.claim_after,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_merge_reply")
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
    use std::str::FromStr;

    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, Decimal, SubMsgResponse, SubMsgResult,
    };
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgWithdrawPositionResponse;

    use crate::{
        math::tick::build_tick_exp_cache,
        rewards::CoinList,
        state::{MODIFY_RANGE_STATE, RANGE_ADMIN, STRATEGIST_REWARDS},
        test_helpers::{mock_deps_with_querier, mock_deps_with_querier_with_balance},
    };

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
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

        let env = mock_env();
        let lower_price = Decimal::from_str("100").unwrap();
        let upper_price = Decimal::from_str("100.20").unwrap();
        let max_slippage = Decimal::from_str("0.5").unwrap();

        let res = super::execute_update_range(
            deps.as_mut(),
            &env,
            info,
            lower_price,
            upper_price,
            max_slippage,
            Decimal::one(),
            45,
            None,
            false,
            None,
        )
        .unwrap();

        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "execute");
        assert_eq!(res.attributes[1].value, "update_range_ticks");
        assert_eq!(res.attributes[2].value, "1");
        assert_eq!(res.attributes[3].value, "1000000.1");
    }

    #[test]
    fn test_handle_withdraw_position_reply_selects_correct_next_step_for_new_range() {
        let info = mock_info("addr0000", &[]);
        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(MOCK_CONTRACT_ADDR, &[coin(11234, "token1")])],
        );

        STRATEGIST_REWARDS
            .save(
                deps.as_mut().storage,
                &CoinList::from_coins(vec![coin(1000, "token0"), coin(500, "token1")]),
            )
            .unwrap();

        // moving into a range
        MODIFY_RANGE_STATE
            .save(
                deps.as_mut().storage,
                &Some(crate::state::ModifyRangeState {
                    lower_tick: 100,
                    upper_tick: 1000, // since both times we are moving into range and in the quasarquerier we configured the current_tick as 500, this would mean we are trying to move into range
                    new_range_position_ids: vec![],
                    max_slippage: Decimal::zero(),
                    ratio_of_swappable_funds_to_use: Decimal::one(),
                    twap_window_seconds: 45,
                    recommended_swap_route: None,
                    force_swap_route: false,
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
        assert_eq!(res.attributes[1].value, "do_swap_deposit_merge");
        // check that our token1 attribute is incremented with the local balance - strategist rewards
        assert_eq!(
            res.attributes
                .iter()
                .find(|a| { a.key == "token_in" })
                .unwrap()
                .value,
            "5962token1"
        );

        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[coin(11000, "token0"), coin(11234, "token1")],
            )],
        );

        STRATEGIST_REWARDS
            .save(
                deps.as_mut().storage,
                &CoinList::from_coins(vec![coin(1000, "token0"), coin(500, "token1")]),
            )
            .unwrap();

        // moving into a range
        MODIFY_RANGE_STATE
            .save(
                deps.as_mut().storage,
                &Some(crate::state::ModifyRangeState {
                    lower_tick: 100,
                    upper_tick: 1000, // since both times we are moving into range and in the quasarquerier we configured the current_tick as 500, this would mean we are trying to move into range
                    new_range_position_ids: vec![],
                    max_slippage: Decimal::zero(),
                    ratio_of_swappable_funds_to_use: Decimal::one(),
                    twap_window_seconds: 45,
                    recommended_swap_route: None,
                    force_swap_route: false,
                }),
            )
            .unwrap();

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
        assert_eq!(res.attributes[0].value, "reply");
        assert_eq!(res.attributes[1].value, "handle_withdraw_position");
        assert_eq!(
            res.attributes
                .iter()
                .find(|a| { a.key == "token1" })
                .unwrap()
                .value,
            "10734token1"
        ); // 10000 withdrawn + 1234 local balance - 500 rewards
    }
}
