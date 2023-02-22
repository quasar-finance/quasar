use cosmwasm_std::{Decimal, Uint128};
use std::fmt::Debug;

pub trait Curve: Debug {
    /// price returns the current price from the curve. Equal to f(x) on the curve
    /// The state of the curve should be updated afterwards by the caller
    fn price(&self, supply: &Uint128) -> Decimal;
    /// deposit() calculates the amount of shares that should be given out in exchange for deposit
    /// amount of tokens. Equal to F(x)
    /// The state of the curve should be updated afterwards by the caller
    fn deposit(&self, deposit: &Uint128) -> Uint128;
    /// withdraw() calculates the amount of funds that should be returned in exchange for
    /// shares amount of shares under the current state in perfect circumstances. equal to F^-1(x)
    /// The state of the curve should be updated afterwards by the caller
    fn withdraw(&self, shares: &Uint128) -> Uint128;
}
