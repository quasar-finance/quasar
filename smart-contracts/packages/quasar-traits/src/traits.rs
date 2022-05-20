use cw20::{Cw20Coin};
use cosmwasm_std::{Uint128};


pub trait ShareDistributor {
    fn deposit_funds(&mut self, deposit: &Vec<Cw20Coin>, state: &Vec<Cw20Coin>) -> Result<Uint128, cosmwasm_std::StdError>;
    fn withdraw_funds(&mut self, shares: &Uint128, state: &Vec<Cw20Coin>) -> Result<Vec<Cw20Coin>, cosmwasm_std::StdError>;
}