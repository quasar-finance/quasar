use cw20::{Cw20Coin};
use cosmwasm_std::{Uint128};


pub trait ShareDistributor {
    fn distribute_shares(&self, deposit: Vec<Cw20Coin>, state: Vec<Cw20Coin>) -> Result<Uint128, cosmwasm_std::StdError>;
}