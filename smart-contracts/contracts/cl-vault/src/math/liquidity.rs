use crate::{helpers::PositionRatio, ContractError};
use cosmwasm_std::{Decimal256, StdError, Uint128, Uint256, Uint512};

use super::helpers::abs_diff;

/// liquidity0 calculates the amount of liquitiy gained from adding an amount of token0 to a position
pub fn _liquidity0(
    amount: Decimal256,
    sqrt_price_a: Decimal256,
    sqrt_price_b: Decimal256,
) -> Result<Decimal256, ContractError> {
    let mut sqrt_price_a: Uint512 = sqrt_price_a.atomics().into();
    let mut sqrt_price_b: Uint512 = sqrt_price_b.atomics().into();
    let amount: Uint512 = amount.atomics().into();

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let product = sqrt_price_a.checked_mul(sqrt_price_b)?;
    // let product = Uint256::from(sqrt_price_a.atomics().u128()).checked_mul(Uint256::from(sqrt_price_b.atomics().u128()))?;
    let diff = sqrt_price_b.checked_sub(sqrt_price_a)?;

    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "liquidity0 diff is zero",
        )));
    }

    // during this check mul, the result is being truncated and giving is a different final result than expected
    let result = amount.checked_mul(product)?.checked_div(diff)?;
    // convert the Uint512 back to a decimal, we want to place the decimal at decimal_place 36
    // to do this, we truncate the first 18 digits, and then call Decimal::new
    // Should we check here that the leftover bytes are zero? that is technically an overflow
    let result_bytes: [u8; 64] = result.to_le_bytes();
    for b in result_bytes[32..64].iter() {
        if b != &0_u8 {
            return Err(ContractError::Overflow {});
        }
    }
    let intermediate = Uint256::from_le_bytes(result_bytes[..32].try_into().unwrap());
    // we use Decimal256 to
    let intermediate_2 = Decimal256::from_atomics(intermediate, 36).unwrap();

    // since we start with Decimal and multiply with big_factor, we expect to be able to convert back here
    Ok(Decimal256::new(intermediate_2.atomics()))
}

/// asset0 calculates the
pub fn asset0(
    liquidity: Decimal256,
    sqrt_price_a: Decimal256,
    sqrt_price_b: Decimal256,
) -> Result<Uint128, ContractError> {
    let mut sqrt_price_a = sqrt_price_a.atomics();
    let mut sqrt_price_b = sqrt_price_b.atomics();
    let liquidity = liquidity.atomics();

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b.checked_sub(sqrt_price_a)?;

    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "liquidity0 diff is zero",
        )));
    }

    let product = sqrt_price_a.checked_mul(sqrt_price_b)?;

    let total = liquidity.checked_mul(diff)?.checked_div(product)?;

    Ok(total.try_into()?)
}

pub fn asset1(
    liquidity: Decimal256,
    sqrt_price_a: Decimal256,
    sqrt_price_b: Decimal256,
) -> Result<Uint128, ContractError> {
    let mut sqrt_price_a = sqrt_price_a.atomics();
    let mut sqrt_price_b = sqrt_price_b.atomics();
    let liquidity = liquidity.atomics();

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b.checked_sub(sqrt_price_a)?;

    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "liquidity0 diff is zero",
        )));
    }

    let total = liquidity.checked_div(diff)?;

    Ok(total.try_into()?)
}

pub fn get_position_ratio(
    sqrt_price_lower: Decimal256,
    sqrt_price_current: Decimal256,
    sqrt_price_upper: Decimal256,
) -> Result<PositionRatio, ContractError> {
    let sqrt_price_lower = sqrt_price_lower.atomics();
    let sqrt_price_current = sqrt_price_current.atomics();
    let sqrt_price_upper = sqrt_price_upper.atomics();

    let diff_upper = abs_diff(sqrt_price_upper, sqrt_price_current);
    let diff_lower = abs_diff(sqrt_price_lower, sqrt_price_current);

    let prod_upper = sqrt_price_current.checked_mul(sqrt_price_upper)?;

    if diff_upper.is_zero() || diff_lower.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "liquidity0 diff is zero",
        )));
    }

    let numerator = diff_upper.checked_mul(diff_lower)?;

    Ok(PositionRatio::new(
        numerator.try_into()?,
        prod_upper.try_into()?,
    ))
}

// TODO figure out if liquidity1 need to be Uint512's aswell, currently I (Laurens) don't believe so since we should only need more precision if we multiply decimals
/// liquidity1 calculates the amount of liquitiy gained from adding an amount of token1 to a position
pub fn _liquidity1(
    amount: Decimal256,
    sqrt_price_a: Decimal256,
    sqrt_price_b: Decimal256,
) -> Result<Decimal256, ContractError> {
    let mut sqrt_price_a = sqrt_price_a;
    let mut sqrt_price_b = sqrt_price_b;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b
        .checked_sub(sqrt_price_a)
        .map_err(|err| StdError::generic_err(err.to_string()))?;
    if diff.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "liquidity1 diff is zero",
        )));
    }

    let result = amount.checked_div(diff)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidity0() {
        // from the osmosis math tests
        // current_sqrt_p:      sqrt5000BigDec, // 5000
        // sqrtPHigh:         sqrt5500BigDec, // 5500
        // amount0Desired:    sdk.NewInt(1000000),
        // expectedLiquidity: "1519437308.014768571720923239",
        let amount0_desired = Decimal256::from_ratio(1000000_u128, 1_u128);
        let current_sqrt_p = Decimal256::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_p_high = Decimal256::from_atomics(74161984870956629487_u128, 18).unwrap();

        let result = _liquidity0(amount0_desired, current_sqrt_p, sqrt_p_high).unwrap();
        // TODO our amount is slightly different 10 digits behind the comma, do we care about that?
        assert_eq!(result.to_string(), "1519437308.014768571720923239")
    }

    #[test]
    fn test_liquidity1() {
        let amount1_desired = Decimal256::from_atomics(5000000000_u128, 0).unwrap();
        let current_sqrt_p = Decimal256::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_p_low = Decimal256::from_atomics(67416615162732695594_u128, 18).unwrap();

        let result = _liquidity1(amount1_desired, current_sqrt_p, sqrt_p_low).unwrap();
        assert_eq!(result.to_string(), "1517882343.751510418088349649");
    }

    #[test]
    fn test_max_liquidity0() {
        let max_sqrt_price = Decimal256::raw(10000000000000000000000000000000000000_u128);
        let max_sqrt_price_low = Decimal256::raw(300000000000000000000000000000000_u128);
        let amount0_desired = Decimal256::from_ratio(1000000_u128, 1_u128);
        // we only care about overflows here
        let _ = _liquidity0(amount0_desired, max_sqrt_price, max_sqrt_price_low).unwrap();
    }

    #[test]
    fn test_max_liquidity1() {
        let max_sqrt_price = Decimal256::raw(10000000000000000000000000000000000000_u128);
        let max_sqrt_price_low = Decimal256::raw(1000000000000000000000000000000000000_u128);
        let amount0_desired = Decimal256::from_ratio(1000000_u128, 1_u128);
        // we only care about overflows here
        let _ = _liquidity1(amount0_desired, max_sqrt_price, max_sqrt_price_low).unwrap();
    }
}
