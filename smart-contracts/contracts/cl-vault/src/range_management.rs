use cosmwasm_std::{Storage, SubMsg, Uint128, Env, Coin};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{MsgWithdrawPosition, MsgWithdrawPositionResponse, MsgCreatePosition};

use crate::{ContractError, concentrated_liquidity::create_position};

pub struct Range {
    lower_tick: i64,
    upper_tick: i64
}

pub fn move_range(storage: &mut dyn Storage) -> Result<SubMsg, ContractError> {
    // query the old range

    // prepare to liquididate the old range

    // in the reply hook after the submsg, we want to create a new position. We might need to swap to support this 
    // new position
    todo!()
}

pub fn handle_remove_position(storage: &mut dyn Storage, env: &Env, response: MsgWithdrawPositionResponse, new_range: Range) -> Result<SubMsg, ContractError> {
    // we need to transform amount0 and amount1 to a Vec<Coin>
    let amount0 = Uint128::new(response.amount0.parse()?);
    let amount1 = Uint128::new(response.amount1.parse()?);
    let tokens_provided = vec![Coin{ denom: todo!(), amount: amount0 }, Coin{ denom: todo!(), amount: amount1 }];

    // the amounts out most likely do not match what we need for the new range, we need to calculate what ratio is expected
    // and how to swap to that ratio. Most likely some funds remain, should we loop positions and merge?
    // Or do we leave dust and include it in the next position movement?

    // after swapping
    // create_position(storage, env, new_range.lower_tick, new_range.upper_tick, tokens_provided, token_min_amount0, token_min_amount1)?
}
