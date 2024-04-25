use crate::error::ContractResult;
use crate::helpers::{get_unused_balances, must_pay_one_or_two_from_balance};
use crate::msg::{ExecuteMsg, MergePositionMsg};
use crate::reply::Replies;
use crate::state::{POOL_CONFIG, POSITION};
use crate::vault::concentrated_liquidity::create_position;
use crate::ContractError;
use cosmwasm_std::{
    to_json_binary, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

/// Execute the redeposit process, creating a new position with unused balances.
///
/// # Arguments
///
/// * `deps` - Dependencies for interacting with the contract.
/// * `env` - Environment for fetching contract address.
/// * `_info` - Message information (not used).
///
/// # Errors
///
/// Returns a `ContractError` if the operation fails.
///
/// # Returns
///
/// Returns a `Response` containing the result of the redeposit operation.
pub fn execute_redeposit(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let position_id = (POSITION.load(deps.storage)?).position_id;
    let position = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let balance = get_unused_balances(deps.storage, &deps.querier, &env).unwrap();
    let pool = POOL_CONFIG.load(deps.storage)?;
    let (token0, token1) =
        must_pay_one_or_two_from_balance(balance.coins(), (pool.token0, pool.token1))?;

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
        &env,
        position.lower_tick,
        position.upper_tick,
        coins_to_send,
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::Redeposit as u64,
        ))
        .add_attribute("method", "execute")
        .add_attribute("action", "redeposit")
        .add_attribute("lower_tick", format!("{:?}", position.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", position.upper_tick))
        .add_attribute("token0", format!("{:?}", token0.clone()))
        .add_attribute("token1", format!("{:?}", token1.clone())))
}

/// Handle the reply from the redeposit operation and then calling merge position
/// on the newly created position.
///
/// # Arguments
///
/// * `deps` - Dependencies for interacting with the contract.
/// * `env` - Environment for fetching contract address.
/// * `data` - Result of the redeposit operation.
///
/// # Errors
///
/// Returns a `ContractError` if the operation fails.
///
/// # Returns
///
/// Returns a `Response` containing the result of the merge operation.
pub fn handle_redeposit_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
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
        .add_attribute("action", "handle_redeposit_reply")
        .add_attribute(
            "position_ids",
            format!("{:?}", vec![create_position_message.position_id]),
        ))
}
