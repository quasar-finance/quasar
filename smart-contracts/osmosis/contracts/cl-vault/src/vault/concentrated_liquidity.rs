use cosmwasm_std::{Coin, Decimal256, DepsMut, Env, QuerierWrapper, Storage, Uint128, Uint256};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, FullPositionBreakdown, MsgCreatePosition, MsgWithdrawPosition,
    Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use prost::Message;

use crate::helpers::generic::{round_up_to_nearest_multiple, sort_tokens};
use crate::{
    state::{POOL_CONFIG, POSITION},
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
        tokens_provided: sorted_tokens
            .into_iter()
            .filter(|c| !c.amount.is_zero())
            .map(|c| c.into())
            .collect(),
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
        Some(pool) => {
            let decoded_pool = Message::decode(pool.value.as_ref())?;
            Ok(decoded_pool)
        }
        None => Err(ContractError::PoolNotFound { pool_id }),
    }
}

pub fn _may_get_position(
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

// see https://uniswap.org/whitepaper-v3.pdf for below formulas (eq 6.29 & 6.30)
pub fn get_liquidity_for_base_token(
    amount: Uint256,
    sqrt_p: Decimal256,
    sqrt_pl: Decimal256,
    sqrt_pu: Decimal256,
) -> Result<Uint256, ContractError> {
    debug_assert!(
        sqrt_p < sqrt_pu,
        "can't compute liquidity if sqrt_p >= sqrt_pu"
    );
    if sqrt_p >= sqrt_pu {
        return Ok(Uint256::MAX);
    }
    let sqrt_p = std::cmp::max(sqrt_p, sqrt_pl);
    let delta_p = sqrt_pu - sqrt_p;
    Ok(amount.checked_mul_floor(sqrt_pu.checked_mul(sqrt_p)?.checked_div(delta_p)?)?)
}

pub fn get_liquidity_for_quote_token(
    amount: Uint256,
    sqrt_p: Decimal256,
    sqrt_pl: Decimal256,
    sqrt_pu: Decimal256,
) -> Result<Uint256, ContractError> {
    debug_assert!(
        sqrt_p > sqrt_pl,
        "can't compute liquidity if sqrt_p <= sqrt_pl"
    );
    if sqrt_p <= sqrt_pl {
        return Ok(Uint256::MAX);
    }
    let sqrt_p = std::cmp::min(sqrt_p, sqrt_pu);
    let delta_p = sqrt_p - sqrt_pl;
    Ok(amount.checked_div_floor(delta_p)?)
}

pub fn get_amount_from_liquidity_for_base_token(
    liq: Uint256,
    sqrt_p: Decimal256,
    sqrt_pl: Decimal256,
    sqrt_pu: Decimal256,
) -> Result<Uint256, ContractError> {
    let sqrt_p = std::cmp::max(sqrt_p, sqrt_pl);
    let delta_p = sqrt_pu.checked_sub(sqrt_p).unwrap_or_default();
    Ok(liq.checked_mul_floor(delta_p.checked_div(sqrt_pu.checked_mul(sqrt_p)?)?)?)
}

pub fn get_amount_from_liquidity_for_quote_token(
    liq: Uint256,
    sqrt_p: Decimal256,
    sqrt_pl: Decimal256,
    sqrt_pu: Decimal256,
) -> Result<Uint256, ContractError> {
    let sqrt_p = std::cmp::min(sqrt_p, sqrt_pu);
    let delta_p = sqrt_p.checked_sub(sqrt_pl).unwrap_or_default();
    Ok(liq.checked_mul_floor(delta_p)?)
}

#[cfg(test)]
mod tests {
    use crate::{
        state::{PoolConfig, Position},
        test_helpers::QuasarQuerier,
    };
    use cosmwasm_std::{
        assert_approx_eq, coin,
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
            FullPositionBreakdown {
                position: Some(OsmoPosition {
                    position_id: 1,
                    address: "bob".to_string(),
                    pool_id: 1,
                    lower_tick: 1,
                    upper_tick: 100,
                    join_time: None,
                    liquidity: "123.214".to_string(),
                }),
                asset0: Some(coin(1000, "uosmo").into()),
                asset1: Some(coin(1000, "uatom").into()),
                claimable_spread_rewards: vec![coin(1000, "uosmo").into()],
                claimable_incentives: vec![coin(123, "uatom").into()],
                forfeited_incentives: vec![],
            },
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
        POSITION
            .save(
                deps.as_mut().storage,
                &Position {
                    position_id,
                    join_time: 0,
                    claim_after: None,
                },
            )
            .unwrap();

        let result = withdraw_from_position(&deps.storage, &env, liquidity_amount).unwrap();

        assert_eq!(
            result,
            MsgWithdrawPosition {
                position_id,
                sender: env.contract.address.into(),
                liquidity_amount: liquidity_amount.atomics().to_string()
            }
        );
    }

    #[test]
    fn test_get_amount_from_liquidity_for_base_token_if_price_above_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(50);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(200);
        let amount =
            get_amount_from_liquidity_for_base_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = Uint256::zero();
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_amount_from_liquidity_for_base_token_if_price_in_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(25);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(50);
        let amount =
            get_amount_from_liquidity_for_base_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = liq;
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_amount_from_liquidity_for_base_token_if_price_below_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(50);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(25);
        let amount =
            get_amount_from_liquidity_for_base_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = liq;
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_amount_from_liquidity_for_quote_token_if_price_below_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(50);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(25);
        let amount =
            get_amount_from_liquidity_for_quote_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = Uint256::zero();
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_amount_from_liquidity_for_quote_token_if_price_in_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(25);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(75);
        let amount =
            get_amount_from_liquidity_for_quote_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = Uint256::from(25u64);
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_amount_from_liquidity_for_quote_token_if_price_above_range() {
        let liq = Uint256::from(50u64);
        let sqrt_pl = Decimal256::percent(50);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(200);
        let amount =
            get_amount_from_liquidity_for_quote_token(liq, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_amount = Uint256::from(25u64);
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_get_liquidity_from_amount_for_base_token_if_price_in_range() {
        let amount = Uint256::from(150u64);
        let sqrt_pl = Decimal256::percent(10);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(25);
        let liq = get_liquidity_for_base_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(49u64);
        assert_eq!(liq, expected_liq);
    }

    #[test]
    fn test_get_liquidity_from_amount_for_base_token_if_price_below_range() {
        let amount = Uint256::from(150u64);
        let sqrt_pl = Decimal256::percent(25);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(10);
        let liq = get_liquidity_for_base_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(49u64);
        assert_eq!(liq, expected_liq);
    }

    #[test]
    fn test_get_liquidity_from_amount_for_quote_token_if_price_in_range() {
        let amount = Uint256::from(150u64);
        let sqrt_pl = Decimal256::percent(10);
        let sqrt_pu = Decimal256::percent(100);
        let sqrt_p = Decimal256::percent(60);
        let liq = get_liquidity_for_quote_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(300u64);
        assert_eq!(liq, expected_liq);
    }

    #[test]
    fn test_get_liquidity_from_amount_for_quote_token_if_price_above_range() {
        let amount = Uint256::from(150u64);
        let sqrt_pl = Decimal256::percent(10);
        let sqrt_pu = Decimal256::percent(60);
        let sqrt_p = Decimal256::percent(100);
        let liq = get_liquidity_for_quote_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(300u64);
        assert_eq!(liq, expected_liq);
    }

    // tests

    #[test]
    fn test_get_liquidity_for_base() {
        let amount = Uint256::from(1_000_000u64);
        let sqrt_pl = Decimal256::percent(65);
        let sqrt_pu = Decimal256::percent(130);
        let sqrt_p = Decimal256::one();
        let liq = get_liquidity_for_base_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(4_333_333u64);
        assert_eq!(liq, expected_liq);

        let final_amount: Uint128 =
            get_amount_from_liquidity_for_base_token(liq, sqrt_p, sqrt_pl, sqrt_pu)
                .unwrap()
                .try_into()
                .unwrap();
        assert_approx_eq!(final_amount, amount.try_into().unwrap(), "0.000001");

        let used_liquidity = Uint256::from(2_857_142u64);
        let residual_liquidity = Uint256::from(4_333_333u64) - used_liquidity;
        let residual_amount: Uint128 =
            get_amount_from_liquidity_for_base_token(residual_liquidity, sqrt_p, sqrt_pl, sqrt_pu)
                .unwrap()
                .try_into()
                .unwrap();
        let expected_residual_amount = Uint128::from(340659u64);
        assert_eq!(residual_amount, expected_residual_amount);

        let used_amount =
            get_amount_from_liquidity_for_base_token(used_liquidity, sqrt_p, sqrt_pl, sqrt_pu)
                .unwrap();
        let residual_amount: Uint128 = (amount - used_amount).try_into().unwrap();
        let expected_residual_amount = Uint128::from(340660u64);
        assert_eq!(residual_amount, expected_residual_amount);
    }
    #[test]
    fn test_get_liquidity_for_quote() {
        let amount = Uint256::from(1_000_000u64);
        let sqrt_pl = Decimal256::percent(65);
        let sqrt_pu = Decimal256::percent(130);
        let sqrt_p = Decimal256::one();
        let liq = get_liquidity_for_quote_token(amount, sqrt_p, sqrt_pl, sqrt_pu).unwrap();
        let expected_liq = Uint256::from(2_857_142u64);
        assert_eq!(liq, expected_liq);

        let final_amount: Uint128 =
            get_amount_from_liquidity_for_quote_token(liq, sqrt_p, sqrt_pl, sqrt_pu)
                .unwrap()
                .try_into()
                .unwrap();
        assert_approx_eq!(final_amount, amount.try_into().unwrap(), "0.000001");
    }
}
