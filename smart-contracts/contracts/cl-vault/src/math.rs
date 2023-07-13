use cosmwasm_std::{Decimal, StdError, Decimal256};

use crate::ContractError;

pub fn liquidity0(amount: Decimal256, sqrt_price_a: Decimal256, sqrt_price_b: Decimal256) -> Result<Decimal256, ContractError> {
    let mut sqrt_price_a = sqrt_price_a;
    let mut sqrt_price_b = sqrt_price_b;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let product = sqrt_price_a.checked_mul(sqrt_price_b)?;
    let diff = sqrt_price_b
    .checked_sub(sqrt_price_a)?;

    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err("liquidity0 diff is zero")));
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};

    #[test]
    fn test_liquidity0() {
        // from the osmosis math tests
        // current_sqrt_p:      sqrt5000BigDec, // 5000
        // sqrtPHigh:         sqrt5500BigDec, // 5500
        // amount0Desired:    sdk.NewInt(1000000),
        // expectedLiquidity: "1519437308.014768571720923239",
        let amount0_desired = Decimal::from_atomics(1000000_u128, 0).unwrap();
        let current_sqrt_p = Decimal::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_p_high = Decimal::from_atomics(74161984870956629487_u128, 18).unwrap();

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
