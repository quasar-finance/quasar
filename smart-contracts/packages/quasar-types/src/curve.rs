use std::fmt::{Debug, Formatter};
use quasar_traits::traits::Curve;
use cosmwasm_std::{Decimal, Uint128};

pub enum CurveType {
    Constant
}

#[derive(Debug)]
pub struct Constant {
    pub value: Decimal,
    pub normalize: DecimalPlaces,
}

impl Curve for Constant {
    /// price returns the current price, equal to f(x)
    fn price(&self, _supply: Uint128) -> Result<Uint128, cosmwasm_std::StdError> {
        Ok(self.value)
    }

    /// returns the amount of shares gotten for amount of reserve tokens, equal to F(x)
    fn deposit(&mut self, amount: &Uint128) -> Result<Uint128, cosmwasm_std::StdError> {
        // f(x) = supply * self.value
        let reserve = self.normalize.from_supply(amount) * self.value;
        self.normalize.to_reserve(reserve)
    }

    /// returns the amount of reserve tokens that should be returned for the shares, equal to F^-1(x)
    fn withdraw(&mut self, shares: &Uint128) -> Result<Vec<Cw20Coin>, cosmwasm_std::StdError> {
        // f(x) = reserve / self.value
        let supply = self.normalize.from_reserve(reserve) / self.value;
        self.normalize.to_supply(supply)
    }
}

impl Constant {
    pub fn new(value: Decimal, normalize: DecimalPlaces) -> Self {
        Self { value, normalize }
    }
}

/// DecimalPlaces should be passed into curve constructors
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema, Default)]
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