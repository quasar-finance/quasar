use cosmwasm_std::{Decimal, Uint128};
use std::fmt::Debug;

/// ShareDistributor is the trait describing the logic behind distributing shares within a quasar vault.
/// A share distributor does not allow for preferential treatment of certain addresses. Preferential
/// treatment has to be done at contract level.
/// deposit_funds() and withdraw_funds() should be reversible at the same state.
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
