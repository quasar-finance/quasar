use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, from_json, to_json_binary, CosmosMsg, Decimal256, DepsMut, Env, MessageInfo, Response,
    StdError, SubMsg, SubMsgResult, Uint128,
};
use cw_utils::parse_execute_response_data;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, MsgCreatePositionResponse, MsgWithdrawPosition,
    MsgWithdrawPositionResponse,
};

use crate::{
    error::ContractResult,
    msg::MergePositionMsg,
    reply::Replies,
    state::{CurrentMergePosition, CURRENT_MERGE, CURRENT_MERGE_POSITION, POOL_CONFIG},
    vault::concentrated_liquidity::create_position,
    ContractError,
};

#[cw_serde]
pub struct MergeResponse {
    pub new_position_id: u64,
}

pub fn execute_merge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MergePositionMsg,
) -> ContractResult<Response> {
    //check that the sender is our contract
    if env.contract.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut range: Option<CurrentMergePosition> = None;
    // Withdraw all positions
    let withdraw_msgs: ContractResult<Vec<MsgWithdrawPosition>> = msg
        .position_ids
        .into_iter()
        .map(|position_id| {
            let cl_querier = ConcentratedliquidityQuerier::new(&deps.querier);
            let position = cl_querier.position_by_id(position_id)?;
            let p = position.position.unwrap().position.unwrap();

            // if we already have queried a range to seen as "canonical", compare the range of the position
            // and error if they are not the same else we set the value of range. Thus the first queried position is seen as canonical
            if let Some(range) = &range {
                if range.lower_tick != p.lower_tick || range.upper_tick != p.upper_tick {
                    return Err(ContractError::DifferentTicksInMerge);
                }
            } else {
                range = Some(CurrentMergePosition {
                    lower_tick: p.lower_tick,
                    upper_tick: p.upper_tick,
                })
            }

            // save the position as an ongoing withdraw
            // create a withdraw msg to dispatch
            let liquidity_amount = Decimal256::from_str(p.liquidity.as_str())?;

            Ok(MsgWithdrawPosition {
                position_id,
                sender: env.contract.address.to_string(),
                liquidity_amount: liquidity_amount.atomics().to_string(),
            })
        })
        .collect();

    CURRENT_MERGE_POSITION.save(deps.storage, &range.unwrap())?;

    // push all items on the queue
    for msg in withdraw_msgs? {
        CURRENT_MERGE.push_back(deps.storage, &CurrentMergeWithdraw { result: None, msg })?;
    }

    // check the first item and dispatch it
    let current = CURRENT_MERGE.front(deps.storage)?.unwrap();

    // let msg: CosmosMsg = current.msg.into();
    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            current.msg,
            Replies::WithdrawMerge as u64,
        ))
        .add_attribute("method", "merge")
        .add_attribute("action", "merge"))
}

#[cw_serde]
pub struct CurrentMergeWithdraw {
    pub result: Option<WithdrawResponse>,
    pub msg: MsgWithdrawPosition,
}

#[cw_serde]
pub struct WithdrawResponse {
    pub amount0: Uint128,
    pub amount1: Uint128,
}

pub fn handle_merge_withdraw_reply(
    deps: DepsMut,
    env: Env,
    msg: SubMsgResult,
) -> ContractResult<Response> {
    let response: MsgWithdrawPositionResponse = msg.try_into()?;

    // get the corresponding withdraw
    let last = CURRENT_MERGE.pop_front(deps.storage)?.unwrap();

    // mark the current response as finished
    CURRENT_MERGE.push_back(
        deps.storage,
        &CurrentMergeWithdraw {
            result: Some(WithdrawResponse {
                amount0: response.amount0.parse()?,
                amount1: response.amount1.parse()?,
            }),
            msg: last.msg,
        },
    )?;

    let next = CURRENT_MERGE.front(deps.storage)?.unwrap();

    // if next already has a result, we already performed that withdraw
    // so then we empty the entire queue, add all results together and create a new position
    // under the current range with that
    if next.result.is_some() {
        let range = CURRENT_MERGE_POSITION.load(deps.storage)?;
        let (mut amount0, mut amount1) = (Uint128::zero(), Uint128::zero());

        // sum all results in the queue while emptying the queue
        while !CURRENT_MERGE.is_empty(deps.storage)? {
            let w = CURRENT_MERGE
                .pop_front(deps.storage)?
                .unwrap()
                .result
                .unwrap();
            amount0 += w.amount0;
            amount1 += w.amount1;
        }

        let pool: crate::state::PoolConfig = POOL_CONFIG.load(deps.storage)?;

        // amount0 and amount1 can be zero only in the case of a single side position handling
        let mut tokens = vec![];
        if !amount0.is_zero() {
            tokens.push(coin(amount0.into(), pool.token0))
        }
        if !amount1.is_zero() {
            tokens.push(coin(amount1.into(), pool.token1))
        }

        // this is expected to panic if tokens is an empty vec![]
        // tokens should never be an empty vec![] as this would mean that all the current positions
        // are returning zero tokens and this would fail on osmosis side
        let position = create_position(
            deps,
            &env,
            range.lower_tick,
            range.upper_tick,
            tokens,
            Uint128::zero(),
            Uint128::zero(),
        )?;

        Ok(Response::new().add_submessage(SubMsg::reply_on_success(
            position,
            Replies::CreatePositionMerge as u64,
        )))
    } else {
        let msg: CosmosMsg = next.msg.into();

        Ok(Response::new()
            .add_submessage(SubMsg::reply_on_success(msg, Replies::WithdrawMerge as u64))
            .add_attribute("method", "withdraw-position-reply")
            .add_attribute("action", "merge"))
    }
}

pub fn handle_merge_create_position_reply(
    _deps: DepsMut,
    _env: Env,
    msg: SubMsgResult,
) -> ContractResult<Response> {
    let response: MsgCreatePositionResponse = msg.try_into()?;
    // TODO decide if we want any healthchecks here
    Ok(Response::new()
        .set_data(
            to_json_binary(&MergeResponse {
                new_position_id: response.position_id,
            })?
            .0,
        )
        .add_attribute("method", "create-position-reply")
        .add_attribute("action", "merge"))
}

impl TryFrom<SubMsgResult> for MergeResponse {
    type Error = StdError;

    fn try_from(value: SubMsgResult) -> Result<Self, Self::Error> {
        let data = &value
            .into_result()
            .map_err(StdError::generic_err)?
            .data
            .ok_or(StdError::NotFound {
                kind: "MergeResponse".to_string(),
            })?;
        let response = parse_execute_response_data(&data.0).unwrap();
        from_json(response.data.unwrap())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn execute_merge_works() {}

    #[test]
    fn serde_merge_response_is_inverse() {
        let expected = MergeResponse { new_position_id: 5 };

        let data = &to_json_binary(&expected).unwrap();

        let result = from_json(data).unwrap();
        assert_eq!(expected, result)
    }
}
