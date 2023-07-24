use crate::ContractError;
use cosmwasm_std::{Decimal, Decimal256, StdError, Uint128, Uint256, Uint512};

// calcActualAmounts calculates and returns actual amounts based on where the current tick is located relative to position's
// lower and upper ticks.
// There are 3 possible cases:
// -The position is active ( lowerTick <= p.CurrentTick < upperTick).
//   - The provided liquidity is distributed in both tokens.
//   - Actual amounts might differ from desired because we recalculate them from liquidity delta and sqrt price.
//
// - Current tick is below the position ( p.CurrentTick < lowerTick).
//   - The provided liquidity is distributed in token0 only.
//
// - Current tick is above the position ( p.CurrentTick >= p.upperTick ).
//   - The provided liquidity is distributed in token1 only.
//
// Note, that liquidityDelta can be positive or negative but cannot be zero.
// If zero, an error is returned.
// If positive, we assume, liquidity being added. As a result, we round up so that
// we request a user to add more liquidity in favor of the pool.
// If negative, we assume, liquidity being removed. As a result, we round down so that
// we request a user to remove less liquidity in favor of the pool.
// -The position is active ( lowerTick <= p.CurrentTick < upperTick).
//   - The provided liquidity is distributed in both tokens.
//   - Actual amounts might differ from desired because we recalculate them from liquidity delta and sqrt price.

/// calc_amount0_delta calculates the amount of token0 between two prices. token0 is the token
fn calc_amount0_delta(
    liq: Decimal256,
    sqrt_price_a: Decimal256,
    sqrt_price_b: Decimal256,
) -> Result<Decimal256, ContractError> {
    let mut sqrt_price_a = sqrt_price_a;
    println!("{:?}", sqrt_price_a);
    let mut sqrt_price_b = sqrt_price_b;
    println!("{:?}", sqrt_price_b);
    let liq = liq;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b - sqrt_price_a;
    let denom = sqrt_price_a.checked_mul(sqrt_price_b)?;
    let result = liq * diff / denom;
    Ok(result)
}

fn calc_amount1_delta(
    liq: Decimal,
    sqrt_price_a: Decimal,
    sqrt_price_b: Decimal,
    round_up: bool,
) -> Result<Decimal, ContractError> {
    let mut sqrt_price_a = sqrt_price_a;
    let mut sqrt_price_b = sqrt_price_b;

    if sqrt_price_a > sqrt_price_b {
        std::mem::swap(&mut sqrt_price_a, &mut sqrt_price_b);
    }

    let diff = sqrt_price_b - sqrt_price_a;

    if round_up {
        let result = liq * diff;
        Ok(result.ceil())
    } else {
        let result = liq * diff;
        Ok(result)
    }
}

pub fn liquidity0(
    amount: Decimal,
    sqrt_price_a: Decimal,
    sqrt_price_b: Decimal,
) -> Result<Decimal, ContractError> {
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
    Ok(Decimal::new(intermediate_2.atomics().try_into()?))
}

// TODO figure out if liquidity1 need to be Uint512's aswell, currently I (Laurens) don't believe so since we should only need more precision if we multiply decimals
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
    use cosmwasm_std::testing::{mock_env, mock_info};
    use std::str::FromStr;

    #[test]
    fn test_calc_amount0_delta_happy_path_round_down() {
        let liquidity = Decimal::from_str("1517882343.751510418088349649").unwrap();
        let sqrt_pa = Decimal::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_pb = Decimal::from_atomics(74161984870956629487_u128, 18).unwrap();
        // Original Osmosis expected value was "998976.618347426388356629", due to precision rounding
        // we expect "998976.61834742638835681", see comment below for justification
        let amount0_expected = Decimal256::from_str("998976.61834742638835681").unwrap();

        let result = calc_amount0_delta(liquidity.into(), sqrt_pa.into(), sqrt_pb.into()).unwrap();
        // we find some rounding errors vs the expected osmosis value here, how do we handle those?
        // The Decimal out given in Osmosis code is finally truncated to an int, see https://github.com/osmosis-labs/osmosis/blob/main/x/concentrated-liquidity/lp.go#L436
        // In our case then the rounding that far behind the comma should not matter for calculating user liquidity leaving
        // the vault. We just always want to make sure that we have more round down on assets leaving the vault so that
        // the user never gets more funds than shares, and round up on joining, so the user never gets more shares than funds
        assert_eq!(result, amount0_expected);
    }

    #[test]
    fn test_calc_amount0_delta_happy_path_round_up() {
        let liquidity = Decimal256::from_str("1517882343.751510418088349649").unwrap();
        let sqrt_pa = Decimal256::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_pb = Decimal256::from_atomics(74161984870956629487_u128, 18).unwrap();
        let round_up = true;
        let amount0_expected = Decimal256::from_str("998976.618347426388356630")
            .unwrap()
            .ceil();

        let result = calc_amount0_delta(liquidity, sqrt_pa, sqrt_pb).unwrap();
        assert_eq!(result, amount0_expected);
    }

    #[test]
    fn test_calc_amount0_delta_round_down_large_liquidity_amount() {
        let liquidity = Decimal256::from_str("931361973132462178951297").unwrap();
        let sqrt_pa = Decimal256::from_str("0.000000152731791058").unwrap();
        let sqrt_pb = Decimal256::from_str("30860351331.852813530648276680").unwrap();
        let round_up = false;
        let amount0_expected =
            Decimal256::from_str("6098022989717817431593106314408.88812810159039320984467945943")
                .unwrap();

        let result = calc_amount0_delta(liquidity, sqrt_pa, sqrt_pb).unwrap();
        assert_eq!(result, amount0_expected);
    }

    #[test]
    fn test_calc_amount0_delta_round_up_large_liquidity_amount() {
        let liquidity = Decimal256::from_str("931361973132462178951297").unwrap();
        let sqrt_pa = Decimal256::from_str("0.000000152731791058").unwrap();
        let sqrt_pb = Decimal256::from_str("30860351331.852813530648276680").unwrap();
        let round_up = true;
        let amount0_expected =
            Decimal256::from_str("6098022989717817431593106314408.88812810159039320984467945943")
                .unwrap()
                .ceil();

        let result = calc_amount0_delta(liquidity, sqrt_pa, sqrt_pb).unwrap();
        assert_eq!(result, amount0_expected);
    }

    #[test]
    fn test_liquidity0() {
        // from the osmosis math tests
        // current_sqrt_p:      sqrt5000BigDec, // 5000
        // sqrtPHigh:         sqrt5500BigDec, // 5500
        // amount0Desired:    sdk.NewInt(1000000),
        // expectedLiquidity: "1519437308.014768571720923239",
        let amount0_desired: Decimal = Decimal::from_ratio(1000000_u128, 1_u128);
        let current_sqrt_p = Decimal::from_atomics(70710678118654752440_u128, 18).unwrap();
        let sqrt_p_high = Decimal::from_atomics(74161984870956629487_u128, 18).unwrap();

        let result = liquidity0(
            amount0_desired.into(),
            current_sqrt_p.into(),
            sqrt_p_high.into(),
        )
        .unwrap();
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

    #[test]
    fn test_max_liquidity0() {
        let max_sqrt_price = Decimal::raw(10000000000000000000000000000000000000_u128);
        let max_sqrt_price_low = Decimal::raw(300000000000000000000000000000000_u128);
        let amount0_desired: Decimal = Decimal::from_ratio(1000000_u128, 1_u128);
        // we only care about overflows here
        let _ = liquidity0(amount0_desired.into(), max_sqrt_price, max_sqrt_price_low).unwrap();
    }

    #[test]
    fn test_max_liquidity1() {
        let max_sqrt_price = Decimal::raw(10000000000000000000000000000000000000_u128);
        let max_sqrt_price_low = Decimal::raw(1000000000000000000000000000000000000_u128);
        let amount0_desired: Decimal = Decimal::from_ratio(1000000_u128, 1_u128);
        // we only care about overflows here
        let _ = liquidity1(amount0_desired.into(), max_sqrt_price, max_sqrt_price_low).unwrap();
    }
}
