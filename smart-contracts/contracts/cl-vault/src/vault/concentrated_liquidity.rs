use cosmwasm_std::{
    Coin, Decimal256, DepsMut, Env, Order, QuerierWrapper, StdError, Storage, Timestamp, Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, FullPositionBreakdown, MsgCreatePosition, MsgWithdrawPosition,
    Pool, Position,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use prost::Message;

use crate::helpers::generic::{round_up_to_nearest_multiple, sort_tokens};
use crate::{
    helpers::{round_up_to_nearest_multiple, sort_tokens},
    state::POOL_CONFIG,
    ContractError,
};

pub fn create_position(
    deps: DepsMut,
    env: &Env,
    lower_tick: i64,
    upper_tick: i64,
    tokens_provided: Vec<Coin>,
    token_min_amount0: Uint128,
    token_min_amount1: Uint128,
) -> Result<MsgCreatePosition, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let sender = env.contract.address.to_string();

    let sorted_tokens = sort_tokens(tokens_provided);

    let pool_details = get_cl_pool_info(&deps.querier, pool_config.pool_id)?;
    let tick_spacing = pool_details
        .tick_spacing
        .try_into()
        .expect("tick spacing is too big to fit into i64");

    let create_position = MsgCreatePosition {
        pool_id: pool_config.pool_id,
        sender,
        lower_tick: round_up_to_nearest_multiple(lower_tick, tick_spacing),
        upper_tick: round_up_to_nearest_multiple(upper_tick, tick_spacing),
        tokens_provided: sorted_tokens.into_iter().map(|c| c.into()).collect(),
        // An sdk.Int in the Go code
        token_min_amount0: token_min_amount0.to_string(),
        // An sdk.Int in the Go code
        token_min_amount1: token_min_amount1.to_string(),
    };
    Ok(create_position)
}

// TODO verify that liquidity amount should be Decimal256
pub fn withdraw_from_position(
    env: &Env,
    position_id: u64,
    liquidity_amount: Decimal256,
) -> Result<MsgWithdrawPosition, ContractError> {
    let sender = env.contract.address.to_string();

    let withdraw_position = MsgWithdrawPosition {
        position_id,
        sender,
        liquidity_amount: liquidity_amount.atomics().to_string(),
    };
    Ok(withdraw_position)
}

pub fn get_positions(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
) -> Result<Vec<(crate::state::Position, FullPositionParsed)>, ContractError> {
    let position_ids: Result<Vec<(u64, crate::state::Position)>, StdError> = POSITIONS
        .range(storage, None, None, Order::Ascending)
        .collect();

    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let positions: Result<Vec<(crate::state::Position, FullPositionParsed)>, ContractError> =
        position_ids?
            .into_iter()
            .map(|(id, position)| Ok((position, get_parsed_position(querier, id)?)))
            .collect();

    positions
}

pub fn get_position(
    querier: &QuerierWrapper,
    position_id: u64,
) -> Result<FullPositionBreakdown, ContractError> {
    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let position = cl_querier.position_by_id(position_id)?;
    position.position.ok_or(ContractError::PositionNotFound)
}

pub fn get_parsed_position(
    querier: &QuerierWrapper,
    position_id: u64,
) -> Result<FullPositionParsed, ContractError> {
    let cl_querier = ConcentratedliquidityQuerier::new(querier);
    let position = cl_querier.position_by_id(position_id)?;
    position
        .position
        .ok_or(ContractError::PositionNotFound)?
        .try_into()
}

// TODO move these structs to a package and enable direct proto decoding
// into these structs
pub struct FullPositionParsed {
    pub position: PositionParsed,
    pub asset0: Coin,
    pub asset1: Coin,
    pub claimable_spread_rewards: Vec<Coin>,
    pub claimable_incentives: Vec<Coin>,
    pub forfeited_incentives: Vec<Coin>,
}

impl TryFrom<FullPositionBreakdown> for FullPositionParsed {
    type Error = ContractError;

    fn try_from(value: FullPositionBreakdown) -> Result<Self, Self::Error> {
        Ok(Self {
            position: value
                .position
                .ok_or(ContractError::PositionNotFound)?
                .try_into()?,
            asset0: value.asset0.unwrap().try_into()?,
            asset1: value.asset1.unwrap().try_into()?,
            claimable_spread_rewards: osmosis_std::try_proto_to_cosmwasm_coins(
                value.claimable_spread_rewards,
            )?,
            claimable_incentives: osmosis_std::try_proto_to_cosmwasm_coins(
                value.claimable_incentives,
            )?,
            forfeited_incentives: osmosis_std::try_proto_to_cosmwasm_coins(
                value.forfeited_incentives,
            )?,
        })
    }
}

// TODO move these structs to a package and enable direct proto decoding
// into these structs
pub struct PositionParsed {
    pub position_id: u64,
    pub address: ::prost::alloc::string::String,
    pub pool_id: u64,
    pub lower_tick: i64,
    pub upper_tick: i64,
    pub join_time: Timestamp,
    pub liquidity: Decimal256,
}

impl TryFrom<Position> for PositionParsed {
    type Error = ContractError;

    fn try_from(value: Position) -> Result<Self, Self::Error> {
        Ok(Self {
            position_id: value.position_id,
            address: value.address,
            pool_id: value.pool_id,
            lower_tick: value.lower_tick,
            upper_tick: value.upper_tick,
            join_time: value
                .join_time
                // This conversion is sloppy and loses seconds information
                .map(|t| Timestamp::from_seconds(t.seconds.try_into().unwrap()))
                .unwrap_or(Timestamp::default()),
            liquidity: value.liquidity.parse()?,
        })
    }
}

pub fn get_cl_pool_info(querier: &QuerierWrapper, pool_id: u64) -> Result<Pool, ContractError> {
    let pm_querier = PoolmanagerQuerier::new(querier);
    let pool: osmosis_std::types::osmosis::poolmanager::v1beta1::PoolResponse =
        pm_querier.pool(pool_id)?;

    match pool.pool {
        // Some(pool) => Some(Pool::decode(pool.value.as_slice())?),
        Some(pool) => {
            let decoded_pool = Message::decode(pool.value.as_ref())?;
            Ok(decoded_pool)
        }
        None => Err(ContractError::PoolNotFound { pool_id }),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        state::{PoolConfig, Position, MAIN_POSITION, POSITIONS},
        test_helpers::{FullPositionBuilder, QuasarQuerier},
    };
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env},
        Coin, Uint128,
    };

    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Position as OsmoPosition;

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
                    token0: "uosmo".to_string(),
                    token1: "uatom".to_string(),
                },
            )
            .unwrap();
        let qq = QuasarQuerier::new(
            vec![FullPositionBuilder::new(
                1,
                1,
                1,
                100,
                None,
                Decimal256::from_str("123.214").unwrap(),
                coin(1000, "uosmo"),
                coin(1000, "uatom"),
            )
            .with_spread_rewards(vec![coin(123, "uatom")])
            .with_incentives(vec![coin(1000, "uosmo")])
            .build()],
            100,
        );

        let mut deps_mut = deps.as_mut();
        deps_mut.querier = QuerierWrapper::new(&qq);

        let env = mock_env();
        let lower_tick = 100;
        let upper_tick = 200;
        let tokens_provided = vec![Coin::new(100, "uosmo"), Coin::new(200, "uatom")];
        let token_min_amount0 = Uint128::new(1000);
        let token_min_amount1 = Uint128::new(2000);

        let result = create_position(
            deps_mut,
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
                tokens_provided: sort_tokens(tokens_provided)
                    .into_iter()
                    .map(|c| c.into())
                    .collect(),
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
        MAIN_POSITION
            .save(deps.as_mut().storage, &position_id)
            .unwrap();
        POSITIONS
            .save(
                deps.as_mut().storage,
                position_id,
                &Position {
                    position_id,
                    join_time: 0,
                    claim_after: None,
                },
            )
            .unwrap();

        let result = withdraw_from_position(&env, position_id, liquidity_amount).unwrap();

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
