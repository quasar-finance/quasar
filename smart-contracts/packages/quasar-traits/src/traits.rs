use cw20::{Cw20Coin};
use cosmwasm_std::{Uint128};


/// ShareDistributor is the trait describing the logic behind distributing shares within a quasar vault.
/// A share distributor does not allow for preferential treatment of certain addresses. Preferential
/// treatment has to be done at contract level.
/// deposit_funds() and withdraw_funds() should be reversible at the same state.
pub trait ShareDistributor {
    /// deposit_funds() calculates the amount of shares that should be given out in exchange for deposit
    /// amount of tokens. It does not update the state, but leaves that up to the smart contract
    fn deposit_funds(&mut self, deposit: &Vec<Cw20Coin>, state: &Vec<Cw20Coin>) -> Result<Uint128, cosmwasm_std::StdError>;
    /// withdraw_funds() calculates the amount of funds that should be returned in exchange for
    /// shares amount of shares under the current state in perfect circumstances.
    /// It does not update the state but leaves that up to the smart contract.
    fn withdraw_funds(&mut self, shares: &Uint128, state: &Vec<Cw20Coin>) -> Result<Vec<Cw20Coin>, cosmwasm_std::StdError>;
}