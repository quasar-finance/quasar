use std::{fmt::Display, slice::Iter};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

use crate::ContractError;

pub const RECEIVERS: Item<Receivers> = Item::new("receivers");

pub const ADMIN: Item<Addr> = Item::new("admin");

#[cw_serde]
pub struct Receivers(Vec<Receiver>);

impl Receivers {
    pub fn new(value: Vec<Receiver>) -> Result<Self, ContractError> {
        Self::try_from(value)
    }

    pub fn iter(&self) -> Iter<'_, Receiver> {
        self.0.iter()
    }
}

impl TryFrom<Vec<Receiver>> for Receivers {
    type Error = ContractError;

    fn try_from(value: Vec<Receiver>) -> Result<Self, Self::Error> {
        if value.iter().fold(Decimal::zero(), |acc, v| acc + v.share) != Decimal::one() {
            return Err(ContractError::IncorrectReceivers);
        }

        Ok(Receivers(value))
    }
}

impl Display for Receivers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cw_serde]
pub struct Receiver {
    pub address: Addr,
    pub share: Decimal,
}
