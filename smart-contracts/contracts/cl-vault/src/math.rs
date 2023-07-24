use std::ops::Mul;

use cosmwasm_std::{Decimal, StdError, Decimal256};

use crate::ContractError;

const big_factor: Decimal256 = Decimal256::raw(100000000);

pub fn liquidity0(amount: Decimal, sqrt_price_a: Decimal, sqrt_price_b: Decimal) -> Result<Decimal256, ContractError> {
    let mut sqrt_price_a = Decimal256::raw(sqrt_price_a.atomics().u128()).checked_mul(big_factor)?;
    let mut sqrt_price_b = Decimal256::raw(sqrt_price_b.atomics().u128()).checked_mul(big_factor)?;
    let amount = Decimal256::from(amount).checked_mul(big_factor)?;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let product = sqrt_price_a.checked_mul(sqrt_price_b)?; 
    // let product = Uint256::from(sqrt_price_a.atomics().u128()).checked_mul(Uint256::from(sqrt_price_b.atomics().u128()))?;
    let diff = sqrt_price_b.checked_sub(sqrt_price_a)?;
    println!("{:?}", diff);


    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("liquidity0 diff is zero")));
    }

    // println!("product: {:?}", product);
    // println!("diff: {:?}", diff);

    // let intermediate = amount.atomics().checked_mul(product)?;
    // println!("amount*product: {:?}", amount.atomics().checked_mul(product)?);

    // let intermediate_decimal = Decimal256::new(intermediate);
    // println!("{:?}", intermediate_decimal);
    // println!("{:?}", intermediate_decimal.checked_div(diff.into()).unwrap());

    // during this check mul, the result is being truncated and giving is a different final result than expected
    let result = amount.checked_mul(product)?.checked_div(diff)?;
    Ok(result)
}

pub fn liquidity1(
    amount: Decimal,
    sqrt_price_a: Decimal,
    sqrt_price_b: Decimal,
) -> Result<Decimal, ContractError> {
    let mut sqrt_price_a = sqrt_price_a;
    let mut sqrt_price_b = sqrt_price_b;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b
        .checked_sub(sqrt_price_a)
        .map_err(|err| StdError::generic_err(err.to_string()))?;
    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("liquidity1 diff is zero")));
    }

    let result = amount.checked_div(diff)?;
    Ok(result)
}

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
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};

    // #[test]
    // fn math () {
    //     let val = Uint256::from(70710678118654752440_u128)* Uint256::from(74161984870956629487_u128);
    //     println!("{:?}", val)   
    // }

    #[test]
    fn test_liquidity0() {
        // from the osmosis math tests
        // current_sqrt_p:      sqrt5000BigDec, // 5000
        // sqrtPHigh:         sqrt5500BigDec, // 5500
        // amount0Desired:    sdk.NewInt(1000000),
        // expectedLiquidity: "1519437308.014768571720923239",
        let amount0_desired = Decimal::from_ratio(1000000_u128, 1_u128);
        let current_sqrt_p = Decimal::from_atomics(7071067811865475244000000000_u128, 18).unwrap();
        let sqrt_p_high = Decimal::from_atomics(7416198487095662948700000000_u128, 18).unwrap();

        let result = liquidity0(amount0_desired.into(), current_sqrt_p.into(), sqrt_p_high.into()).unwrap();
        // TODO our amount is slightly different 10 digits behind the comma, do we care about that?
        assert_eq!(result.to_string(), "1519437308.014768571720923239")
    }

    #[test]
    fn test_liquidity1() {
        let amount1_desired = Decimal::from_atomics(5000000000_u128, 0).unwrap();
        let current_sqrt_p = Decimal::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_p_low = Decimal::from_atomics(67416615162732695594_u128, 18).unwrap();

        let result = liquidity1(amount1_desired, current_sqrt_p, sqrt_p_low).unwrap();
        assert_eq!(result.to_string(), "1517882343.751510418088349649");
    }
}
