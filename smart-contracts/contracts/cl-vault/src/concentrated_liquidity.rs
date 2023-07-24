use cosmwasm_std::{CosmosMsg, Storage, Env, Coin, Uint128, Decimal256, QuerierWrapper};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{MsgCreatePosition, MsgWithdrawPosition, PositionByIdRequest, ConcentratedliquidityQuerier, FullPositionBreakdown, MsgFungifyChargedPositions};

use crate::{ContractError, state::{POOL_CONFIG, POSITION}};

pub fn create_position(storage: &mut dyn Storage, env: &Env, lower_tick: i64, upper_tick: i64, tokens_provided: Vec<Coin>, token_min_amount0: Uint128, token_min_amount1: Uint128) -> Result<MsgCreatePosition, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;
    let sender = env.contract.address.to_string();

    let create_position = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender,
        lower_tick,
        upper_tick,
        tokens_provided: tokens_provided.into_iter().map(|c| c.into()).collect(),
        // An sdk.Int in the Go code
        token_min_amount0: token_min_amount0.to_string(),
        // An sdk.Int in the Go code
        token_min_amount1: token_min_amount1.to_string(),
    }
    .into();
    Ok(create_position)
}

// TODO verify that liquidity amount should be Decimal256
pub fn withdraw_from_position(storage: &mut dyn Storage, env: &Env, liquidity_amount: Decimal256) -> Result<MsgWithdrawPosition, ContractError> {
    let sender = env.contract.address.to_string();
    let position = POSITION.load(storage)?;    

    let withdraw_position = MsgWithdrawPosition { position_id: position.position_id, sender, liquidity_amount: liquidity_amount.to_string() };
    Ok(withdraw_position)
}

// merge any newly created user positions with our main position
pub fn merge_positions(storage: &mut dyn Storage, env: &Env, mut position_ids: Vec<u64>) -> Result<MsgFungifyChargedPositions, ContractError> {
    let sender = env.contract.address.to_string();
    let position =POSITION.load(storage)?;

    position_ids.push(position.position_id);    
    let fungify = MsgFungifyChargedPositions { position_ids, sender };
    Ok(fungify)
}

pub fn get_position(storage: &mut dyn Storage, querier: &QuerierWrapper, env: &Env) -> Result<FullPositionBreakdown, ContractError> {
    let position = POSITION.load(storage)?;

    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let position = cl_querier.position_by_id(position.position_id)?;
    position.position.ok_or(ContractError::PositionNotFound)
}
