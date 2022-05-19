use std::hash::Hash;
use cosmwasm_std::{StdError, Uint128};
use quasar_traits::{ShareDistributor};
use cw20::Cw20Coin;
use std::collections::HashSet;

pub struct Distributor{}

impl quasar_traits::traits::ShareDistributor for Distributor {
    fn distribute_shares(&self, deposit: Vec<Cw20Coin>, state: Vec<Cw20Coin>) -> Result<Uint128, StdError> {
        if deposit.len() != state.len() {
            return Err("deposit and state must be the same length".into());
        }
        if deposit.len() != 2 {
            return Err("deposit can only be two tokens".into());
        }
        if state.len() != 2 {
            return Err("state can only be two tokens".into());
        }
        if eq_token(&deposit, &state) {
            return Err("deposit and state must contain same tokens".into());
        }
        todo!()
    }
}

fn eq_token(a: &Vec<Cw20Coin>, b: &Vec<Cw20Coin>) -> bool
where
    T: Eq + Hash
{
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();

    for x in a.iter() {
        b.get(x).map(|y| x.address == y.address).unwrap_or(false)
    }
    true
}