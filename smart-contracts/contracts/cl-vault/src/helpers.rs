use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, Decimal, StdResult, Uint128, WasmMsg};

use crate::{msg::ExecuteMsg, ContractError};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
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
        return Ok(1u128
            / 10u128
                .checked_pow(exponent as u32)
                .ok_or(ContractError::Overflow {})?);
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

// exponent_at_current_price_one is fixed at -6?
// we assume exp is always neg
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

    let exponen_at_current_tick: i128 = exponent_at_price_one + geometric_exponential_delta as i128;

    // TODO: tick_index should always be positive, right? Osmosis go code has a check for it.

    let current_additive_increment_in_ticks = pow_ten_internal_dec(exponen_at_current_tick)?;

    let num_additive_ticks: u128 = tick_index.u128()
        - geometric_exponential_delta * geometric_exponent_increment_distance_in_ticks;

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
    let mut current_price = Decimal::one();
    let mut ticks_passed: Uint128 = Uint128::zero();

    let geometric_exponent_increment_distance_in_ticks = 9u128
        .checked_mul(10u128.pow(exponent_at_price_one.abs() as u32))
        .ok_or(ContractError::Overflow {})?;

    // TODO: need this to live after the loop, is there a better way?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_to_price() {
        let tick_index = Uint128::new(36650010u128);
        let exponen_at_current_price_one = -6;
        let expected_price = Decimal::checked_from_ratio(165001u128, 10u128);
        let price = tick_to_price(tick_index, exponen_at_current_price_one).unwrap();
        assert_eq!(price, expected_price.unwrap());
    }

    #[test]
    fn test_price_to_tick() {
        let price = Decimal::checked_from_ratio(165001u128, 10u128).unwrap();
        let exponen_at_current_price_one = -6;
        let expected_tick_index = Uint128::new(36650010u128);
        let tick_index = price_to_tick(price, exponen_at_current_price_one).unwrap();
        assert_eq!(tick_index, expected_tick_index);
    }
}
