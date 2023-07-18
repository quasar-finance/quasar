use std::str::FromStr;

use crate::{
    state::{TickExpIndexData, TICK_EXP_CACHE},
    ContractError,
};
use cosmwasm_std::{Decimal, Decimal256, DepsMut, Uint128, Uint256};

// due to pow restrictions we need to use unsigned integers; i.e. 10.pow(-exp: u32)
// so if the resulting power is positive, we take 10**exp;
// and if it is negative, we take 1/10**exp.
fn pow_ten_internal(exponent: i128) -> Result<u128, ContractError> {
    if exponent >= 0 {
        return 10u128
            .checked_pow(exponent.abs() as u32)
            .ok_or(ContractError::Overflow {});
    } else {
        // TODO: write tests for negative exponents as it looks like this will always be 0
        Ok(1u128
            / 10u128
                .checked_pow(exponent as u32)
                .ok_or(ContractError::Overflow {})?)
    }
}

// same as pow_ten_internal but returns a Decimal to work with negative exponents
fn pow_ten_internal_dec(exponent: i128) -> Result<Decimal, ContractError> {
    let p = 10u128
        .checked_pow(exponent.abs() as u32)
        .ok_or(ContractError::Overflow {})?;
    if exponent >= 0 {
        return Ok(Decimal::from_ratio(p, 1u128));
    } else {
        Ok(Decimal::from_ratio(1u128, p))
    }
}

// same as pow_ten_internal but returns a Decimal to work with negative exponents
fn pow_ten_internal_dec_256(exponent: i128) -> Result<Decimal256, ContractError> {
    let p = 10u128
        .checked_pow(exponent.abs() as u32)
        .ok_or(ContractError::Overflow {})?;
    if exponent >= 0 {
        return Ok(Decimal256::from_ratio(p, 1u128));
    } else {
        Ok(Decimal256::from_ratio(1u128, p))
    }
}

// TODO: exponent_at_current_price_one is fixed at -6? We assume exp is always neg?
pub fn tick_to_price(
    tick_index: Uint128,
    exponent_at_price_one: i128,
) -> Result<Decimal, ContractError> {
    if tick_index == Uint128::zero() {
        return Ok(Decimal::one());
    }

    let geometric_exponent_increment_distance_in_ticks = 9u128
        .checked_mul(pow_ten_internal(-exponent_at_price_one)?)
        .ok_or(ContractError::Overflow {})?;

    // TODO: if exponent_at_price_one is not negative, we'll hit division by zero error with Osmosis current logic
    let geometric_exponential_delta: u128 = tick_index
        .checked_div(geometric_exponent_increment_distance_in_ticks.into())?
        .u128();

    let exponent_at_current_tick: i128 =
        exponent_at_price_one + geometric_exponential_delta as i128;

    // TODO: tick_index should always be positive, right? Osmosis go code has a check for it being neg. We use Uint128 so it can't be neg

    let current_additive_increment_in_ticks = pow_ten_internal_dec(exponent_at_current_tick)?;

    let num_additive_ticks: u128 = tick_index.u128()
        - geometric_exponential_delta
            .checked_mul(geometric_exponent_increment_distance_in_ticks)
            .ok_or(ContractError::Overflow {})?;

    let price = pow_ten_internal_dec(geometric_exponential_delta as i128)?.checked_add(
        Decimal::from_ratio(num_additive_ticks, 1u128)
            .checked_mul(current_additive_increment_in_ticks)?,
    )?;

    Ok(price)
}

pub fn price_to_tick(
    price: Decimal,
    mut exponent_at_price_one: i128,
) -> Result<Uint128, ContractError> {
    if price == Decimal::one() {
        return Ok(Uint128::zero());
    }

    let mut current_price = Decimal::one();
    let mut ticks_passed: Uint128 = Uint128::zero();

    let geometric_exponent_increment_distance_in_ticks = 9u128
        .checked_mul(pow_ten_internal(-exponent_at_price_one)?)
        .ok_or(ContractError::Overflow {})?;

    // TODO: need this to live after the loop, is there a better way to do it?
    let mut current_additive_increment_in_ticks = Decimal::zero();

    // TODO: what about when price is less or equal to one?
    while current_price < price {
        current_additive_increment_in_ticks = Decimal::from_ratio(
            Uint128::one(),
            Uint128::from(10u128.pow(exponent_at_price_one.abs() as u32) as u128),
        );

        exponent_at_price_one += 1;

        let max_price_for_current_increment_in_ticks = current_additive_increment_in_ticks
            .checked_mul(Decimal::from_ratio(
                geometric_exponent_increment_distance_in_ticks,
                1u128,
            ))?;

        ticks_passed += Uint128::new(geometric_exponent_increment_distance_in_ticks.into());

        current_price = current_price.checked_add(max_price_for_current_increment_in_ticks)?;
    }

    // this was a negative number is Osmosis, did inverse logic to get it to work
    let ticks_to_be_fullfilled_by_exponent_at_current_tick = current_price
        .checked_sub(price)?
        .checked_div(current_additive_increment_in_ticks)?;

    // decide whether to use floor vs ceil
    let tick_index = ticks_passed
        .checked_sub(ticks_to_be_fullfilled_by_exponent_at_current_tick.to_uint_floor())?;

    Ok(tick_index)
}

pub fn price_to_tick_2(
    price: Decimal,
    mut exponent_at_price_one: i128,
) -> Result<Uint128, ContractError> {
    if price == Decimal::one() {
        return Ok(Uint128::zero());
    }

    let mut current_price = Decimal::one();
    let mut ticks_passed: Uint128 = Uint128::zero();

    let geometric_exponent_increment_distance_in_ticks = 9u128
        .checked_mul(pow_ten_internal(-exponent_at_price_one)?)
        .ok_or(ContractError::Overflow {})?;

    // TODO: need this to live after the loop, is there a better way to do it?
    let mut current_additive_increment_in_ticks = Decimal::zero();

    // TODO: what about when price is less or equal to one?
    while current_price < price {
        current_additive_increment_in_ticks = Decimal::from_ratio(
            Uint128::one(),
            Uint128::from(10u128.pow(exponent_at_price_one.abs() as u32) as u128),
        );

        exponent_at_price_one += 1;

        let max_price_for_current_increment_in_ticks = current_additive_increment_in_ticks
            .checked_mul(Decimal::from_ratio(
                geometric_exponent_increment_distance_in_ticks,
                1u128,
            ))?;

        ticks_passed += Uint128::new(geometric_exponent_increment_distance_in_ticks.into());

        current_price = current_price.checked_add(max_price_for_current_increment_in_ticks)?;
    }

    // this was a negative number in Osmosis, did inverse logic to get it to work
    let ticks_to_be_fullfilled_by_exponent_at_current_tick = current_price
        .checked_sub(price)?
        .checked_div(current_additive_increment_in_ticks)?;

    // decide whether to use floor vs ceil
    let tick_index = ticks_passed
        .checked_sub(ticks_to_be_fullfilled_by_exponent_at_current_tick.to_uint_floor())?;

    Ok(tick_index)
}

// ###########################
// testing: this is super ugly
// ###########################

// TODO: had to use a Decimal256 to support 10^35 (Osmosis max spot price)
const MAX_SPOT_PRICE: &str = "100000000000000000000000000000000000000";
const MIN_SPOT_PRICE: &str = "0.000000000001"; // 10^-12
const EXPONENT_AT_PRICE_ONE: i128 = -6;

fn build_tick_exp_cache(deps: DepsMut) -> Result<DepsMut, ContractError> {
    // Build positive indices
    let mut max_price = Decimal256::one();
    let mut cur_exp_index = 0i64;

    while max_price < Decimal256::from_str(MAX_SPOT_PRICE)? {
        let tick_exp_index_data = TickExpIndexData {
            initial_price: pow_ten_internal_dec_256(cur_exp_index.into())?,
            max_price: pow_ten_internal_dec_256((cur_exp_index + 1).into())?,
            additive_increment_per_tick: pow_ten_internal_dec_256(
                EXPONENT_AT_PRICE_ONE + cur_exp_index as i128,
            )?,
            initial_tick: (9u128
                .checked_mul(pow_ten_internal(-EXPONENT_AT_PRICE_ONE)?)
                .ok_or(ContractError::Overflow {})? as i64)
                .checked_mul(cur_exp_index)
                .ok_or(ContractError::Overflow {})?,
        };
        TICK_EXP_CACHE.save(deps.storage, cur_exp_index, &tick_exp_index_data)?;

        max_price = TICK_EXP_CACHE.load(deps.storage, cur_exp_index)?.max_price;
        cur_exp_index += 1;
    }

    // Build negative indices
    let mut min_price = Decimal256::one();
    cur_exp_index = -1;
    while min_price > Decimal256::from_str(MIN_SPOT_PRICE)? {
        let initial_price = pow_ten_internal_dec_256(cur_exp_index.into())?;
        let max_price = pow_ten_internal_dec_256((cur_exp_index + 1).into())?;
        let additive_increment_per_tick =
            pow_ten_internal_dec_256(EXPONENT_AT_PRICE_ONE + cur_exp_index as i128)?;
        let initial_tick = (9u128
            .checked_mul(pow_ten_internal(-EXPONENT_AT_PRICE_ONE)?)
            .ok_or(ContractError::Overflow {})? as i64)
            .checked_mul(cur_exp_index)
            .ok_or(ContractError::Overflow {})?;

        let tick_exp_index_data = TickExpIndexData {
            initial_price,
            max_price,
            additive_increment_per_tick,
            initial_tick,
        };
        TICK_EXP_CACHE.save(deps.storage, cur_exp_index, &tick_exp_index_data)?;

        min_price = TICK_EXP_CACHE
            .load(deps.storage, cur_exp_index)?
            .initial_price;
        cur_exp_index -= 1;
    }
    Ok(deps)
}

pub fn price_to_tick_3(mut deps: DepsMut, price: Decimal256) -> Result<Uint256, ContractError> {
    if price > Decimal256::from_str(MAX_SPOT_PRICE)?
        || price < Decimal256::from_str(MIN_SPOT_PRICE)?
    {
        return Err(ContractError::PriceBoundError { price });
    }
    if price == Decimal256::one() {
        return Ok(Uint256::zero());
    }

    deps = build_tick_exp_cache(deps)?;

    let mut geo_spacing;
    if price > Decimal256::one() {
        let mut index = 0i64;
        geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        while geo_spacing.max_price < price {
            index += 1;
            geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        }
    } else {
        let mut index = -1;
        geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        while geo_spacing.initial_price > price {
            index -= 1;
            geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        }
    }

    let price_in_this_exponent = price - geo_spacing.initial_price;
    let ticks_filled_by_current_spacing =
        price_in_this_exponent / geo_spacing.additive_increment_per_tick;
    let tick_index =
        ticks_filled_by_current_spacing + Decimal256::raw(geo_spacing.initial_tick as u128);
    Ok(tick_index.to_uint_floor())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;

    use super::*;

    #[test]
    fn test_tick_to_price() {
        let tick_index = Uint128::new(36650010u128);
        let exponen_at_current_price_one = -6;
        let expected_price = Decimal::from_atomics(165001u128, 1);
        let price = tick_to_price(tick_index, exponen_at_current_price_one).unwrap();
        assert_eq!(price, expected_price.unwrap());
    }

    #[test]
    fn test_price_to_tick() {
        let price = Decimal::from_atomics(165001u128, 1).unwrap();
        let exponen_at_current_price_one = -6;
        let expected_tick_index = Uint128::new(36650010u128);
        let tick_index = price_to_tick(price, exponen_at_current_price_one).unwrap();
        assert_eq!(tick_index, expected_tick_index);
    }

    #[test]
    fn test_price_to_tick_3() {
        let mut deps = mock_dependencies();
        let price = Decimal256::from_atomics(165001u128, 1).unwrap();
        let expected_tick_index = Uint256::from_u128(36650010u128);
        let tick_index = price_to_tick_3(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);
    }
}
