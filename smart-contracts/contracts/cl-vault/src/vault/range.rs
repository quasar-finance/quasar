use std::str::FromStr;

use cosmwasm_std::{
    Addr, Decimal, Decimal256, Deps, DepsMut, Empty, Env, Fraction, MessageInfo, QuerierWrapper,
    Response, Storage, SubMsg, Uint128,
};
use cw_utils::nonpayable;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
        concentratedliquidity::{
            self,
            v1beta1::{
                MsgCreatePosition, MsgCreatePositionResponse, MsgFungifyChargedPositionsResponse,
                MsgWithdrawPosition, MsgWithdrawPositionResponse,
            },
        },
        gamm::v1beta1::{MsgSwapExactAmountIn, MsgSwapExactAmountInResponse},
        poolmanager::{self, v1beta1::SwapAmountInRoute},
    },
};

use crate::{
    concentrated_liquidity::get_position,
    helpers::{
        get_deposit_amounts_for_liquidity_needed, get_liquidity_needed_for_tokens, get_spot_price,
        with_slippage,
    },
    math::tick::price_to_tick,
    reply::Replies,
    state::{
        ModifyRangeState, Position, SwapDirection, ADMIN_ADDRESS, MODIFY_RANGE_STATE, POOL_CONFIG,
        POSITION, RANGE_ADMIN, VAULT_CONFIG,
    },
    swap::swap,
    ContractError,
};

fn assert_range_admin(storage: &mut dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if &admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

pub fn execute_modify_range(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lower_price: Uint128,
    upper_price: Uint128,
) -> Result<Response, ContractError> {
    // let lower_tick = price_to_tick(price, exponent_at_price_one)

    let storage = deps.storage;
    let querier = deps.querier;

    let lower_tick = price_to_tick(storage, Decimal256::from_atomics(lower_price, 0)?)?;
    let upper_tick = price_to_tick(storage, Decimal256::from_atomics(upper_price, 0)?)?;

    execute_modify_range_ticks(
        storage,
        &querier,
        env,
        info,
        lower_price,
        upper_price,
        lower_tick,
        upper_tick,
    )
}

pub fn execute_modify_range_ticks(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    lower_price: Uint128,
    upper_price: Uint128,
    lower_tick: i128,
    upper_tick: i128,
) -> Result<Response, ContractError> {
    assert_range_admin(storage, &info.sender)?;

    // todo: prevent re-entrancy by checking if we have anything in MODIFY_RANGE_STATE (redundant check but whatever)

    let pool_config = POOL_CONFIG.load(storage)?;
    let vault_config = VAULT_CONFIG.load(storage)?;

    // This function is the entrypoint into the dsm routine that will go through the following steps
    // * how much liq do we have in current range
    // * so how much of each asset given liq would we have at current price
    // * how much of each asset do we need to move to get to new range
    // * deposit up to max liq we can right now, then swap remaining over and deposit again

    // this will error if we dont have a position anyway
    let position_breakdown = get_position(storage, &querier, &env)?;

    let position = match position_breakdown.position {
        Some(position) => position,
        None => {
            // todo: i guess we can support range modification without a position too? then we'd need to check if we have any balance.
            // however line 83 (6 lines up from here) will error if we dont have a position anyway
            return Err(ContractError::PositionNotFound {});
        }
    };

    let withdraw_msg = MsgWithdrawPosition {
        position_id: position.position_id.clone(),
        sender: env.contract.address.to_string(),
        liquidity_amount: position.liquidity.clone(),
    };

    let msg = SubMsg::reply_always(withdraw_msg, Replies::WithdrawPosition.into());

    MODIFY_RANGE_STATE.save(
        storage,
        // todo: should modifyrangestate be an enum?
        &Some(ModifyRangeState {
            lower_tick,
            upper_tick,
            new_range_position_ids: vec![],
        }),
    )?;

    Ok(Response::default()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "withdraw_position")
        .add_attribute("position_id", position.position_id.to_string())
        .add_attribute("liquidity_amount", position.liquidity.to_string()))
}

// do create new position
pub fn handle_withdraw_position_response(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    msg: MsgWithdrawPositionResponse,
) -> Result<Response, ContractError> {
    let mut modify_range_state = match MODIFY_RANGE_STATE.load(storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(storage)?;
    let vault_config = VAULT_CONFIG.load(storage)?;

    let amount0 = msg.amount0;
    let amount1 = msg.amount1;

    // should move this into the reply of withdraw position
    let (liquidity_needed_0, liquidity_needed_1) = get_liquidity_needed_for_tokens(
        amount0.clone(),
        amount1.clone(),
        modify_range_state.lower_tick,
        modify_range_state.upper_tick,
    )?;

    let (deposit, remainders) = get_deposit_amounts_for_liquidity_needed(
        liquidity_needed_0,
        liquidity_needed_1,
        amount0,
        amount1,
    )?;

    // let (swap_amount, swap_direction) = if !remainders.0.is_zero() {
    //     (remainders.0, SwapDirection::ZeroToOne)
    // } else if !remainders.1.is_zero() {
    //     (remainders.1, SwapDirection::OneToZero)
    // } else {
    //     // we shouldn't reach here
    //     (Uint128::zero(), SwapDirection::ZeroToOne)
    // };

    // we can naively re-deposit up to however much keeps the proportion of tokens the same. Then swap & re-deposit the proper ratio with the remaining tokens
    let create_position_msg = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: modify_range_state
            .lower_tick
            .try_into()
            .expect("Could not convert lower_tick to i64 from i128"),
        upper_tick: modify_range_state
            .upper_tick
            .try_into()
            .expect("Could not convert upper_tick to i64 from i128"),
        tokens_provided: vec![
            OsmoCoin {
                denom: pool_config.token0.clone(),
                amount: deposit.0.to_string(),
            },
            OsmoCoin {
                denom: pool_config.token1.clone(),
                amount: deposit.1.to_string(),
            },
        ],
        // slippage is a mis-nomer here, we won't suffer any slippage. but the pool may still return us more of one of the tokens. This is fine.
        token_min_amount0: with_slippage(deposit.0, vault_config.create_position_max_slippage)?
            .to_string(),
        token_min_amount1: with_slippage(deposit.1, vault_config.create_position_max_slippage)?
            .to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(create_position_msg, Replies::CreatePosition.into());

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "create_position")
        .add_attribute("lower_tick", format!("{:?}", modify_range_state.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", modify_range_state.upper_tick))
        .add_attribute("token0", format!("{:?}{:?}", deposit.0, pool_config.token0))
        .add_attribute("token1", format!("{:?}{:?}", deposit.1, pool_config.token1)))
}

// do swap
pub fn handle_create_position_response(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    create_position_message: MsgCreatePositionResponse,
) -> Result<Response, ContractError> {
    let mut modify_range_state = match MODIFY_RANGE_STATE.load(storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(storage)?;
    let vault_config = VAULT_CONFIG.load(storage)?;

    // get remaining balance in contract for each token (one of these should be zero i think)
    let balance0 =
        querier.query_balance(env.contract.address.clone(), pool_config.token0.clone())?;
    let balance1 =
        querier.query_balance(env.contract.address.clone(), pool_config.token1.clone())?;

    let (swap_amount, swap_direction) = if !balance0.amount.is_zero() {
        (balance0.amount, SwapDirection::ZeroToOne)
    } else if !balance1.amount.is_zero() {
        (balance1.amount, SwapDirection::OneToZero)
    } else {
        // we shouldn't reach here
        (Uint128::zero(), SwapDirection::ZeroToOne)
    };

    // todo check that this math is right with spot price (numerators, denominators)
    let spot_price = get_spot_price(storage, querier)?;
    let (token_in_denom, token_out_ideal_amount) = match swap_direction {
        SwapDirection::ZeroToOne => (
            pool_config.token0,
            swap_amount.checked_multiply_ratio(spot_price.numerator(), spot_price.denominator()),
        ),
        SwapDirection::OneToZero => (
            pool_config.token1,
            swap_amount.checked_multiply_ratio(spot_price.denominator(), spot_price.numerator()),
        ),
    };

    let token_out_min_amount = token_out_ideal_amount?.checked_multiply_ratio(
        vault_config.swap_max_slippage.numerator(),
        vault_config.swap_max_slippage.denominator(),
    )?;

    let swap_msg = swap(
        querier,
        storage,
        &env,
        swap_amount,
        &token_in_denom,
        token_out_min_amount,
    )?;

    let msg: SubMsg = SubMsg::reply_always(swap_msg, Replies::Swap.into());

    MODIFY_RANGE_STATE.update(
        storage,
        |mrs| -> Result<Option<ModifyRangeState>, ContractError> {
            Ok(match mrs {
                Some(mut mrs) => {
                    mrs.new_range_position_ids
                        .push(create_position_message.position_id);
                    Some(mrs)
                }
                None => None,
            })
        },
    )?;

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "swap")
        .add_attribute("token_in", format!("{:?}{:?}", swap_amount, token_in_denom))
        .add_attribute("token_out_min", format!("{:?}", token_out_min_amount)))
}

// do deposit
pub fn handle_swap_response(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    msg: MsgSwapExactAmountInResponse,
) -> Result<Response, ContractError> {
    let mut modify_range_state = match MODIFY_RANGE_STATE.load(storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    let pool_config = POOL_CONFIG.load(storage)?;
    let vault_config = VAULT_CONFIG.load(storage)?;

    // get post swap balances to create positions with
    let balance0 =
        querier.query_balance(env.contract.address.clone(), pool_config.token0.clone())?;
    let balance1 =
        querier.query_balance(env.contract.address.clone(), pool_config.token1.clone())?;

    // todo: extract this to a function
    let create_position_msg = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender: env.contract.address.to_string(),
        lower_tick: modify_range_state
            .lower_tick
            .try_into()
            .expect("Could not convert lower_tick to i64 from i128"),
        upper_tick: modify_range_state
            .upper_tick
            .try_into()
            .expect("Could not convert upper_tick to i64 from i128"),
        tokens_provided: vec![
            OsmoCoin {
                denom: pool_config.token0.clone(),
                amount: balance0.amount.to_string(),
            },
            OsmoCoin {
                denom: pool_config.token1.clone(),
                amount: balance1.amount.to_string(),
            },
        ],
        // slippage is a mis-nomer here, we won't suffer any slippage. but the pool may still return us more of one of the tokens. This is fine.
        token_min_amount0: with_slippage(
            balance0.amount,
            vault_config.create_position_max_slippage,
        )?
        .to_string(),
        token_min_amount1: with_slippage(
            balance1.amount,
            vault_config.create_position_max_slippage,
        )?
        .to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(create_position_msg, Replies::CreatePosition.into());

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "create_position2")
        .add_attribute("lower_tick", format!("{:?}", modify_range_state.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", modify_range_state.upper_tick))
        .add_attribute(
            "token0",
            format!("{:?}{:?}", balance0.amount, pool_config.token0),
        )
        .add_attribute(
            "token1",
            format!("{:?}{:?}", balance1.amount, pool_config.token1),
        ))
}

// do merge position & exit
pub fn handle_deposit_response(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    create_position_message: MsgCreatePositionResponse,
) -> Result<Response, ContractError> {
    let mut modify_range_state = match MODIFY_RANGE_STATE.load(storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    modify_range_state
        .new_range_position_ids
        .push(create_position_message.position_id);

    let fungify_positions_msg = concentratedliquidity::v1beta1::MsgFungifyChargedPositions {
        position_ids: modify_range_state.new_range_position_ids.clone(),
        sender: env.contract.address.to_string(),
    };

    let msg: SubMsg = SubMsg::reply_always(fungify_positions_msg, Replies::Fungify.into());

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "modify_range")
        .add_attribute("method", "fungify_positions")
        .add_attribute(
            "position_ids",
            format!("{:?}", modify_range_state.new_range_position_ids),
        ))
}

// store new position id and exit
pub fn handle_fungify_charged_positions_response(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: Env,
    info: MessageInfo,
    fungify_positions_msg: MsgFungifyChargedPositionsResponse,
) -> Result<Response, ContractError> {
    let modify_range_state = match MODIFY_RANGE_STATE.load(storage)? {
        Some(modify_range_state) => modify_range_state,
        None => return Err(ContractError::ModifyRangeStateNotFound {}),
    };

    POSITION.save(
        storage,
        &Position {
            position_id: fungify_positions_msg.new_position_id,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "modify_range")
        .add_attribute("method", "fungify_positions_success")
        .add_attribute("modify_range_status", "success")
        .add_attribute("status", "success")
        .add_attribute(
            "position_ids",
            format!("{:?}", modify_range_state.new_range_position_ids),
        ))
}
