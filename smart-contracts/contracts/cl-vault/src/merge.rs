use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, from_binary, to_binary, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response,
    StdError, SubMsg, SubMsgResult, Uint128,
};
use cw_utils::{parse_execute_response_data, parse_reply_execute_data};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, MsgWithdrawPosition, MsgWithdrawPositionResponse,
};

use crate::{
    concentrated_liquidity::{create_position, get_position},
    error::ContractResult,
    msg::MergePositionMsg,
    reply::Replies,
    state::{
        CurrentMergePosition, CURRENT_MERGE, CURRENT_MERGE_POSITION, MODIFY_RANGE_STATE,
        POOL_CONFIG,
    },
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
            let position = get_position(deps.storage, &deps.querier, &env)?;
            let p = position.position.unwrap();

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
            let liquidity_amount = Decimal::from_str(p.liquidity.as_str())?;

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

    // pop the first item and dispatch it
    let current = CURRENT_MERGE.front(deps.storage)?.unwrap();

    // let msg: CosmosMsg = current.msg.into();
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        current.msg,
        Replies::WithdrawMerge as u64,
    )))
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
    if let Some(_) = next.result {
        let range = CURRENT_MERGE_POSITION.load(deps.storage)?;
        let (amount0, amount1) = CURRENT_MERGE.iter(deps.storage)?.try_fold(
            (Uint128::zero(), Uint128::zero()),
            |(acc0, acc1), withdraw| -> Result<(Uint128, Uint128), ContractError> {
                let w = withdraw?.result.unwrap();
                Ok((acc0 + w.amount0, acc1 + w.amount1))
            },
        )?;
        let pool: crate::state::PoolConfig = POOL_CONFIG.load(deps.storage)?;

        let mut tokens = vec![];
        if !amount0.is_zero() {
            tokens.push(coin(amount0.into(), pool.token0))
        }
        if !amount1.is_zero() {
            tokens.push(coin(amount1.into(), pool.token1))
        }

        let position = create_position(
            deps.storage,
            &env,
            range.lower_tick as i64,
            range.upper_tick as i64,
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
            .add_submessage(SubMsg::reply_on_success(msg, Replies::WithdrawMerge as u64)))
    }
}

pub fn handle_merge_create_position_reply(
    deps: DepsMut,
    env: Env,
    msg: SubMsgResult,
) -> ContractResult<Response> {
    let response: MsgCreatePositionResponse = msg.try_into()?;
    // TODO decide if we want any healthchecks here
    Ok(Response::new().set_data(
        to_binary(&MergeResponse {
            new_position_id: response.position_id,
        })?
        .0,
    ))
}

impl TryFrom<SubMsgResult> for MergeResponse {
    type Error = StdError;

    fn try_from(value: SubMsgResult) -> Result<Self, Self::Error> {
        let data = &value
            .into_result()
            .map_err(|err| StdError::generic_err(err))?
            .data
            .ok_or(StdError::NotFound {
                kind: "MergeResponse".to_string(),
            })?;
        let response = parse_execute_response_data(&data.0).unwrap();
        from_binary(&response.data.unwrap())
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

        let data = &to_binary(&expected).unwrap();
        println!("{:?}", data);

        let result = from_binary(&data).unwrap();
        assert_eq!(expected, result)
    }
}
