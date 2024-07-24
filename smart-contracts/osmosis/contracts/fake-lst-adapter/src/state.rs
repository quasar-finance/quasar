use cosmwasm_std::Decimal;
use cw_storage_plus::Item;

pub const STATE: Item<Decimal> = Item::new("state");
