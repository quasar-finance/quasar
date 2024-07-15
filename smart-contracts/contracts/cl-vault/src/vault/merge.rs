use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, from_json, to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    SubMsg, SubMsgResult, Uint128,
};
use cw_utils::parse_execute_response_data;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, MsgCreatePositionResponse, MsgWithdrawPosition,
    MsgWithdrawPositionResponse,
};

use crate::{
    msg::MergePositionMsg,
    reply::Replies,
    state::{
        CurrentMergePosition, Position, CURRENT_MERGE, CURRENT_MERGE_POSITION, MAIN_POSITION_ID,
        MERGE_MAIN_POSITION, POOL_CONFIG, POSITIONS,
    },
    vault::concentrated_liquidity::create_position,
    ContractError,
};

use super::concentrated_liquidity::get_parsed_position;

#[cw_serde]
pub struct MergeResponse {
    pub new_position_id: u64,
}

pub fn execute_merge_position(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MergePositionMsg,
) -> Result<Response, ContractError> {
    //check that the sender is our contract
    if env.contract.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    MERGE_MAIN_POSITION.save(deps.storage, &msg.main_position)?;

    let mut range: Option<CurrentMergePosition> = None;
    // Withdraw all positions
    let withdraw_msgs: Result<Vec<MsgWithdrawPosition>, ContractError> = msg
        .position_ids
        .into_iter()
        .map(|position_id| {
            let _cl_querier = ConcentratedliquidityQuerier::new(&deps.querier);
            let full_position = get_parsed_position(&deps.querier, position_id)?;
            let p = full_position.position;

            // if we already have queried a range to seen as "canonical", compare the range of the position
            // and error if they are not the same else we set the value of range. Thus the first queried position is seen as canonical
            if let Some(range) = &range {
                if range.lower_tick != p.lower_tick || range.upper_tick != p.upper_tick {
                    return Err(ContractError::DifferentTicksInMerge);
                }
            } else {
                let position = POSITIONS.load(deps.storage, position_id)?;
                range = Some(CurrentMergePosition {
                    lower_tick: p.lower_tick,
                    upper_tick: p.upper_tick,
                    claim_after_secs: position.claim_after,
                })
            }

            POSITIONS.remove(deps.storage, p.position_id);

            // save the position as an ongoing withdraw
            // create a withdraw msg to dispatch
            // TODO we can probably use the concentrated liquidity helper here
            Ok(MsgWithdrawPosition {
                position_id,
                sender: env.contract.address.to_string(),
                liquidity_amount: p.liquidity.atomics().to_string(),
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
        .add_attribute("method", "execute")
        .add_attribute("action", "merge_position"))
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

pub fn handle_merge_withdraw_position_reply(
    deps: DepsMut,
    env: Env,
    msg: SubMsgResult,
) -> Result<Response, ContractError> {
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
            .add_attribute("method", "reply")
            .add_attribute("action", "handle_merge_withdraw_position")
            .add_submessage(SubMsg::reply_on_success(msg, Replies::WithdrawMerge as u64)))
    }
}

pub fn handle_merge_create_position_reply(
    deps: DepsMut,
    env: Env,
    msg: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = msg.try_into()?;

    let is_main_position = MERGE_MAIN_POSITION.load(deps.storage)?;
    if is_main_position {
        MAIN_POSITION_ID.save(deps.storage, &response.position_id)?
    }

    let cur = CURRENT_MERGE_POSITION.load(deps.storage)?;
    POSITIONS.save(
        deps.storage,
        response.position_id,
        &Position {
            position_id: response.position_id,
            join_time: env.block.time.seconds(),
            claim_after: cur.claim_after_secs,
        },
    )?;

    CURRENT_MERGE_POSITION.remove(deps.storage);

    // TODO decide if we want any healthchecks here
    Ok(Response::new()
        .set_data(
            to_json_binary(&MergeResponse {
                new_position_id: response.position_id,
            })?
            .0,
        )
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_merge_create_position")
        .add_attribute("main_position_merged", is_main_position.to_string()))
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
