use cosmwasm_std::{
    to_json_binary, Coin, Decimal256, DepsMut, Env, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::{
    helpers::{get_one_or_two, get_unused_balances},
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    rewards::CoinList,
    state::{CURRENT_POSITION_ID, MAIN_POSITION_ID, POOL_CONFIG, POSITIONS},
    vault::concentrated_liquidity::{create_position, get_position, withdraw_from_position},
    ContractError,
};

/// increase the amount of funds in a position. These funds have to be part of the free funds
/// Any refund is then again ignored
pub fn increase_position_funds(
    deps: DepsMut,
    env: &Env,
    position_id: u64,
    token0: Coin,
    token1: Coin,
) -> Result<Response, ContractError> {
    let current_id = CURRENT_POSITION_ID.save(deps.storage, &position_id)?;
    let position = get_position(&deps.querier, position_id)?.position.unwrap();

    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balances = get_unused_balances(&deps.querier, env)?;
    let (unused0, unused1) = get_one_or_two(&unused_balances.coins(), (pool.token0, pool.token1))?;

    if unused0.amount < token0.amount || unused1.amount < token1.amount {
        return Err(ContractError::InsufficientFunds);
    }

    let create = create_position(
        deps,
        env,
        position.lower_tick,
        position.upper_tick,
        CoinList::from_coins(vec![token0, token1]).coins(),
        Uint128::zero(),
        Uint128::zero(),
    )?;

    // we need to save the position we are adding to and merge the response of this

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        create,
        Replies::RangeAddToPosition as u64,
    )))
}

pub fn handle_range_add_to_position_reply(
    deps: DepsMut,
    env: Env,
    result: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = result.try_into()?;

    let current_id = CURRENT_POSITION_ID.load(deps.storage)?;

    // create the main
    let main_position = current_id == MAIN_POSITION_ID.load(deps.storage)?;

    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: vec![current_id, response.position_id],
            main_position,
        }));

    // merge the positions
    let msg = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_json_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    Ok(Response::new().add_submessage(msg))
}

/// decrease the amount of funds in a position, these funds are returned to the vaults free balance.
/// To completely withdraw a position, use delete position
pub fn decrease_position_funds(
    deps: DepsMut,
    env: &Env,
    position_id: u64,
    liquidity: Decimal256,
) -> Result<Response, ContractError> {
    let msg = withdraw_from_position(env, position_id, liquidity)?;

    Ok(Response::new().add_message(msg))
}
