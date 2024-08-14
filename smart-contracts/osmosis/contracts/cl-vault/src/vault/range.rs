use crate::{
    helpers::{
        assert::assert_range_admin,
        getters::{
            get_single_sided_deposit_0_to_1_swap_amount,
            get_single_sided_deposit_1_to_0_swap_amount, get_twap_price, get_unused_pair_balances,
        },
    },
    math::tick::price_to_tick,
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDepositMergeState, DEX_ROUTER, MODIFY_RANGE_STATE,
        POOL_CONFIG, POSITION, SWAP_DEPOSIT_MERGE_STATE,
    },
    vault::{
        concentrated_liquidity::{create_position, get_cl_pool_info, get_position},
        merge::MergeResponse,
        swap::calculate_swap_amount,
    },
    ContractError,
};
use cosmwasm_std::{
    attr, coin, to_json_binary, CheckedMultiplyFractionError, Coin, Decimal, Decimal256, DepsMut,
    Env, Fraction, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::{MsgCreatePositionResponse, MsgWithdrawPosition},
    poolmanager::v1beta1::SwapAmountInRoute,
};
use std::str::FromStr;

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
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
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
        forced_swap_route,
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

// do create new position
pub fn handle_withdraw_position_reply(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let modify_range_state = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    let unused_pair_balances = get_unused_pair_balances(&deps, &env, &pool_config)?;

    if (unused_pair_balances[0].amount.is_zero()
        && pool_details.current_tick < modify_range_state.upper_tick)
        || (unused_pair_balances[1].amount.is_zero()
            && pool_details.current_tick > modify_range_state.lower_tick)
    {
        SWAP_DEPOSIT_MERGE_STATE.save(
            deps.storage,
            &SwapDepositMergeState {
                target_lower_tick: modify_range_state.lower_tick,
                target_upper_tick: modify_range_state.upper_tick,
                target_range_position_ids: vec![],
            },
        )?;

        do_swap_deposit_merge(
            deps,
            env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            &unused_pair_balances,
            modify_range_state.ratio_of_swappable_funds_to_use,
            modify_range_state.twap_window_seconds,
        )
    } else {
        // we can naively re-deposit up to however much keeps the proportion of tokens the same.
        // Then swap & re-deposit the proper ratio with the remaining tokens
        let create_position_msg = create_position(
            deps,
            &env,
            modify_range_state.lower_tick,
            modify_range_state.upper_tick,
            unused_pair_balances.clone(),
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
            .add_attribute("lower_tick", modify_range_state.lower_tick.to_string())
            .add_attribute("upper_tick", modify_range_state.upper_tick.to_string())
            .add_attribute("token0", format!("{}", unused_pair_balances[0]))
            .add_attribute("token1", format!("{}", unused_pair_balances[1])))
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
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    // taking from response message is important because they may differ from the ones in our request
    let target_lower_tick = create_position_message.lower_tick;
    let target_upper_tick = create_position_message.upper_tick;

    let unused_pair_balances = get_unused_pair_balances(&deps, &env, &pool_config)?;

    if unused_pair_balances
        .iter()
        .all(|coin: &Coin| coin.amount.is_zero())
    {
        let position = POSITION.load(deps.storage)?;

        // if we have not tokens to swap, that means all tokens we correctly used in the create position
        // this means we can save the position id of the first create_position
        POSITION.save(
            deps.storage,
            &Position {
                position_id: create_position_message.position_id,
                join_time: position.join_time,
                claim_after: position.claim_after,
            },
        )?;

        return Ok(Response::new()
            .add_attribute("method", "reply")
            .add_attribute("action", "handle_initial_create_position_reply")
            .add_attribute(
                "new_position",
                create_position_message.position_id.to_string(),
            ));
    }

    SWAP_DEPOSIT_MERGE_STATE.save(
        deps.storage,
        &SwapDepositMergeState {
            target_lower_tick,
            target_upper_tick,
            target_range_position_ids: vec![create_position_message.position_id],
        },
    )?;

    do_swap_deposit_merge(
        deps,
        env,
        target_lower_tick,
        target_upper_tick,
        &unused_pair_balances,
        modify_range_state.ratio_of_swappable_funds_to_use,
        modify_range_state.twap_window_seconds,
    )
}

/// this function assumes that we are swapping and depositing into a valid range
///
/// It also calculates the exact amount we should be swapping based on current balances and the new range
#[allow(clippy::too_many_arguments)]
fn do_swap_deposit_merge(
    deps: DepsMut,
    env: Env,
    target_lower_tick: i64,
    target_upper_tick: i64,
    tokens_provided: &[Coin],
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
) -> Result<Response, ContractError> {
    let swap_tokens: Result<Vec<_>, _> = tokens_provided
        .iter()
        .map(|c| -> Result<Coin, CheckedMultiplyFractionError> {
            Ok(coin(
                c.amount
                    .checked_mul_floor(ratio_of_swappable_funds_to_use)?
                    .into(),
                c.denom.clone(),
            ))
        })
        .collect();
    let swap_tokens = swap_tokens?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;

    let mrs = MODIFY_RANGE_STATE.load(deps.storage)?.unwrap();
    let twap_price = get_twap_price(deps.storage, &deps.querier, &env, twap_window_seconds)?;
    //TODO: further optimizations can be made by increasing the swap amount by half of our expected slippage,
    // to reduce the total number of non-deposited tokens that we will then need to refund
    let (token_in, out_denom, price) = if !swap_tokens[0].amount.is_zero() {
        (
            // range is above current tick
            if pool_details.current_tick > target_upper_tick {
                swap_tokens[0].clone()
            } else {
                coin(
                    get_single_sided_deposit_0_to_1_swap_amount(
                        swap_tokens[0].amount,
                        target_lower_tick,
                        pool_details.current_tick,
                        target_upper_tick,
                    )?
                    .into(),
                    swap_tokens[0].denom.clone(),
                )
            },
            swap_tokens[1].denom.clone(),
            twap_price,
        )
    } else {
        (
            // current tick is above range
            if pool_details.current_tick < target_lower_tick {
                swap_tokens[1].clone()
            } else {
                coin(
                    get_single_sided_deposit_1_to_0_swap_amount(
                        swap_tokens[1].amount,
                        target_lower_tick,
                        pool_details.current_tick,
                        target_upper_tick,
                    )?
                    .into(),
                    swap_tokens[1].denom.clone(),
                )
            },
            swap_tokens[0].denom.clone(),
            twap_price.inv().expect("Invalid price"),
        )
    };

    let dex_router = DEX_ROUTER.may_load(deps.storage)?;
    let swap_calc_result = calculate_swap_amount(
        env.contract.address,
        pool_config,
        token_in,
        out_denom,
        mrs.max_slippage,
        mrs.forced_swap_route,
        price,
        dex_router,
    )?;

    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "do_swap_deposit_merge")
        .add_submessage(SubMsg::reply_on_success(
            swap_calc_result.swap_msg,
            Replies::Swap.into(),
        ))
        .add_attributes(vec![
            attr("token_in", format!("{:?}", swap_calc_result.token_in)),
            attr(
                "token_out_min",
                swap_calc_result.token_out_min_amount.to_string(),
            ),
        ]))
}

// do deposit
pub fn handle_swap_reply(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.load(deps.storage)?;

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
            Replies::RangeIterationCreatePosition.into(),
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

// do merge position & exit
pub fn handle_iteration_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;
    let mut swap_deposit_merge_state = SWAP_DEPOSIT_MERGE_STATE.load(deps.storage)?;

    // add the position id to the ones we need to merge
    swap_deposit_merge_state
        .target_range_position_ids
        .push(create_position_message.position_id);

    // call merge
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: swap_deposit_merge_state.target_range_position_ids.clone(),
        }));

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
            "11234token1"
        ); // 10000 withdrawn + 1234 local balance
    }
}
