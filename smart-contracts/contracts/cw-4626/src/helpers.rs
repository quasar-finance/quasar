use cosmwasm_std::{Addr, Deps, Order, Uint128};
use crate::state::VAULT_RESERVES;

pub fn reserve(deps: Deps) -> Uint128 {
    VAULT_RESERVES.range(deps.storage, None, None, Order::Ascending).fold(Uint128::zero(), |mut total, val| {
        total + val.unwrap_or((Addr::unchecked("") ,Uint128::zero())).1
    })
}

#[cfg(test)]
mod tests {
    // TODO write some tests for helpers
}