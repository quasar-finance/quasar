use crate::math::tick::tick_to_price;
use crate::state::{PoolConfig, RANGE_ADMIN};
use std::str::FromStr;

use osmosis_std::shim::Timestamp as OsmoTimestamp;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::twap::v1beta1::TwapQuerier;

use crate::vault::concentrated_liquidity::get_position;
use crate::{state::POOL_CONFIG, ContractError};
use cosmwasm_std::{
    Addr, Coin, Decimal, Decimal256, Deps, DepsMut, Env, Fraction, QuerierWrapper, Storage,
    Uint128, Uint256,
};
use osmosis_std::try_proto_to_cosmwasm_coins;

use super::coinlist::CoinList;

pub fn get_range_admin(deps: Deps) -> Result<Addr, ContractError> {
    Ok(RANGE_ADMIN.load(deps.storage)?)
}

/// Calculate the total value of two assets in asset0.
pub fn get_asset0_value(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    token0: Uint128,
    token1: Uint128,
) -> Result<Uint128, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let pm_querier = PoolmanagerQuerier::new(querier);
    let spot_price: Decimal = pm_querier
        .spot_price(pool_config.pool_id, pool_config.token0, pool_config.token1)?
        .spot_price
        .parse()?;

    let total = token0
        .checked_add(token1.multiply_ratio(spot_price.denominator(), spot_price.numerator()))?;

    Ok(total)
}

pub fn get_twap_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    twap_window_seconds: u64,
) -> Result<Decimal, ContractError> {
    let pool_config = POOL_CONFIG.load(storage)?;

    let twap_querier = TwapQuerier::new(querier);
    let start_of_window = env.block.time.minus_seconds(twap_window_seconds);
    let twap_price = twap_querier.arithmetic_twap_to_now(
        pool_config.pool_id,
        pool_config.token0,
        pool_config.token1,
        Some(OsmoTimestamp {
            seconds: start_of_window.seconds().try_into()?,
            nanos: 0,
        }),
    )?;

    Ok(Decimal::from_str(&twap_price.arithmetic_twap)?)
}

/// Calculate the amount of tokens that can be deposited while maintaining the current position ratio in the vault.
#[allow(clippy::type_complexity)]
pub fn get_depositable_tokens(
    deps: DepsMut,
    token0: Coin,
    token1: Coin,
) -> Result<((Uint128, Uint128), (Uint128, Uint128)), ContractError> {
    let position = get_position(deps.storage, &deps.querier)?;
    let asset0_amount = Uint128::from_str(&position.clone().asset0.unwrap_or_default().amount)?;
    let asset1_amount = Uint128::from_str(&position.clone().asset1.unwrap_or_default().amount)?;

    match (asset0_amount.is_zero(), asset1_amount.is_zero()) {
        (true, false) => Ok((
            (Uint128::zero(), token1.amount),
            (token0.amount, Uint128::zero()),
        )),
        (false, true) => Ok((
            (token0.amount, Uint128::zero()),
            (Uint128::zero(), token1.amount),
        )),
        /*
           Figure out how many of the tokens we can use for a double sided position.
           First we find whether token0 or token0 is the limiting factor for the token by
           dividing token0 by the current amount of token0 in the position and do the same for token1
           for calculating further amounts we start from:
           token0 / token1 = ratio0 / ratio1, where ratio0 / ratio1 are the ratios from the position

           if token0 is limiting, we calculate the token1 amount by
           token1 = token0*ratio1/ratio0

           if token1 is limiting, we calculate the token0 amount by
           token0 = token1 * ratio0 / ratio1
        */
        (false, false) => {
            let token0 = token0.amount;
            let token1 = token1.amount;
            let assets = try_proto_to_cosmwasm_coins(vec![
                position.asset0.unwrap(),
                position.asset1.unwrap(),
            ])?;
            let ratio = Decimal::from_ratio(assets[0].amount, assets[1].amount);

            // Refund token0 if ratio.numerator is zero
            if ratio.numerator().is_zero() {
                return Ok(((Uint128::zero(), token1), (token0, Uint128::zero())));
            }

            let zero_usage: Uint128 = ((Uint256::from(token0)
                * Uint256::from_u128(1_000_000_000_000_000_000u128))
                / Uint256::from(ratio.numerator()))
            .try_into()?;
            let one_usage: Uint128 = ((Uint256::from(token1)
                * Uint256::from_u128(1_000_000_000_000_000_000u128))
                / Uint256::from(ratio.denominator()))
            .try_into()?;

            if zero_usage < one_usage {
                let t1: Uint128 = (Uint256::from(token0) * (Uint256::from(ratio.denominator()))
                    / Uint256::from(ratio.numerator()))
                .try_into()?;
                Ok(((token0, t1), (Uint128::zero(), token1.checked_sub(t1)?)))
            } else {
                let t0: Uint128 = ((Uint256::from(token1) * Uint256::from(ratio.numerator()))
                    / Uint256::from(ratio.denominator()))
                .try_into()?;
                Ok(((t0, token1), (token0.checked_sub(t0)?, Uint128::zero())))
            }
        }
        // (true, true) => {
        _ => Err(ContractError::InvalidRatioOfSwappableFundsToUse {}),
    }
}

// this math is straight from the readme
pub fn get_single_sided_deposit_0_to_1_swap_amount(
    token0_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    // TODO error here if this condition holds
    // if current_tick < lower_tick {
    //     return ;
    // }

    let lower_price = tick_to_price(lower_tick)?;
    let current_price = tick_to_price(current_tick)?;
    let upper_price = tick_to_price(upper_tick)?;

    let cur_price_sqrt = current_price.sqrt();
    let lower_price_sqrt = lower_price.sqrt();
    let upper_price_sqrt = upper_price.sqrt();

    // let pool_metadata_constant: Decimal256 = cur_price_sqrt
    //     .checked_mul(lower_price_sqrt)?
    //     .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?)?
    //     .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let pool_metadata_constant: Decimal256 = (upper_price_sqrt
        .checked_mul(cur_price_sqrt)?
        .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?))?
    .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let spot_price_over_pool_metadata_constant =
        current_price.checked_div(pool_metadata_constant)?;

    let denominator = Decimal256::one().checked_add(spot_price_over_pool_metadata_constant)?;

    let swap_amount: Uint128 = Uint256::from(token0_balance)
        .multiply_ratio(denominator.denominator(), denominator.numerator())
        .try_into()?;

    Ok(swap_amount)
}

pub fn get_single_sided_deposit_1_to_0_swap_amount(
    token1_balance: Uint128,
    lower_tick: i64,
    current_tick: i64,
    upper_tick: i64,
) -> Result<Uint128, ContractError> {
    let lower_price = tick_to_price(lower_tick)?;
    let current_price = tick_to_price(current_tick)?;
    let upper_price = tick_to_price(upper_tick)?;

    let cur_price_sqrt = current_price.sqrt();
    let lower_price_sqrt = lower_price.sqrt();
    let upper_price_sqrt = upper_price.sqrt();

    let pool_metadata_constant: Decimal256 = (upper_price_sqrt
        .checked_mul(cur_price_sqrt)?
        .checked_mul(cur_price_sqrt.checked_sub(lower_price_sqrt)?))?
    .checked_div(upper_price_sqrt.checked_sub(cur_price_sqrt)?)?;

    let pool_metadata_constant_over_spot_price: Decimal256 =
        pool_metadata_constant.checked_div(current_price)?;

    let denominator = Decimal256::one().checked_add(pool_metadata_constant_over_spot_price)?;

    let swap_amount: Uint128 = Uint256::from(token1_balance)
        .multiply_ratio(denominator.denominator(), denominator.numerator())
        .try_into()?;

    Ok(swap_amount)
}

/// this function subtracts out anything from the raw contract balance that isn't dedicated towards user or strategist rewards.
pub fn get_unused_balances(querier: &QuerierWrapper, env: &Env) -> Result<CoinList, ContractError> {
    Ok(CoinList::from_coins(
        querier.query_all_balances(env.contract.address.to_string())?,
    ))
}

pub fn get_unused_pair_balances(
    deps: &DepsMut,
    env: &Env,
    pool_config: &PoolConfig,
) -> Result<(Uint128, Uint128), ContractError> {
    // Get unused balances from the contract. This is the amount of tokens that are not currently in a position.
    // This amount already includes the withdrawn amounts from previous steps as in this reply those funds already compose the contract balance.
    let unused_balances = get_unused_balances(&deps.querier, env)?;

    // Use the unused balances to get the token0 and token1 amounts that we can use to create a new position
    let amount0 = unused_balances.find_coin(pool_config.token0.clone()).amount;
    let amount1 = unused_balances.find_coin(pool_config.token1.clone()).amount;

    Ok((amount0, amount1))
}

pub fn get_tokens_provided(
    amount0: Uint128,
    amount1: Uint128,
    pool_config: &PoolConfig,
) -> Result<Vec<Coin>, ContractError> {
    let mut tokens_provided = vec![];
    if !amount0.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token0.clone(),
            amount: amount0,
        })
    }
    if !amount1.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token1.clone(),
            amount: amount1,
        })
    }

    Ok(tokens_provided)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cosmwasm_std::testing::mock_dependencies;

    use crate::math::tick::{build_tick_exp_cache, price_to_tick};

    use super::*;

    #[test]
    fn test_0_to_1_swap() {
        let mut deps = mock_dependencies();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

        let lower_price = "4500";
        let upper_price = "5500";
        let token0amt = 200000u128;

        // prices and expected amounts taken from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
        let mut prices = HashMap::new();
        prices.insert("4501", Uint128::new(232));
        prices.insert("4600", Uint128::new(22674));
        prices.insert("4700", Uint128::new(44304));
        prices.insert("4800", Uint128::new(65099));
        prices.insert("4900", Uint128::new(85241));
        prices.insert("5000", Uint128::new(104884));
        prices.insert("5100", Uint128::new(124166));
        prices.insert("5200", Uint128::new(143210));
        prices.insert("5300", Uint128::new(162128));
        prices.insert("5400", Uint128::new(181025));
        prices.insert("5499", Uint128::new(199809));

        let lower_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(lower_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        let upper_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(upper_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        for (price, result) in prices.into_iter() {
            let curr_tick =
                price_to_tick(deps.as_mut().storage, Decimal256::from_str(price).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();

            let swap_amount = get_single_sided_deposit_0_to_1_swap_amount(
                token0amt.into(),
                lower_tick,
                curr_tick,
                upper_tick,
            )
            .unwrap();

            assert_eq!(swap_amount, result);
        }
    }

    #[test]
    fn test_1_to_0_swap() {
        let mut deps = mock_dependencies();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

        let lower_price = "4500";
        let upper_price = "5500";

        let token1amt = 200000u128;

        // prices and expected amounts taken from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
        let mut prices = HashMap::new();
        prices.insert("4501", Uint128::new(199767));
        prices.insert("4600", Uint128::new(177325));
        prices.insert("4700", Uint128::new(155695));
        prices.insert("4800", Uint128::new(134900));
        prices.insert("4900", Uint128::new(114758));
        prices.insert("5000", Uint128::new(95115));
        prices.insert("5100", Uint128::new(75833));
        prices.insert("5200", Uint128::new(56789));
        prices.insert("5300", Uint128::new(37871));
        prices.insert("5400", Uint128::new(18974));
        prices.insert("5499", Uint128::new(190));

        let lower_tick = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(lower_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        let upper_tick: i64 = price_to_tick(
            deps.as_mut().storage,
            Decimal256::from_str(upper_price).unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();

        for (price, result) in prices.into_iter() {
            let curr_tick =
                price_to_tick(deps.as_mut().storage, Decimal256::from_str(price).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();

            let swap_amount = get_single_sided_deposit_1_to_0_swap_amount(
                token1amt.into(),
                lower_tick,
                curr_tick,
                upper_tick,
            )
            .unwrap();

            assert_eq!(swap_amount, result);
        }
    }
}
