use std::str::FromStr;

use cosmwasm_std::{Decimal, Decimal256, DepsMut, Uint128, Uint256};

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

// THIS IS TRYING TO REPLICATE OSMOSIS GO LOGIC BUT MATH IS A BIT OFF
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

pub fn price_to_tick(mut deps: DepsMut, price: Decimal256) -> Result<Uint256, ContractError> {
    if price > Decimal256::from_str(MAX_SPOT_PRICE)?
        || price < Decimal256::from_str(MIN_SPOT_PRICE)?
    {
        return Err(ContractError::PriceBoundError { price });
    }
    if price == Decimal256::one() {
        return Ok(Uint256::zero());
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
        let price_in_this_exponent = price - geo_spacing.initial_price;
        let ticks_filled_by_current_spacing =
            price_in_this_exponent / geo_spacing.additive_increment_per_tick;
        let tick_index = Decimal256::from_ratio(geo_spacing.initial_tick as u128, 1u128)
            .checked_add(ticks_filled_by_current_spacing)?;

        return Ok(tick_index.to_uint_floor());
    } else {
        let mut index = -1;
        geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        while geo_spacing.initial_price > price {
            index -= 1;
            geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        }
        println!("###DEBUGGING###");
        println!("price: {:?}", price);
        println!("geo_spacing: {:?}", geo_spacing);

        let price_in_this_exponent = price - geo_spacing.initial_price;
        println!("price_in_this_exponent: {:?}", price_in_this_exponent);

        let ticks_filled_by_current_spacing =
            price_in_this_exponent / geo_spacing.additive_increment_per_tick;
        println!(
            "ticks_filled_by_current_spacing: {:?}",
            ticks_filled_by_current_spacing
        );

        println!("geo_spacing.initial_tick: {:?}", geo_spacing.initial_tick);

        println!("###DEBUGGING###");
        println!(
            "{} - {}",
            Decimal256::from_ratio(geo_spacing.initial_tick as u128, 1u128),
            ticks_filled_by_current_spacing
        );

        let tick_index = Decimal256::from_ratio(geo_spacing.initial_tick as u128, 1u128)
            .checked_sub(ticks_filled_by_current_spacing)?;

        Ok(tick_index.to_uint_floor())
    }
}

// TODO: hashmaps for CW maps?
pub fn price_to_tick_signed(mut deps: DepsMut, price: Decimal256) -> Result<i128, ContractError> {
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
        let price_in_this_exponent = price - geo_spacing.initial_price;
        let ticks_filled_by_current_spacing =
            price_in_this_exponent / geo_spacing.additive_increment_per_tick;
        let tick_index = Decimal256::from_ratio(geo_spacing.initial_tick as u128, 1u128)
            .checked_add(ticks_filled_by_current_spacing)?;
        let tick_i128 = decimal256_to_i128(tick_index).ok_or(ContractError::Overflow {})?;

        return Ok(tick_i128);
    } else {
        let mut index = -1;
        geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        while geo_spacing.initial_price > price {
            index -= 1;
            geo_spacing = TICK_EXP_CACHE.load(deps.storage, index)?;
        }

        let price_in_this_exponent = price - geo_spacing.initial_price;

        let ticks_filled_by_current_spacing =
            price_in_this_exponent / geo_spacing.additive_increment_per_tick;
        let tick_index = geo_spacing.initial_tick as i128
            + decimal256_to_i128(ticks_filled_by_current_spacing)
                .ok_or(ContractError::Overflow {})?;

        Ok(tick_index)
    }
}

fn decimal256_to_i128(decimal: Decimal256) -> Option<i128> {
    if decimal > Decimal256::from_str(&i128::MAX.to_string()).unwrap() {
        return None;
    }
    // Convert Decimal256 to a string
    let decimal_str = decimal.to_string();

    // Split the string at the decimal point and take the integer part
    let integer_part = decimal_str.split('.').next()?;

    // Parse the integer part to i128
    integer_part.parse::<i128>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::mock_dependencies, Uint128};

    #[test]
    fn test_decimal_256_to_i128() {
        // zero
        let decimal = Decimal256::zero();
        let i128 = decimal256_to_i128(decimal).unwrap();
        assert_eq!(i128, 0);
        // small number
        let decimal = Decimal256::from_str("123456").unwrap();
        let i128 = decimal256_to_i128(decimal).unwrap();
        assert_eq!(i128, 123456);
        // small decimal
        let decimal = Decimal256::from_str("123456.123456").unwrap();
        let i128 = decimal256_to_i128(decimal).unwrap();
        assert_eq!(i128, 123456);
        // large number
        let decimal = Decimal256::from_str("170141183460469231731687303715884105728").unwrap();
        let i128 = decimal256_to_i128(decimal);
        assert!(i128.is_none());
        // large decimal
        let decimal =
            Decimal256::from_str("170141183460469231731687303715884105728.923919239129391293")
                .unwrap();
        let i128 = decimal256_to_i128(decimal);
        assert!(i128.is_none());
    }

    #[test]
    fn test_price_to_tick_signed() {
        let mut deps = mock_dependencies();
        // example1
        let mut price = Decimal256::from_str("0.000011790").unwrap();
        let mut expected_tick_index = -44821000;
        let tick_index = price_to_tick_signed(deps.as_mut(), price).unwrap();
        assert_eq!(expected_tick_index, tick_index);
    }

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
        let mut price = Decimal256::from_str("16500.1").unwrap();
        let mut expected_tick_index = Uint256::from_u128(36650010u128);
        let mut tick_index = price_to_tick(deps.as_mut(), price.into()).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example2
        price = Decimal256::from_str("30352").unwrap();
        expected_tick_index = Uint256::from_u128(38035200u128);
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example3
        // price = Decimal256::from_str("1.000030").unwrap();
        // expected_tick_index = Uint256::from_u128(0u128);
        // tick_index = price_to_tick_3(deps.as_mut(), price).unwrap();
        // assert_eq!(tick_index, expected_tick_index);

        // example4
        price = Decimal256::from_str("30353").unwrap();
        expected_tick_index = Uint256::from_u128(38035300u128);
        tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        assert_eq!(tick_index, expected_tick_index);

        // example5
        // price = Decimal256::from_str("0.000011790").unwrap();
        // expected_tick_index = Uint256::from_u128(44821000u128);
        // tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        // assert_eq!(tick_index, expected_tick_index);

        // // example4
        // price = Decimal256::from_str("30353").unwrap();
        // expected_tick_index = Uint256::from_u128(38035300u128);
        // tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        // assert_eq!(tick_index, expected_tick_index);
        // // example4
        // price = Decimal256::from_str("30353").unwrap();
        // expected_tick_index = Uint256::from_u128(38035300u128);
        // tick_index = price_to_tick(deps.as_mut(), price).unwrap();
        // assert_eq!(tick_index, expected_tick_index);
    }
}
