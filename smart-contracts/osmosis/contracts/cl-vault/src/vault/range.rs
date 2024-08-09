use crate::{
    error::assert_range_admin,
    helpers::getters::{
        get_single_sided_deposit_0_to_1_swap_amount, get_single_sided_deposit_1_to_0_swap_amount,
        get_twap_price, get_unused_pair_balances,
    },
    math::tick::{price_to_tick, tick_to_price},
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, DEX_ROUTER, MODIFY_RANGE_STATE,
        POOL_CONFIG, POSITION, SWAP_DEPOSIT_MERGE_STATE,
    },
    vault::{
        concentrated_liquidity::{
            create_position, get_amount_from_liquidity_for_base_token,
            get_amount_from_liquidity_for_quote_token, get_cl_pool_info,
            get_liquidity_for_base_token, get_liquidity_for_quote_token, get_position,
        },
        swap::{calculate_swap_amount, SwapCalculationResult, SwapDirection},
    },
    ContractError,
};
use cosmwasm_std::{
    attr, coin, Coin, Decimal, Decimal256, DepsMut, Env, MessageInfo, Response, SubMsg,
    SubMsgResult, Uint128, Uint256,
};
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::{MsgCreatePositionResponse, MsgWithdrawPosition},
    poolmanager::v1beta1::SwapAmountInRoute,
};
use std::str::FromStr;

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
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    claim_after: Option<u64>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    let lower_tick: i64 = price_to_tick(deps.storage, lower_price.into())?.try_into()?;
    let upper_tick: i64 = price_to_tick(deps.storage, upper_price.into())?.try_into()?;

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
        forced_swap_route,
    };

    execute_update_range_ticks(deps, env, info, modify_range_config, claim_after)
}

#[allow(clippy::too_many_arguments)]
pub fn execute_update_range_ticks(
    deps: DepsMut,
    env: &Env,
    info: MessageInfo,
    modify_range_config: ModifyRangeState,
    claim_after: Option<u64>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    let position_breakdown = get_position(deps.storage, &deps.querier)?;
    let position = position_breakdown
        .position
        .ok_or(ContractError::MissingPosition {})?;

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id,
        sender: env.contract.address.to_string(),
        liquidity_amount: Decimal256::from_str(position.liquidity.as_str())?
            .atomics()
            .to_string(),
    };

    MODIFY_RANGE_STATE.save(deps.storage, &Some(modify_range_config))?;

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

fn requires_swap(
    sqrt_p: Decimal256,
    sqrt_pl: Decimal256,
    sqrt_pu: Decimal256,
    base_amount: Uint128,
    quote_amount: Uint128,
    base_liquidity: Uint256,
    quote_liquidity: Uint256,
) -> bool {
    if sqrt_p >= sqrt_pu {
        return !base_amount.is_zero();
    }
    if sqrt_p <= sqrt_pl {
        return !quote_amount.is_zero();
    }

    base_liquidity != quote_liquidity
}

pub fn handle_withdraw_position_reply(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;
    let unused_pair_balances = get_unused_pair_balances(&deps, &env, &pool_config)?;

    let sqrt_pu = tick_to_price(modify_range_state.upper_tick)?.sqrt();
    let sqrt_pl = tick_to_price(modify_range_state.lower_tick)?.sqrt();
    let sqrt_p = tick_to_price(pool_details.current_tick)?.sqrt();
    let base_liquidity = get_liquidity_for_base_token(
        unused_pair_balances[0].amount.into(),
        sqrt_p,
        sqrt_pl,
        sqrt_pu,
    )?;
    let quote_liquidity = get_liquidity_for_quote_token(
        unused_pair_balances[1].amount.into(),
        sqrt_p,
        sqrt_pl,
        sqrt_pu,
    )?;

    let response = Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_withdraw_position")
        .add_attribute("lower_tick", modify_range_state.lower_tick.to_string())
        .add_attribute("upper_tick", modify_range_state.upper_tick.to_string())
        .add_attribute("token0", format!("{}", unused_pair_balances[0]))
        .add_attribute("token1", format!("{}", unused_pair_balances[1]));
    if requires_swap(
        sqrt_p,
        sqrt_pl,
        sqrt_pu,
        unused_pair_balances[0].amount,
        unused_pair_balances[1].amount,
        base_liquidity,
        quote_liquidity,
    ) {
        let swap_tokens = if sqrt_p <= sqrt_pl {
            vec![Coin::default(), unused_pair_balances[1].clone()]
        } else if sqrt_p >= sqrt_pu {
            vec![unused_pair_balances[0].clone(), Coin::default()]
        } else if base_liquidity > quote_liquidity {
            let residual_liquidity = base_liquidity - quote_liquidity;
            let residual_amount: Uint128 = get_amount_from_liquidity_for_base_token(
                residual_liquidity,
                sqrt_p,
                sqrt_pl,
                sqrt_pu,
            )?
            .try_into()?;
            vec![
                coin(
                    residual_amount.into(),
                    unused_pair_balances[0].denom.clone(),
                ),
                Coin::default(),
            ]
        } else {
            let residual_liquidity = quote_liquidity - base_liquidity;
            let residual_amount: Uint128 = get_amount_from_liquidity_for_quote_token(
                residual_liquidity,
                sqrt_p,
                sqrt_pl,
                sqrt_pu,
            )?
            .try_into()?;
            vec![
                Coin::default(),
                coin(
                    residual_amount.into(),
                    unused_pair_balances[1].denom.clone(),
                ),
            ]
        };

        SWAP_DEPOSIT_MERGE_STATE.save(
            deps.storage,
            &SwapDepositMergeState {
                target_lower_tick: modify_range_state.lower_tick,
                target_upper_tick: modify_range_state.upper_tick,
                target_range_position_ids: vec![],
            },
        )?;

        let swap_calculation_result = get_swap_calculation_result(
            deps,
            env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            &swap_tokens,
            modify_range_state.ratio_of_swappable_funds_to_use,
            modify_range_state.twap_window_seconds,
        )?;

        Ok(response
            .add_submessage(SubMsg::reply_on_success(
                swap_calculation_result.swap_msg,
                Replies::Swap.into(),
            ))
            .add_attributes(vec![
                attr("token_in", format!("{:?}", swap_calculation_result.offer,)),
                attr(
                    "token_out_min",
                    swap_calculation_result.token_out_min_amount.to_string(),
                ),
            ]))
    } else {
        let create_position_msg = create_position(
            deps,
            &env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            unused_pair_balances.clone(),
            Uint128::zero(),
            Uint128::zero(),
        )?;

        Ok(response.add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::RangeCreatePosition.into(),
        )))
    }
}

/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
#[allow(clippy::too_many_arguments)]
pub fn get_swap_calculation_result(
    deps: DepsMut,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
    tokens_provided: &[Coin],
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
) -> Result<SwapCalculationResult, ContractError> {
    let swap_amounts: Result<Vec<_>, _> = tokens_provided
        .iter()
        .map(|c| c.amount.checked_mul_floor(ratio_of_swappable_funds_to_use))
        .collect();
    let swap_amounts = swap_amounts?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    let mrs = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();

    let (token_in_amount, swap_direction) = if !swap_amounts[0].is_zero() {
        (
            // range is above current tick
            if pool_details.current_tick > target_upper_tick {
                swap_amounts[0]
            } else if pool_details.current_tick < target_lower_tick {
                Uint128::zero()
            } else {
                get_single_sided_deposit_0_to_1_swap_amount(
                    swap_amounts[0],
                    target_lower_tick,
                    pool_details.current_tick,
                    target_upper_tick,
                )?
            },
            SwapDirection::ZeroToOne,
        )
    } else {
        (
            // current tick is above range
            if pool_details.current_tick < target_lower_tick {
                swap_amounts[1]
            } else if pool_details.current_tick > target_upper_tick {
                Uint128::zero()
            } else {
                get_single_sided_deposit_1_to_0_swap_amount(
                    swap_amounts[1],
                    target_lower_tick,
                    pool_details.current_tick,
                    target_upper_tick,
                )?
            },
            SwapDirection::OneToZero,
        )
    };

    let dex_router = DEX_ROUTER.may_load(deps.storage)?;
    let twap_price = get_twap_price(deps.storage, &deps.querier, &env, twap_window_seconds)?;
    let swap_calc_result = calculate_swap_amount(
        env.contract.address,
        pool_config,
        swap_direction,
        token_in_amount,
        mrs.max_slippage,
        mrs.forced_swap_route,
        twap_price,
        dex_router,
    )?;

    Ok(swap_calc_result)
}

// do deposit
pub fn handle_swap_reply(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.load(deps.storage)?;
    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let unused_pair_balances = get_unused_pair_balances(&deps, &env, &pool_config)?;

    let create_position_msg = create_position(
        deps,
        &env,
        swap_deposit_merge_state.target_lower_tick,
        swap_deposit_merge_state.target_upper_tick,
        unused_pair_balances.clone(),
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::RangeCreatePosition.into(),
        ))
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_swap_success")
        .add_attribute(
            "lower_tick",
            swap_deposit_merge_state.target_lower_tick.to_string(),
        )
        .add_attribute(
            "upper_tick",
            swap_deposit_merge_state.target_upper_tick.to_string(),
        )
        .add_attribute(
            "token0",
            format!("{:?}{:?}", unused_pair_balances[0], pool_config.token0),
        )
        .add_attribute(
            "token1",
            format!("{:?}{:?}", unused_pair_balances[1], pool_config.token1),
        ))
}

pub fn handle_create_position(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;
    let position = POSITION.load(deps.storage)?;
    POSITION.save(
        deps.storage,
        &Position {
            position_id: create_position_message.position_id,
            join_time: env.block.time.seconds(),
            claim_after: position.claim_after,
        },
    )?;
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, Decimal,
    };

    use crate::{
        helpers::getters::get_range_admin,
        math::tick::build_tick_exp_cache,
        state::{MODIFY_RANGE_STATE, RANGE_ADMIN},
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

        assert_eq!(get_range_admin(deps.as_ref()).unwrap(), info.sender);
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
        let env = mock_env();

        let mut deps = mock_deps_with_querier_with_balance(
            &info,
            &[(
                MOCK_CONTRACT_ADDR,
                &[coin(11000, "token0"), coin(11234, "token1")],
            )],
        );

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
                    forced_swap_route: None,
                }),
            )
            .unwrap();

        let res = super::handle_withdraw_position_reply(deps.as_mut(), env).unwrap();

        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.attributes[0].value, "reply");
        assert_eq!(res.attributes[1].value, "handle_withdraw_position");
        assert_eq!(
            res.attributes
                .iter()
                .find(|a| { a.key == "token1" })
                .unwrap()
                .value,
            "11234token1"
        ); // 10000 withdrawn + 1234 local balance
    }
}
