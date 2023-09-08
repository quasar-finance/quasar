use cosmwasm_std::{Coin, Decimal256, Env, QuerierWrapper, Storage, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, FullPositionBreakdown, MsgCreatePosition, MsgWithdrawPosition,
    Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use prost::Message;

use crate::{
    state::{POOL_CONFIG, POSITION},
    ContractError,
};

pub fn create_position(
    storage: &mut dyn Storage,
    env: &Env,
    lower_tick: i64,
    upper_tick: i64,
    tokens_provided: Vec<Coin>,
    token_min_amount0: Uint128,
    token_min_amount1: Uint128,
) -> Result<MsgCreatePosition, ContractError> {
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
    };
    Ok(create_position)
}

// TODO verify that liquidity amount should be Decimal256
pub fn withdraw_from_position(
    storage: &dyn Storage,
    env: &Env,
    liquidity_amount: Decimal256,
) -> Result<MsgWithdrawPosition, ContractError> {
    let sender = env.contract.address.to_string();
    let position = POSITION.load(storage)?;

    let withdraw_position = MsgWithdrawPosition {
        position_id: position.position_id,
        sender,
        liquidity_amount: liquidity_amount.atomics().to_string(),
    };
    Ok(withdraw_position)
}

pub fn get_position(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    _env: &Env,
) -> Result<FullPositionBreakdown, ContractError> {
    let position = POSITION.load(storage)?;

    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let position = cl_querier.position_by_id(position.position_id)?;
    position.position.ok_or(ContractError::PositionNotFound)
}

pub fn get_cl_pool_info(querier: &QuerierWrapper, pool_id: u64) -> Result<Pool, ContractError> {
    let pm_querier = PoolmanagerQuerier::new(querier);
    let pool = pm_querier.pool(pool_id)?;

    match pool.pool {
        // Some(pool) => Some(Pool::decode(pool.value.as_slice()).unwrap()),
        Some(pool) => {
            let decoded_pool = Message::decode(pool.value.as_ref())?;
            Ok(decoded_pool)
        }
        None => Err(ContractError::PoolNotFound { pool_id }),
    }
}

pub fn may_get_position(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    _env: &Env,
) -> Result<Option<FullPositionBreakdown>, ContractError> {
    let position = POSITION.may_load(storage)?;
    if let Some(position) = position {
        let cl_querier = ConcentratedliquidityQuerier::new(querier);
        let position = cl_querier.position_by_id(position.position_id)?;
        Ok(Some(
            position.position.ok_or(ContractError::PositionNotFound)?,
        ))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{PoolConfig, Position};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Coin, Uint128,
    };

    use super::*;

    #[test]
    fn test_create_position() {
        let mut deps = mock_dependencies();
        let pool_id = 1;
        POOL_CONFIG
            .save(
                deps.as_mut().storage,
                &PoolConfig {
                    pool_id,
                    token0: "token0".to_string(),
                    token1: "token1".to_string(),
                },
            )
            .unwrap();

        let env = mock_env();
        let lower_tick = 100;
        let upper_tick = 200;
        let tokens_provided = vec![Coin::new(100, "token0"), Coin::new(200, "token1")];
        let token_min_amount0 = Uint128::new(1000);
        let token_min_amount1 = Uint128::new(2000);

        let result = create_position(
            deps.as_mut().storage,
            &env,
            lower_tick,
            upper_tick,
            tokens_provided.clone(),
            token_min_amount0,
            token_min_amount1,
        )
        .unwrap();

        assert_eq!(
            result,
            MsgCreatePosition {
                pool_id,
                sender: env.contract.address.into(),
                lower_tick,
                upper_tick,
                tokens_provided: tokens_provided.into_iter().map(|c| c.into()).collect(),
                token_min_amount0: token_min_amount0.to_string(),
                token_min_amount1: token_min_amount1.to_string()
            }
        );
    }

    #[test]
    fn test_withdraw_from_position() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let liquidity_amount = Decimal256::from_ratio(100_u128, 1_u128);

        let position_id = 1;
        POSITION
            .save(deps.as_mut().storage, &Position { position_id })
            .unwrap();

        let result = withdraw_from_position(&mut deps.storage, &env, liquidity_amount).unwrap();

        assert_eq!(
            result,
            MsgWithdrawPosition {
                position_id,
                sender: env.contract.address.into(),
                liquidity_amount: liquidity_amount.atomics().to_string()
            }
        );
    }
}
