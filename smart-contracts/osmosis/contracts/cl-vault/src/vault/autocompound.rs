use cosmwasm_std::{
    to_json_binary, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::helpers::getters::get_unused_pair_balances;
use crate::msg::{ExecuteMsg, MergePositionMsg};
use crate::reply::Replies;
use crate::state::{Position, POOL_CONFIG, POSITION};
use crate::vault::{concentrated_liquidity::create_position, merge::MergeResponse};
use crate::ContractError;

pub fn execute_autocompound(
    deps: DepsMut,
    env: &Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let position_state = POSITION.load(deps.storage)?;

    // If the position claim after timestamp is not reached yet, return an error
    if position_state.claim_after.is_some()
        && position_state.claim_after.unwrap() <= env.block.time.seconds()
    {
        return Err(ContractError::ClaimAfterNotExpired {});
    }

    let position = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position_state.position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let pool = POOL_CONFIG.load(deps.storage)?;
    let balance = get_unused_pair_balances(&deps, env, &pool)?;

    let token0 = balance[0].clone();
    let token1 = balance[1].clone();

    // Create coins_to_send with no zero amounts
    let mut coins_to_send = vec![];
    if !token0.amount.is_zero() {
        coins_to_send.push(token0.clone());
    }
    if !token1.amount.is_zero() {
        coins_to_send.push(token1.clone());
    }

    let create_position_msg = create_position(
        deps,
        env,
        position.lower_tick,
        position.upper_tick,
        coins_to_send,
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::Autocompound as u64,
        ))
        .add_attribute("method", "execute")
        .add_attribute("action", "autocompound")
        .add_attribute("lower_tick", format!("{:?}", position.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", position.upper_tick))
        .add_attribute("token0", format!("{:?}", token0.clone()))
        .add_attribute("token1", format!("{:?}", token1.clone())))
}

pub fn handle_autocompound_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    // set claim after
    let position_id = (POSITION.load(deps.storage)?).position_id;
    // call merge
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: vec![position_id, create_position_message.position_id],
        }));

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            cosmwasm_std::WasmMsg::Execute {
                contract_addr: env.contract.address.to_string(),
                msg: to_json_binary(&merge_msg)?,
                funds: vec![],
            },
            Replies::Merge.into(),
        ))
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_autocompound")
        .add_attribute(
            "position_ids",
            format!("{:?}", vec![create_position_message.position_id]),
        ))
}

pub fn handle_merge_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let merge_response: MergeResponse = data.try_into()?;

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
