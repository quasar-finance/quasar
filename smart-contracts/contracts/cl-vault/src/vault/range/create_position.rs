use cosmwasm_std::{CosmosMsg, Decimal, DepsMut, Env, Response, SubMsg, SubMsgResult, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::{
    helpers::get_unused_balances,
    math::tick::price_to_tick,
    reply::Replies,
    state::{Position, CURRENT_RATIO, POSITIONS},
    vault::concentrated_liquidity::create_position,
    ContractError,
};

pub fn create_new_position(
    deps: DepsMut,
    env: Env,
    lower_price: Decimal,
    upper_price: Decimal,
    ratio: Uint128,
) -> Result<Response, ContractError> {
    let lower_tick = price_to_tick(deps.storage, lower_price.into())?;
    let upper_tick = price_to_tick(deps.storage, upper_price.into())?;

    // get the current free liquidity
    let tokens = get_unused_balances(deps.storage, &deps.querier, &env)?;

    // create a new position between the given ticks add free liquidity
    let new_position = create_position(
        deps,
        &env,
        lower_tick.try_into().unwrap(),
        upper_tick.try_into().unwrap(),
        tokens.coins(),
        Uint128::zero(),
        Uint128::zero(),
    )?;

    CURRENT_RATIO.save(deps.storage, &ratio)?;

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
    result: SubMsgResult,
) -> Result<Response, ContractError> {
    let v: MsgCreatePositionResponse = result.try_into()?;

    let ratio = CURRENT_RATIO.load(deps.storage)?;

    POSITIONS.save(
        deps.storage,
        v.position_id,
        &Position {
            position_id: v.position_id,
            ratio,
        },
    )?;
    Ok(Response::new())
}
