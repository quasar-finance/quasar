use cosmwasm_std::{Decimal as StdDecimal, Uint128};
use crate::traits::Curve;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Constant { value: Uint128, scale: u32 },
}

// TODO see if we want to add in curve_fn to deposit and withdraw similar to cw20-bonding
pub type CurveFn = Box<dyn Fn(DecimalPlaces) -> Box<dyn Curve>>;

impl CurveType {
    pub fn to_curve_fn(&self) -> CurveFn {
        let s = self.clone();
        match s {
            CurveType::Constant { value, scale } => {
                let calc = move |places| -> Box<dyn Curve> {
                    Box::new(Constant::new(decimal(value, scale), places))
                };
                Box::new(calc)
            } // CurveType::Linear { slope, scale } => {
              //     let calc = move |places| -> Box<dyn Curve> {
              //         Box::new(Linear::new(decimal(slope, scale), places))
              //     };
              //     Box::new(calc)
              // }
              // CurveType::SquareRoot { slope, scale } => {
              //     let calc = move |places| -> Box<dyn Curve> {
              //         Box::new(SquareRoot::new(decimal(slope, scale), places))
              //     };
              //     Box::new(calc)
              // }
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub value: Decimal,
    pub normalize: DecimalPlaces,
}

impl Curve for Constant {
    /// price returns the current price, equal to f(x)
    fn price(&self, _supply: &Uint128) -> StdDecimal {
        decimal_to_std(self.value)
    }

    /// returns the amount of shares gotten for amount of reserve tokens, equal to F(x)
    fn deposit(&self, amount: &Uint128) -> Uint128 {
        // f(x) = supply * self.value
        let shares = self.normalize.from_supply(*amount) * self.value;
        self.normalize.to_reserve(shares)
    }

    /// returns the amount of reserve tokens that should be returned for the shares, equal to F^-1(x)
    fn withdraw(&self, shares: &Uint128) -> Uint128 {
        // f(x) = reserve / self.value
        let amount = self.normalize.from_reserve(*shares) / self.value;
        self.normalize.to_supply(amount)
    }
}

impl Constant {
    pub fn new(value: Decimal, normalize: DecimalPlaces) -> Self {
        Self { value, normalize }
    }
}

/// DecimalPlaces should be passed into curve constructors
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, JsonSchema, Default)]
pub struct DecimalPlaces {
    /// Number of decimal places for the supply token (this is what was passed in cw20-base instantiate
    pub supply: u32,
    /// Number of decimal places for the reserve token (eg. 6 for uatom, 9 for nstep, 18 for wei)
    pub reserve: u32,
}

impl DecimalPlaces {
    pub fn new(supply: u8, reserve: u8) -> Self {
        DecimalPlaces {
            supply: supply as u32,
            reserve: reserve as u32,
        }
    }

    pub fn to_reserve(self, reserve: Decimal) -> Uint128 {
        let factor = decimal(10u128.pow(self.reserve), 0);
        let out = reserve * factor;
        // TODO: execute overflow better? Result?
        out.floor().to_u128().unwrap().into()
    }

    pub fn to_supply(self, supply: Decimal) -> Uint128 {
        let factor = decimal(10u128.pow(self.supply), 0);
        let out = supply * factor;
        // TODO: execute overflow better? Result?
        out.floor().to_u128().unwrap().into()
    }

    pub fn from_supply(&self, supply: Uint128) -> Decimal {
        decimal(supply, self.supply)
    }

    pub fn from_reserve(&self, reserve: Uint128) -> Decimal {
        decimal(reserve, self.reserve)
    }
}

/// StdDecimal stores as a u128 with 18 decimal points of precision
fn decimal_to_std(x: Decimal) -> StdDecimal {
    // this seems straight-forward (if inefficient), converting via string representation
    // TODO: execute errors better? Result?
    StdDecimal::from_str(x.to_string().as_str()).unwrap()
}

/// decimal returns an object = num * 10 ^ -scale
/// We use this function in contract.rs rather than call the crate constructor
/// itself, in case we want to swap out the implementation, we can do it only in this file.
pub fn decimal<T: Into<u128>>(num: T, scale: u32) -> Decimal {
    Decimal::from_i128_with_scale(num.into() as i128, scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_curve() {
        // supply is nstep (9), reserve is uatom (6)
        let normalize = DecimalPlaces::new(9, 6);
        // decimal is 1.5
        let curve = Constant::new(Decimal::new(15i64, 1), normalize);

        // do some sanity checks....
        // spot price is always 1.5 ATOM
        assert_eq!(StdDecimal::percent(150), curve.price(&Uint128::new(123)));

        // if we have 30 STEP, we should have 45 ATOM
        let reserve = curve.deposit(&Uint128::new(30_000_000_000));
        assert_eq!(Uint128::new(45_000_000), reserve);

        // if we have 36 ATOM, we should have 24 STEP
        let supply = curve.withdraw(&Uint128::new(36_000_000));
        assert_eq!(Uint128::new(24_000_000_000), supply);
    }
}
