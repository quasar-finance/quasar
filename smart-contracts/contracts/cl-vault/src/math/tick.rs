use std::str::FromStr;

use cosmwasm_std::{Decimal, Decimal256, Response, Storage, Uint128};

use crate::{
    state::{TickExpIndexData, TICK_EXP_CACHE},
    ContractError,
};

const MAX_SPOT_PRICE: &str = "100000000000000000000000000000000000000"; // 10^35
const MIN_SPOT_PRICE: &str = "0.000000000001"; // 10^-12
const EXPONENT_AT_PRICE_ONE: i64 = -6;
const MIN_INITIALIZED_TICK: i64 = -108000000;
const MAX_TICK: i128 = 342000000;

// TODO: exponent_at_current_price_one is fixed at -6? We assume exp is always neg?
pub fn tick_to_price(tick_index: i64) -> Result<Decimal256, ContractError> {
    if tick_index == 0 {
        return Ok(Decimal256::one());
    }

    let geometric_exponent_increment_distance_in_ticks = Decimal::from_str("9")?
        .checked_mul(_pow_ten_internal_dec(-EXPONENT_AT_PRICE_ONE)?)?
        .to_string()
        .parse::<i64>()?;

    // Check that the tick index is between min and max value
    if tick_index < MIN_INITIALIZED_TICK {
        return Err(ContractError::TickIndexMinError {});
    }

    if tick_index > MAX_TICK as i64 {
        return Err(ContractError::TickIndexMaxError {});
    }

    // Use floor division to determine what the geometricExponent is now (the delta)
    let geometric_exponent_delta = tick_index / geometric_exponent_increment_distance_in_ticks;

    // Calculate the exponentAtCurrentTick from the starting exponentAtPriceOne and the geometricExponentDelta
    let mut exponent_at_current_tick = EXPONENT_AT_PRICE_ONE + geometric_exponent_delta;

    if tick_index < 0 {
        // We must decrement the exponentAtCurrentTick when entering the negative tick range in order to constantly step up in precision when going further down in ticks
        // Otherwise, from tick 0 to tick -(geometricExponentIncrementDistanceInTicks), we would use the same exponent as the exponentAtPriceOne
        exponent_at_current_tick -= 1
    }

    // Knowing what our exponentAtCurrentTick is, we can then figure out what power of 10 this exponent corresponds to
    // We need to utilize bigDec here since increments can go beyond the 10^-18 limits set by the sdk
    let current_additive_increment_in_ticks = pow_ten_internal_dec_256(exponent_at_current_tick)?;

    // Now, starting at the minimum tick of the current increment, we calculate how many ticks in the current geometricExponent we have passed
    let num_additive_ticks =
        tick_index - (geometric_exponent_delta * geometric_exponent_increment_distance_in_ticks);

    // Finally, we can calculate the price

    let price: Decimal256 = if num_additive_ticks < 0 {
        _pow_ten_internal_dec(geometric_exponent_delta)?
            .checked_sub(
                Decimal::from_str(&num_additive_ticks.abs().to_string())?.checked_mul(
                    Decimal::from_str(&current_additive_increment_in_ticks.to_string())?,
                )?,
            )?
            .into()
    } else {
        pow_ten_internal_dec_256(geometric_exponent_delta)?.checked_add(
            Decimal256::from_str(&num_additive_ticks.to_string())?
                .checked_mul(current_additive_increment_in_ticks)?,
        )?
    };

    // defense in depth, this logic would not be reached due to use having checked if given tick is in between
    // min tick and max tick.
    if price > Decimal256::from_str(MAX_SPOT_PRICE)?
        || price < Decimal256::from_str(MIN_SPOT_PRICE)?
    {
        return Err(ContractError::PriceBoundError { price });
    }
    Ok(price)
}

pub fn price_to_tick(storage: &mut dyn Storage, price: Decimal256) -> Result<i128, ContractError> {
    if price > Decimal256::from_str(MAX_SPOT_PRICE)?
        || price < Decimal256::from_str(MIN_SPOT_PRICE)?
    {
        return Err(ContractError::PriceBoundError { price });
    }
    if price == Decimal256::one() {
        return Ok(0i128);
    }

    let mut geo_spacing;
    if price > Decimal256::one() {
        let mut index = 0i64;
        loop {
            match TICK_EXP_CACHE.may_load(storage, index)? {
                Some(data) => {
                    geo_spacing = data;
                    if geo_spacing.max_price >= price {
                        break;
                    }
                    index += 1;
                }
                None => {
                    // Rebuild the cache if a tick is not found
                    build_tick_exp_cache(storage)?;
                }
            }
        }
    } else {
        let mut index = -1;
        loop {
            match TICK_EXP_CACHE.may_load(storage, index)? {
                Some(data) => {
                    geo_spacing = data;
                    if geo_spacing.initial_price <= price {
                        break;
                    }
                    index -= 1;
                }
                None => {
                    // Rebuild the cache if a tick is not found
                    build_tick_exp_cache(storage)?;
                }
            }
        }
    }
    let price_in_this_exponent = price - geo_spacing.initial_price;

    let ticks_filled_by_current_spacing =
        price_in_this_exponent / geo_spacing.additive_increment_per_tick;

    // TODO: Optimize this type conversion
    let ticks_filled_uint_floor = ticks_filled_by_current_spacing.to_uint_floor();
    let ticks_filled_int: i128 = Uint128::try_from(ticks_filled_uint_floor)?
        .u128()
        .try_into()
        .unwrap();

    let tick_index = geo_spacing.initial_tick as i128 + ticks_filled_int;

    Ok(tick_index)
}

// due to pow restrictions we need to use unsigned integers; i.e. 10.pow(-exp: u32)
// so if the resulting power is positive, we take 10**exp;
// and if it is negative, we take 1/10**exp.
fn pow_ten_internal_u128(exponent: i64) -> Result<u128, ContractError> {
    if exponent >= 0 {
        10u128
            .checked_pow(exponent.unsigned_abs() as u32)
            .ok_or(ContractError::Overflow {})
    } else {
        // TODO: write tests for negative exponents as it looks like this will always be 0
        Err(ContractError::CannotHandleNegativePowersInUint {})
    }
}

// same as pow_ten_internal but returns a Decimal to work with negative exponents
fn _pow_ten_internal_dec(exponent: i64) -> Result<Decimal, ContractError> {
    let p = 10u128
        .checked_pow(exponent.unsigned_abs() as u32)
        .ok_or(ContractError::Overflow {})?;
    if exponent >= 0 {
        Ok(Decimal::from_ratio(p, 1u128))
    } else {
        Ok(Decimal::from_ratio(1u128, p))
    }
}

// same as pow_ten_internal but returns a Decimal to work with negative exponents
fn pow_ten_internal_dec_256(exponent: i64) -> Result<Decimal256, ContractError> {
    let p = Decimal256::from_str("10")?.checked_pow(exponent.unsigned_abs() as u32)?;
    // let p = 10_u128.pow(exponent as u32);
    if exponent >= 0 {
        Ok(p)
    } else {
        Ok(Decimal256::one() / p)
    }
}

// Iterate over the TICK_EXP_CACHE map and get all entries
pub fn get_tick_exp_cache(storage: &mut dyn Storage) -> Result<Vec<i64>, ContractError> {
    let keys: Vec<i64> = TICK_EXP_CACHE
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.unwrap().0)
        .collect();

    Ok(keys)
}

// TODO: Move this entrypoint to another place like execute.rs
// Iterate over the TICK_EXP_CACHE map and remove each entry to purge the cached entries
pub fn purge_tick_exp_cache(storage: &mut dyn Storage) -> Result<Response, ContractError> {
    // TODO: -> assert_admin(deps, caller); if we leave this exposed

    let keys: Vec<i64> = TICK_EXP_CACHE
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.unwrap().0)
        .collect();

    for key in keys {
        TICK_EXP_CACHE.remove(storage, key);
    }

    Ok(Response::default())
}

pub fn build_tick_exp_cache(storage: &mut dyn Storage) -> Result<(), ContractError> {
    // Build positive indices
    let mut max_price = Decimal256::one();
    let mut cur_exp_index = 0i64;

    while max_price < Decimal256::from_str(MAX_SPOT_PRICE)? {
        let tick_exp_index_data = TickExpIndexData {
            initial_price: pow_ten_internal_dec_256(cur_exp_index)?,
            max_price: pow_ten_internal_dec_256(cur_exp_index + 1)?,
            additive_increment_per_tick: pow_ten_internal_dec_256(
                EXPONENT_AT_PRICE_ONE + cur_exp_index,
            )?,
            initial_tick: (9u128
                .checked_mul(pow_ten_internal_u128(-EXPONENT_AT_PRICE_ONE)?)
                .ok_or(ContractError::Overflow {})? as i64)
                .checked_mul(cur_exp_index)
                .ok_or(ContractError::Overflow {})?,
        };
        TICK_EXP_CACHE.save(storage, cur_exp_index, &tick_exp_index_data)?;

        max_price = tick_exp_index_data.max_price;
        cur_exp_index += 1;
    }

    // Build negative indices
    let mut min_price = Decimal256::one();
    cur_exp_index = -1;
    while min_price > Decimal256::from_str(MIN_SPOT_PRICE)? {
        let initial_price = pow_ten_internal_dec_256(cur_exp_index)?;
        let max_price = pow_ten_internal_dec_256(cur_exp_index + 1)?;
        let additive_increment_per_tick =
            pow_ten_internal_dec_256(EXPONENT_AT_PRICE_ONE + cur_exp_index)?;
        let initial_tick = (9u128
            .checked_mul(pow_ten_internal_u128(-EXPONENT_AT_PRICE_ONE)?)
            .ok_or(ContractError::Overflow {})? as i64)
            .checked_mul(cur_exp_index)
            .ok_or(ContractError::Overflow {})?;

        let tick_exp_index_data = TickExpIndexData {
            initial_price,
            max_price,
            additive_increment_per_tick,
            initial_tick,
        };
        TICK_EXP_CACHE.save(storage, cur_exp_index, &tick_exp_index_data)?;

        min_price = tick_exp_index_data.initial_price;
        cur_exp_index -= 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_test_tube_tick_to_price() {
        let mut deps = mock_dependencies();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

        // example1
        let tick_index = 27445000_i128;
        let _expected_price = Decimal256::from_str("30352").unwrap();
        let price = tick_to_price(tick_index.try_into().unwrap()).unwrap();
        // assert_eq!(price, expected_price);
        let tick = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, tick)
    }

    #[test]
    fn test_tick_to_price() {
        // example1
        let tick_index = 38035200;
        let expected_price = Decimal256::from_str("30352").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example2
        let tick_index = 38035300;
        let expected_price = Decimal256::from_str("30353").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example3
        let tick_index = -44821000;
        let expected_price = Decimal256::from_str("0.000011790").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example4
        let tick_index = -44820900;
        let expected_price = Decimal256::from_str("0.000011791").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example5
        let tick_index = -12104000;
        let expected_price = Decimal256::from_str("0.068960").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example6
        let tick_index = -12103900;
        let expected_price = Decimal256::from_str("0.068961").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example7
        let tick_index = MAX_TICK as i64 - 100;
        let expected_price =
            Decimal256::from_str("99999000000000000000000000000000000000").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example8
        let tick_index = MAX_TICK as i64;
        let expected_price = Decimal256::from_str(MAX_SPOT_PRICE).unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example9
        let tick_index = -20594000;
        let expected_price = Decimal256::from_str("0.007406").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example10
        let tick_index = -20593900;
        let expected_price = Decimal256::from_str("0.0074061").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example11
        let tick_index = -29204000;
        let expected_price = Decimal256::from_str("0.00077960").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example12
        let tick_index = -29203900;
        let expected_price = Decimal256::from_str("0.00077961").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example13
        let tick_index = -12150000;
        let expected_price = Decimal256::from_str("0.068500").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example14
        let tick_index = -12149900;
        let expected_price = Decimal256::from_str("0.068501").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example15
        let tick_index = 64576000;
        let expected_price = Decimal256::from_str("25760000").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example16
        let tick_index = 64576100;
        let expected_price = Decimal256::from_str("25761000").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example17
        let tick_index = 0;
        let expected_price = Decimal256::from_str("1").unwrap();
        let price = tick_to_price(tick_index).unwrap();
        assert_eq!(price, expected_price);

        // example19
        assert!(tick_to_price(MAX_TICK as i64 + 1).is_err());

        // example20
        assert!(tick_to_price(MIN_INITIALIZED_TICK - 1).is_err());
    }

    #[test]
    fn test_price_to_tick() {
        let mut deps = mock_dependencies();
        build_tick_exp_cache(deps.as_mut().storage).unwrap();

        // example1
        let mut price = Decimal256::from_str("30352").unwrap();
        let mut expected_tick_index = 38035200;
        let mut tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example2
        price = Decimal256::from_str("30353").unwrap();
        expected_tick_index = 38035300;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(expected_tick_index, tick_index);

        // example3
        price = Decimal256::from_str("0.000011790").unwrap();
        expected_tick_index = -44821000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(expected_tick_index, tick_index);

        // example4
        price = Decimal256::from_str("0.000011791").unwrap();
        expected_tick_index = -44820900;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example5
        price = Decimal256::from_str("0.068960").unwrap();
        expected_tick_index = -12104000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example6
        price = Decimal256::from_str("0.068961").unwrap();
        expected_tick_index = -12103900;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example7
        price = Decimal256::from_str("99999000000000000000000000000000000000").unwrap();
        expected_tick_index = MAX_TICK - 100;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example8
        price = Decimal256::from_str(MAX_SPOT_PRICE).unwrap();
        expected_tick_index = MAX_TICK;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example9
        price = Decimal256::from_str("0.007406").unwrap();
        expected_tick_index = -20594000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example10
        price = Decimal256::from_str("0.0074061").unwrap();
        expected_tick_index = -20593900;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example11
        price = Decimal256::from_str("0.00077960").unwrap();
        expected_tick_index = -29204000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example12
        price = Decimal256::from_str("0.00077961").unwrap();
        expected_tick_index = -29203900;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example13
        price = Decimal256::from_str("0.068500").unwrap();
        expected_tick_index = -12150000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example14
        price = Decimal256::from_str("0.068501").unwrap();
        expected_tick_index = -12149900;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example15
        price = Decimal256::from_str("25760000").unwrap();
        expected_tick_index = 64576000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example16
        price = Decimal256::from_str("25761000").unwrap();
        expected_tick_index = 64576100;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example17
        price = Decimal256::from_str("1").unwrap();
        expected_tick_index = 0;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example18: (won't work)... Decimal256 cannot be negative
        assert!(Decimal256::from_str("-1").is_err());

        price = Decimal256::from_str("4.169478164938714112").unwrap();
        expected_tick_index = 3169478;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        price = Decimal256::from_str("2.101924006248355072").unwrap();
        expected_tick_index = 1101924;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        price = Decimal256::from_str("0.007406").unwrap();
        expected_tick_index = -20594000;
        tick_index = price_to_tick(deps.as_mut().storage, price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example19
        price = Decimal256::from_str(MAX_SPOT_PRICE).unwrap() + Decimal256::one();
        assert!(price_to_tick(deps.as_mut().storage, price).is_err());

        // example20
        price = Decimal256::from_str(MIN_SPOT_PRICE).unwrap() / Decimal256::from_str("10").unwrap();
        assert!(price_to_tick(deps.as_mut().storage, price).is_err());
    }
}
