use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, Response, SubMsgResult};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, MsgCreatePosition, MsgWithdrawPosition, MsgCreatePositionResponse,
};

use crate::{concentrated_liquidity::get_position, error::ContractResult, msg::MergePositionMsg};

pub fn execute_merge(deps: DepsMut, env: Env, msg: MergePositionMsg) {
    // save a state entry that we can reuse over executions

    // Withdraw all positions
    let withdraw_msgs: ContractResult<Vec<MsgWithdrawPosition>> = msg
        .position_ids
        .into_iter()
        .map(|position_id| {
            let position = get_position(deps.storage, &deps.querier, &env)?;

            // create a withdraw msg to dispatch
            Ok(MsgWithdrawPosition {
                position_id,
                sender: env.contract.address.to_string(),
                liquidity_amount: position.position.unwrap().liquidity,
            })
        })
        .collect();
}

pub fn handle_withdraw_reply(deps: DepsMut, env: Env, msg: SubMsgResult) {
    // mark the current response as finished

    // create a new pool using all withdrawn funds

    Response::new().add_submessages(MsgCreatePosition {
        pool_id: todo!(),
        sender: todo!(),
        lower_tick: todo!(),
        upper_tick: todo!(),
        tokens_provided: todo!(),
        token_min_amount0: todo!(),
        token_min_amount1: todo!(),
    });
    todo!()
}

pub fn handle_create_position_reply(deps: DepsMut, env: Env, msg: SubMsgResult) -> ContractResult<AdminResponse>{
    let response: MsgCreatePositionResponse = msg.try_into()?;
    // TODO decide if we want any healthchecks here
    Ok(Response::new().set_data(MergeResponse { new_position_id: response.position_id }))
}

#[cw_serde]
pub struct MergeResponse {
    new_position_id: u64,
}
