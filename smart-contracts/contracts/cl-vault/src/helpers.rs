use std::str::FromStr;

use crate::{state::POOL_CONFIG, ContractError};
use cosmwasm_std::{Decimal, Fraction, QuerierWrapper, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;

// due to pow restrictions we need to use unsigned integers; i.e. 10.pow(-exp: u32)
// so if the resulting power is positive, we take 10**exp;
// and if it is negative, we take 1/10**exp.
fn pow_ten_internal(exponent: i128) -> Result<u128, ContractError> {
    if exponent >= 0 {
        10u128
            .checked_pow(exponent.unsigned_abs() as u32)
            .ok_or(ContractError::Overflow {})
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
        .checked_pow(exponent.unsigned_abs() as u32)
        .ok_or(ContractError::Overflow {})?;
    if exponent >= 0 {
        Ok(Decimal::from_ratio(p, 1u128))
    } else {
        Ok(Decimal::from_ratio(1u128, p))
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
            Uint128::from(10u128.pow(exponent_at_price_one.unsigned_abs() as u32)),
        );

        exponent_at_price_one += 1;

        let max_price_for_current_increment_in_ticks = current_additive_increment_in_ticks
            .checked_mul(Decimal::from_ratio(
                geometric_exponent_increment_distance_in_ticks,
                1u128,
            ))?;

        ticks_passed += Uint128::new(geometric_exponent_increment_distance_in_ticks);

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
            Uint128::from(10u128.pow(exponent_at_price_one.unsigned_abs() as u32)),
        );

        exponent_at_price_one += 1;

        let max_price_for_current_increment_in_ticks = current_additive_increment_in_ticks
            .checked_mul(Decimal::from_ratio(
                geometric_exponent_increment_distance_in_ticks,
                1u128,
            ))?;

        ticks_passed += Uint128::new(geometric_exponent_increment_distance_in_ticks);

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

/// get_tokens_in_range
///
/// gets the total amount of each token in a given range
///
/// thanks: https://github.com/osmosis-labs/osmosis/blob/main/x/concentrated-liquidity/README.md#deducing-the-quantity-of-tokens-x-and-y-for-a-tick-range
// pub fn get_tokens_in_range(
//     storage: &dyn Storage,
//     querier: &QuerierWrapper,
//     pool_id: u64,
//     lower_tick: Uint128,
//     upper_tick: Uint128,
// ) -> Result<Uint128, ContractError> {
//     let pool_config = POOL_CONFIG.load(storage)?;

//     let cl_querier = ConcentratedliquidityQuerier::new(querier);
//     let liquidity = cl_querier.liquidity_net_in_direction(
//         pool_id,
//         pool_config.base_token,
//         lower_tick,
//         false,
//         upper_tick,
//         false,
//     )?;

//     let lower_tick_price =
//     liquidity.current_tick
// }

/// get_liquidity_needed_for_tokens
///
/// this function calculates the liquidity needed for depositing base_token and quote token amounts respectively and returns both.
/// depositing both tokens would result in a refund of the token with higher needed liquidity
///
/// thanks: https://github.com/osmosis-labs/osmosis/blob/ma * (liquidity_needed_base_token - liquidity_needed_quote_token)/ liquidity_needed_base_token
/// in(deposit_amount_0/x/concentrated-liquidity/README.md#adding-liquidity
pub fn get_liquidity_needed_for_tokens(
    delta_base_token: String,
    delta_quote_token: String,
    lower_tick: i64,
    upper_tick: i64,
) -> Result<(Uint128, Uint128), ContractError> {
    let delta_x = Uint128::from_str(&delta_base_token)?;
    let delta_y = Uint128::from_str(&delta_quote_token)?;
    // calc liquidity needed for token
    unimplemented!("get_liquidity_needed_for_tokens")
}

pub fn get_deposit_amounts_for_liquidity_needed(
    liquidity_needed_base_token: Uint128,
    liquidity_needed_quote_token: Uint128,
    base_token_amount: String,
    quote_token_amount: String,
    // i hate this type but it's arguably a good way to write this
) -> Result<((Uint128, Uint128), (Uint128, Uint128)), ContractError> {
    // calc deposit amounts for liquidity needed
    let amount_0 = Uint128::from_str(&base_token_amount)?;
    let amount_1 = Uint128::from_str(&quote_token_amount)?;

    // one of these will be zero
    let mut remainder_0;
    let mut remainder_1;

    let (deposit_amount_0, deposit_amount_1) =
        if (liquidity_needed_base_token > liquidity_needed_quote_token) {
            // scale base token amount down by L1/L0, take full amount of quote token
            let new_amount_0 =
                amount_0.multiply_ratio(liquidity_needed_quote_token, liquidity_needed_base_token);
            remainder_0 = amount_0.checked_sub(new_amount_0).unwrap();

            (new_amount_0, amount_1)
        } else {
            // scale quote token amount down by L0/L1, take full amount of base token
            let new_amount_1 =
                amount_1.multiply_ratio(liquidity_needed_base_token, liquidity_needed_quote_token);
            remainder_1 = amount_1.checked_sub(new_amount_1).unwrap();

            (amount_0, new_amount_1)
        };

    Ok((
        (deposit_amount_0, deposit_amount_1),
        (remainder_0, remainder_1),
    ))
}

pub fn with_slippage(amount: Uint128, slippage: Decimal) -> Result<Uint128, ContractError> {
    let slippage_multiplier = Decimal::one().checked_sub(slippage)?;

    let adjusted_amount = amount.checked_multiply_ratio(
        slippage_multiplier.numerator(),
        slippage_multiplier.denominator(),
    )?;

    Ok(adjusted_amount)
}

#[cfg(test)]
mod tests {
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
}
