use cosmwasm_std::{Coin, CosmosMsg, Decimal256, Env, QuerierWrapper, Storage, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, FullPositionBreakdown, MsgCreatePosition,
    MsgFungifyChargedPositions, MsgWithdrawPosition, PositionByIdRequest,
};

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
    }
    .into();
    Ok(create_position)
}

// TODO verify that liquidity amount should be Decimal256
pub fn withdraw_from_position(
    storage: &mut dyn Storage,
    env: &Env,
    liquidity_amount: Decimal256,
) -> Result<MsgWithdrawPosition, ContractError> {
    let sender = env.contract.address.to_string();
    let position = POSITION.load(storage)?;

    let withdraw_position = MsgWithdrawPosition {
        position_id: position.position_id,
        sender,
        liquidity_amount: liquidity_amount.to_string(),
    };
    Ok(withdraw_position)
}

// merge any newly created user positions with our main position
pub fn merge_positions(
    storage: &mut dyn Storage,
    env: &Env,
    mut position_ids: Vec<u64>,
) -> Result<MsgFungifyChargedPositions, ContractError> {
    let sender = env.contract.address.to_string();
    let position = POSITION.load(storage)?;

    // TODO we could add some extra verifications here checking that the pool positions are the same
    // but we should figure out whether thats something we want, since all positions should be the same according
    // to the logic of the entire cl-vault

    position_ids.push(position.position_id);
    let fungify = MsgFungifyChargedPositions {
        position_ids,
        sender,
    };
    Ok(fungify)
}

pub fn get_position(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
) -> Result<FullPositionBreakdown, ContractError> {
    let position = POSITION.load(storage)?;

    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let position = cl_querier.position_by_id(position.position_id)?;
    position.position.ok_or(ContractError::PositionNotFound)
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
                    base_token: "token0".to_string(),
                    quote_token: "token1".to_string(),
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
                liquidity_amount: liquidity_amount.to_string()
            }
        );
    }

    #[test]
    fn test_merge_positions() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let position_ids = vec![2, 3, 4];

        let position_id = 1;
        POSITION
            .save(deps.as_mut().storage, &Position { position_id })
            .unwrap();

        let mut expected_position_ids = position_ids.clone();
        expected_position_ids.push(position_id);

        let result = merge_positions(&mut deps.storage, &env, position_ids).unwrap();

        assert_eq!(
            result,
            MsgFungifyChargedPositions {
                position_ids: expected_position_ids,
                sender: env.contract.address.into(),
            }
        );
    }
}
