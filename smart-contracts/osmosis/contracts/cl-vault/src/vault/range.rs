use crate::{
    error::{assert_range_admin, assert_ratio},
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
        swap::{estimate_swap_min_out_amount, swap_msg},
    },
    ContractError,
};
use cosmwasm_std::{
    coin, Decimal, Decimal256, DepsMut, Env, Fraction, MessageInfo, Response, StdResult, SubMsg,
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
    assert_ratio(ratio_of_swappable_funds_to_use)?;

    let lower_tick: i64 = price_to_tick(deps.storage, lower_price.into())?.try_into()?;
    let upper_tick: i64 = price_to_tick(deps.storage, upper_price.into())?.try_into()?;

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

    MODIFY_RANGE_STATE.save(
        deps.storage,
        &Some(ModifyRangeState {
            lower_tick,
            upper_tick,
            max_slippage,
            new_range_position_ids: vec![],
            ratio_of_swappable_funds_to_use,
            twap_window_seconds,
            forced_swap_route,
        }),
    )?;

    POSITION.update(deps.storage, |position| -> StdResult<Position> {
        let mut position = position;
        position.join_time = env.block.time.seconds();
        position.claim_after = claim_after;
        Ok(position)
    })?;

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

    let unused_pair_balances = get_unused_pair_balances(&deps.as_ref(), &env, &pool_config)?;

    let sqrt_pu = tick_to_price(modify_range_state.upper_tick)?.sqrt();
    let sqrt_pl = tick_to_price(modify_range_state.lower_tick)?.sqrt();
    let sqrt_p = tick_to_price(pool_details.current_tick)?.sqrt();
    let base_coin = unused_pair_balances[0].clone();
    let quote_coin = unused_pair_balances[1].clone();
    let base_liquidity =
        get_liquidity_for_base_token(base_coin.amount.into(), sqrt_p, sqrt_pl, sqrt_pu)?;
    let quote_liquidity =
        get_liquidity_for_quote_token(quote_coin.amount.into(), sqrt_p, sqrt_pl, sqrt_pu)?;

    let response = Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_withdraw_position")
        .add_attribute("lower_tick", modify_range_state.lower_tick.to_string())
        .add_attribute("upper_tick", modify_range_state.upper_tick.to_string())
        .add_attribute("token0", format!("{}", base_coin))
        .add_attribute("token1", format!("{}", quote_coin));

    if requires_swap(
        sqrt_p,
        sqrt_pl,
        sqrt_pu,
        base_coin.amount,
        quote_coin.amount,
        base_liquidity,
        quote_liquidity,
    ) {
        let twap_price = get_twap_price(
            &deps.querier,
            env.block.time,
            modify_range_state.twap_window_seconds,
            pool_config.pool_id,
            pool_config.token0,
            pool_config.token1,
        )?;
        let (token_in, out_denom, price) = if sqrt_p <= sqrt_pl {
            (
                quote_coin,
                base_coin.denom,
                twap_price.inv().expect("Invalid price"),
            )
        } else if sqrt_p >= sqrt_pu {
            (base_coin, quote_coin.denom, twap_price)
        } else if base_liquidity > quote_liquidity {
            let used_base_amount: Uint128 = get_amount_from_liquidity_for_base_token(
                quote_liquidity,
                sqrt_p,
                sqrt_pl,
                sqrt_pu,
            )?
            .try_into()?;
            let residual_amount = base_coin.amount.checked_sub(used_base_amount)?;
            let swap_amount = get_single_sided_deposit_0_to_1_swap_amount(
                residual_amount,
                modify_range_state.lower_tick,
                pool_details.current_tick,
                modify_range_state.upper_tick,
            )?;
            (
                coin(swap_amount.into(), base_coin.denom),
                quote_coin.denom,
                twap_price,
            )
        } else {
            let used_quote_amount = get_amount_from_liquidity_for_quote_token(
                base_liquidity,
                sqrt_p,
                sqrt_pl,
                sqrt_pu,
            )?
            .try_into()?;
            let residual_amount = quote_coin.amount.checked_sub(used_quote_amount)?;
            let swap_amount = get_single_sided_deposit_1_to_0_swap_amount(
                residual_amount,
                modify_range_state.lower_tick,
                pool_details.current_tick,
                modify_range_state.upper_tick,
            )?;
            (
                coin(swap_amount.into(), quote_coin.denom),
                base_coin.denom,
                twap_price.inv().expect("Invalid price"),
            )
        };

        SWAP_DEPOSIT_MERGE_STATE.save(
            deps.storage,
            &SwapDepositMergeState {
                target_lower_tick: modify_range_state.lower_tick,
                target_upper_tick: modify_range_state.upper_tick,
                target_range_position_ids: vec![],
            },
        )?;

        let token_out_min_amount =
            estimate_swap_min_out_amount(token_in.amount, price, modify_range_state.max_slippage)?;

        let dex_router = DEX_ROUTER.may_load(deps.storage)?;
        let swap_msg = swap_msg(
            env.contract.address,
            pool_config.pool_id,
            token_in.clone(),
            coin(token_out_min_amount.into(), out_denom.clone()),
            None, // TODO: check this None
            dex_router,
        )?;
        Ok(response
            .add_submessage(SubMsg::reply_on_success(swap_msg, Replies::Swap.into()))
            .add_attribute("action", "do_swap_deposit_merge")
            .add_attribute("token_in", format!("{}", token_in))
            .add_attribute("token_out_min", token_out_min_amount.to_string()))
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
            Replies::CreatePosition.into(),
        )))
    }
}

pub fn handle_swap_reply(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.load(deps.storage)?;
    SWAP_DEPOSIT_MERGE_STATE.remove(deps.storage);
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let unused_pair_balances = get_unused_pair_balances(&deps.as_ref(), &env, &pool_config)?;

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
            Replies::CreatePosition.into(),
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
        .add_attribute("token0", format!("{}", unused_pair_balances[0]))
        .add_attribute("token1", format!("{}", unused_pair_balances[1])))
}

pub fn handle_create_position(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    POSITION.update(deps.storage, |position| -> StdResult<Position> {
        let mut position = position;
        position.position_id = create_position_message.position_id;
        position.join_time = env.block.time.seconds();
        Ok(position)
    })?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, Decimal, Decimal256, Uint128, Uint256,
    };

    use crate::{
        helpers::getters::get_range_admin,
        math::tick::build_tick_exp_cache,
        state::{MODIFY_RANGE_STATE, RANGE_ADMIN},
        test_helpers::{
            instantiate_contract, mock_deps_with_querier, mock_deps_with_querier_with_balance,
            BASE_DENOM, POSITION_ID, QUOTE_DENOM,
        },
        vault::range::requires_swap,
    };

    #[test]
    fn test_get_range_admin() {
        let mut deps = mock_dependencies();
        let info = mock_info("addr0000", &[]);

        RANGE_ADMIN.save(&mut deps.storage, &info.sender).unwrap();

        assert_eq!(get_range_admin(deps.as_ref()).unwrap(), info.sender);
    }

    #[test]
    fn test_execute_update_range() {
        let range_admin = "range_admin".to_string();
        let mut deps = mock_deps_with_querier();
        let env = mock_env();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();
        instantiate_contract(deps.as_mut(), env.clone(), &range_admin);
        RANGE_ADMIN
            .save(deps.as_mut().storage, &Addr::unchecked(range_admin.clone()))
            .unwrap();

        let lower_price = Decimal::from_str("100").unwrap();
        let upper_price = Decimal::from_str("100.20").unwrap();
        let max_slippage = Decimal::from_str("0.5").unwrap();

        let info = mock_info(&range_admin, &[]);
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
        assert_eq!(res.attributes[2].value, POSITION_ID.to_string());
        assert_eq!(res.attributes[3].value, "1000000.1");
    }

    #[test]
    fn test_handle_withdraw_position_reply_selects_correct_next_step_for_new_range() {
        let info = mock_info("addr0000", &[]);
        let env = mock_env();

        let mut deps = mock_deps_with_querier_with_balance(&[(
            MOCK_CONTRACT_ADDR,
            &[coin(11000, BASE_DENOM), coin(11234, QUOTE_DENOM)],
        )]);
        instantiate_contract(deps.as_mut(), env.clone(), info.sender.as_str());

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
            format!("11234{}", QUOTE_DENOM)
        ); // 10000 withdrawn + 1234 local balance
    }

    #[test]
    fn test_when_price_is_below_range_and_quote_amount_is_zero_then_no_swap_is_required() {
        let sqrt_p = Decimal256::one();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::one();
        let quote_amount = Uint128::zero();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::one();

        assert!(!requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }

    #[test]
    fn test_when_price_is_below_range_and_quote_amount_is_not_zero_then_swap_is_required() {
        let sqrt_p = Decimal256::one();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::one();
        let quote_amount = Uint128::one();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::one();

        assert!(requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }

    #[test]
    fn test_when_price_is_above_range_and_base_amount_is_zero_then_no_swap_is_required() {
        let sqrt_p = Decimal256::from_str("4.0").unwrap();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::zero();
        let quote_amount = Uint128::one();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::one();

        assert!(!requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }

    #[test]
    fn test_when_price_is_above_range_and_base_amount_is_not_zero_then_swap_is_required() {
        let sqrt_p = Decimal256::from_str("4.0").unwrap();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::one();
        let quote_amount = Uint128::one();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::one();

        assert!(requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }

    #[test]
    fn test_when_price_is_in_range_and_base_liquidity_differs_from_quote_liquidity_then_swap_is_required(
    ) {
        let sqrt_p = Decimal256::from_str("2.5").unwrap();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::one();
        let quote_amount = Uint128::one();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::from(2u32);

        assert!(requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }

    #[test]
    fn test_when_price_is_in_range_and_base_liquidity_equals_quote_liquidity_then_no_swap_is_required(
    ) {
        let sqrt_p = Decimal256::from_str("2.5").unwrap();
        let sqrt_pl = Decimal256::from_str("2.0").unwrap();
        let sqrt_pu = Decimal256::from_str("3.0").unwrap();
        let base_amount = Uint128::one();
        let quote_amount = Uint128::one();
        let base_liquidity = Uint256::one();
        let quote_liquidity = Uint256::one();

        assert!(!requires_swap(
            sqrt_p,
            sqrt_pl,
            sqrt_pu,
            base_amount,
            quote_amount,
            base_liquidity,
            quote_liquidity
        ));
    }
}
