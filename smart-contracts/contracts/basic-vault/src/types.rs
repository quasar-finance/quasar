use cosmwasm_std::{Decimal, Uint128};

pub trait FromUint128 {
    fn from_uint128(value: Uint128) -> Self;
}

impl FromUint128 for Decimal {
    fn from_uint128(value: Uint128) -> Self {
        Decimal::from_ratio(value, 1u128)
    }
}
