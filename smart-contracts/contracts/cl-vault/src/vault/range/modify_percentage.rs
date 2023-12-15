use cosmwasm_std::{
    to_binary, Coin, Decimal256, DepsMut, Env, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::{
    helpers::{get_one_or_two, get_unused_balances},
    msg::{ExecuteMsg, MergePositionMsg},
    reply::Replies,
    rewards::CoinList,
    state::{Position, CURRENT_POSITION_ID, CURRENT_RATIO, POOL_CONFIG, POSITIONS},
    vault::concentrated_liquidity::{create_position, get_position, withdraw_from_position},
    ContractError,
};

/// increase the ratio that a position has in the vault for depositors
pub fn add_ratio(
    deps: DepsMut,
    position_id: u64,
    old_ratio: Uint128,
    new_ratio: Uint128,
) -> Result<Response, ContractError> {
    // check that our exisiting position is what we are moving
    let position = POSITIONS.load(deps.storage, position_id)?;
    if position.ratio != old_ratio {
        return Err(ContractError::BadOldRatio {
            actual: old_ratio,
            expected: position.ratio,
        });
    }

    if old_ratio >= new_ratio {
        // return error
        return Err(ContractError::IncorrectRatioChange {});
    }

    // set and save the new ratio
    POSITIONS.save(
        deps.storage,
        position.position_id,
        &Position {
            position_id,
            ratio: new_ratio,
        },
    )?;

    Ok(Response::new())
}

/// lower the ratio that a position has in the vault for depositors. To completely remove a position, use delete_position
pub fn lower_ratio(
    deps: DepsMut,
    position_id: u64,
    old_ratio: Uint128,
    new_ratio: Uint128,
) -> Result<Response, ContractError> {
    // check that for sure our exisiting position is what we are moving
    let position = POSITIONS.load(deps.storage, position_id)?;
    if position.ratio != old_ratio {
        return Err(ContractError::BadOldRatio {
            actual: old_ratio,
            expected: position.ratio,
        });
    }

    if old_ratio <= new_ratio {
        // return error
        return Err(ContractError::IncorrectRatioChange {});
    }

    // set and save the new ratio
    POSITIONS.save(
        deps.storage,
        position.position_id,
        &Position {
            position_id,
            ratio: new_ratio,
        },
    )?;

    Ok(Response::new())
}

/// increase the amount of funds in a position. These funds have to be part of the free funds
/// Any refund is then again ignored
pub fn increase_position_funds(
    deps: DepsMut,
    env: Env,
    position_id: u64,
    token0: Coin,
    token1: Coin,
) -> Result<Response, ContractError> {
    let position = get_position(&deps.querier, position_id)?.position.unwrap();
    CURRENT_POSITION_ID.save(deps.storage, &position.position_id)?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balances = get_unused_balances(deps.storage, &deps.querier, &env)?;
    let (unused0, unused1) = get_one_or_two(&unused_balances.coins(), (pool.token0, pool.token1))?;

    if unused0.amount < token0.amount || unused1.amount > token1.amount {
        return Err(ContractError::InsufficientFunds);
    }

    let create = create_position(
        deps,
        &env,
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

pub fn handle_add_to_position_reply(
    deps: DepsMut,
    env: Env,
    result: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = result.try_into()?;

    let current_id = CURRENT_POSITION_ID.load(deps.storage)?;
    let ratio = POSITIONS.load(deps.storage, current_id)?.ratio;

    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::CallbackExecuteMsg(
            crate::msg::CallbackExecuteMsg::Merge(MergePositionMsg {
                position_ids: vec![current_id, response.position_id],
                ratio,
            }),
        ));

    // merge the positions
    let msg = SubMsg::reply_on_success(
        cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&merge_msg)?,
            funds: vec![],
        },
        Replies::Merge.into(),
    );

    Ok(Response::new().add_submessage(msg))
}

/// decrease the amount of funds in a position, these funds are returned to the vaults free balance.
/// To completely withdraw a position, use delete position
pub fn decrease_position_funds(
    _deps: DepsMut,
    env: Env,
    position_id: u64,
    liquidity: Decimal256,
) -> Result<Response, ContractError> {
    let msg = withdraw_from_position(&env, position_id, liquidity)?;

    Ok(Response::new().add_message(msg))
}
