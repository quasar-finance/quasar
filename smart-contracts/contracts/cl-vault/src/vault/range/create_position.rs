use cosmwasm_std::{coin, Decimal, DepsMut, Env, Response, SubMsg, SubMsgResult, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::{
    helpers::{get_one_or_two, get_unused_balances},
    math::tick::price_to_tick,
    reply::Replies,
    rewards::CoinList,
    state::{Position, CURRENT_CLAIM_AFTER_SECS, POOL_CONFIG, POSITIONS},
    vault::concentrated_liquidity::create_position,
    ContractError,
};

/// Create a new position using the free balance in the contract. The entire fee balance will be used
/// to create this position
pub fn create_new_position(
    deps: DepsMut,
    env: &Env,
    lower_price: Decimal,
    upper_price: Decimal,
    max_token0: Option<Uint128>,
    max_token1: Option<Uint128>,
    claim_after_secs: Option<u64>,
) -> Result<Response, ContractError> {
    let lower_tick = price_to_tick(deps.storage, lower_price.into())?;
    let upper_tick = price_to_tick(deps.storage, upper_price.into())?;

    let pool = POOL_CONFIG.load(deps.storage)?;

    CURRENT_CLAIM_AFTER_SECS.save(deps.storage, &claim_after_secs)?;

    // get the current free liquidity
    let tokens = get_unused_balances(&deps.querier, env)?;

    let (token0, token1) = get_one_or_two(&tokens.coins(), (pool.token0, pool.token1))?;

    // if a max has been given, balance0 should be max else it should be token0.amount
    let balance0 = if let Some(max) = max_token0 {
        if token0.amount > max {
            max
        } else {
            token0.amount
        }
    } else {
        token0.amount
    };

    // if a max has been given, balance1 should be max else it should be token1.amount
    let balance1 = if let Some(max) = max_token1 {
        if token1.amount > max {
            max
        } else {
            token1.amount
        }
    } else {
        token1.amount
    };

    // create a new position between the given ticks add free liquidity
    let new_position = create_position(
        deps,
        env,
        lower_tick.try_into().unwrap(),
        upper_tick.try_into().unwrap(),
        CoinList::from_coins(vec![
            coin(balance0.u128(), token0.denom),
            coin(balance1.u128(), token1.denom),
        ])
        .coins(),
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            new_position,
            Replies::RangeNewCreatePosition as u64,
        ))
        .add_attribute("lower_price", lower_price.to_string())
        .add_attribute("upper_price", upper_price.to_string()))
}

pub fn handle_range_new_create_position(
    deps: DepsMut,
    env: Env,
    result: SubMsgResult,
) -> Result<Response, ContractError> {
    let v: MsgCreatePositionResponse = result.try_into()?;

    let claim_after = CURRENT_CLAIM_AFTER_SECS.load(deps.storage)?;

    POSITIONS.save(
        deps.storage,
        v.position_id,
        &Position {
            position_id: v.position_id,
            join_time: env.block.time.seconds(),
            claim_after,
        },
    )?;

    CURRENT_CLAIM_AFTER_SECS.remove(deps.storage);

    Ok(Response::new())
}
