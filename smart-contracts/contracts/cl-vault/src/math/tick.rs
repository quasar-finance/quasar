use std::str::FromStr;

use cosmwasm_std::{Decimal, Decimal256, DepsMut, Uint128};

use crate::{
    state::{TickExpIndexData, TICK_EXP_CACHE},
    ContractError,
};

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

// TODO: exponent_at_current_price_one is fixed at -6? We assume exp is always neg?
pub fn tick_to_price_fix(tick_index: i64) -> Result<Decimal, ContractError> {
    if tick_index == 0 {
        return Ok(Decimal::one());
    }

    let geometric_exponent_increment_distance_in_ticks = Decimal::from_str("9")?
        .checked_mul(pow_ten_internal_dec(-EXPONENT_AT_PRICE_ONE)?)?
        .to_string()
        .parse::<i64>()?;

    // Check that the tick index is between min and max value
    if tick_index < MIN_INITIALIZED_TICK as i64 - 1 {
        return Err(ContractError::TickIndexMinError {});
    }

    if tick_index > MAX_TICK as i64 + 1 {
        return Err(ContractError::TickIndexMaxError {});
    }

    // Use floor division to determine what the geometricExponent is now (the delta)
    let geometric_exponent_delta = tick_index / geometric_exponent_increment_distance_in_ticks;

    // Calculate the exponentAtCurrentTick from the starting exponentAtPriceOne and the geometricExponentDelta
    let mut exponent_at_current_tick = EXPONENT_AT_PRICE_ONE as i64 + geometric_exponent_delta;
    if tick_index < 0 {
        // We must decrement the exponentAtCurrentTick when entering the negative tick range in order to constantly step up in precision when going further down in ticks
        // Otherwise, from tick 0 to tick -(geometricExponentIncrementDistanceInTicks), we would use the same exponent as the exponentAtPriceOne
        exponent_at_current_tick -= 1
    }

    // Knowing what our exponentAtCurrentTick is, we can then figure out what power of 10 this exponent corresponds to
    // We need to utilize bigDec here since increments can go beyond the 10^-18 limits set by the sdk
    let current_additive_increment_in_ticks =
        pow_ten_internal_dec_256(exponent_at_current_tick.into())?;

    // Now, starting at the minimum tick of the current increment, we calculate how many ticks in the current geometricExponent we have passed
    let num_additive_ticks =
        tick_index - (geometric_exponent_delta * geometric_exponent_increment_distance_in_ticks);

    // Finally, we can calculate the price
    let price = pow_ten_internal_dec_256(geometric_exponent_delta.into())?
        .checked_add(
            Decimal256::from_str(&num_additive_ticks.to_string())?
                .checked_mul(current_additive_increment_in_ticks)?
                .into(),
        )?
        .to_string()
        .parse::<Decimal>()?;

    // defense in depth, this logic would not be reached due to use having checked if given tick is in between
    // min tick and max tick.
    if price > Decimal::from_str(MAX_SPOT_PRICE)? || price < Decimal::from_str(MIN_SPOT_PRICE)? {
        return Err(ContractError::PriceBoundError {
            price: price.to_string().parse::<Decimal256>()?,
        });
    }
    Ok(price)
}

// THIS IS TRYING TO REPLICATE OSMOSIS GO LOGIC BUT MATH IS A BIT OFF
// TODO: had to use a Decimal256 to support 10^35 (Osmosis max spot price)
const MAX_SPOT_PRICE: &str = "100000000000000000000000000000000000000";
const MIN_SPOT_PRICE: &str = "0.000000000001"; // 10^-12
const EXPONENT_AT_PRICE_ONE: i128 = -6;
const MIN_INITIALIZED_TICK: i128 = -108000000;
const MAX_TICK: i128 = 342000000;

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

        max_price = tick_exp_index_data.max_price;
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

        min_price = tick_exp_index_data.initial_price;
        cur_exp_index -= 1;
    }
    Ok(deps)
}

// TODO: hashmaps vs CW maps?
pub fn price_to_tick(mut deps: DepsMut, price: Decimal256) -> Result<i128, ContractError> {
    if price > Decimal256::from_str(MAX_SPOT_PRICE)?
        || price < Decimal256::from_str(MIN_SPOT_PRICE)?
    {
        return Err(ContractError::PriceBoundError { price });
    }
    if price == Decimal256::one() {
        // return Ok(0i128);
        return Ok(0i128);
    }

    // TODO: move this to instantiate?
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

    let tick_index = geo_spacing.initial_tick as i128
        + ticks_filled_by_current_spacing
            .to_string()
            .parse::<i128>()?;

    Ok(tick_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::mock_dependencies, Uint128};

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
        let mut deps = mock_dependencies();
        // example1
        let mut price = Decimal256::from_str("30352").unwrap();
        let mut expected_tick_index = 38035200;
        let mut tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example2
        price = Decimal256::from_str("30353").unwrap();
        expected_tick_index = 38035300;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(expected_tick_index, tick_index);

        // example3
        price = Decimal256::from_str("0.000011790").unwrap();
        expected_tick_index = -44821000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(expected_tick_index, tick_index);

        // example4
        price = Decimal256::from_str("0.000011791").unwrap();
        expected_tick_index = -44820900;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example5
        price = Decimal256::from_str("0.068960").unwrap();
        expected_tick_index = -12104000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example6
        price = Decimal256::from_str("0.068961").unwrap();
        expected_tick_index = -12103900;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example7
        price = Decimal256::from_str("99999000000000000000000000000000000000").unwrap();
        expected_tick_index = MAX_TICK - 100;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example8
        price = Decimal256::from_str(MAX_SPOT_PRICE).unwrap();
        expected_tick_index = MAX_TICK;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example9
        price = Decimal256::from_str("0.007406").unwrap();
        expected_tick_index = -20594000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example10
        price = Decimal256::from_str("0.0074061").unwrap();
        expected_tick_index = -20593900;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example11
        price = Decimal256::from_str("0.00077960").unwrap();
        expected_tick_index = -29204000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example12
        price = Decimal256::from_str("0.00077961").unwrap();
        expected_tick_index = -29203900;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example13
        price = Decimal256::from_str("0.068500").unwrap();
        expected_tick_index = -12150000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example14
        price = Decimal256::from_str("0.068501").unwrap();
        expected_tick_index = -12149900;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example15
        price = Decimal256::from_str("25760000").unwrap();
        expected_tick_index = 64576000;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example16
        price = Decimal256::from_str("25761000").unwrap();
        expected_tick_index = 64576100;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example17
        price = Decimal256::from_str("1").unwrap();
        expected_tick_index = 0;
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example18: (won't work)... Decimal256 cannot be negative
        assert!(Decimal256::from_str("-1").is_err());

        // example19
        price = Decimal256::from_str(MAX_SPOT_PRICE).unwrap() + Decimal256::one();
        assert!(price_to_tick(deps.as_mut(), price).is_err());

        // example20
        price = Decimal256::from_str(MIN_SPOT_PRICE).unwrap() / Decimal256::from_str("10").unwrap();
        assert!(price_to_tick(deps.as_mut(), price).is_err());
    }
}
