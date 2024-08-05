use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;

pub const OWNER: Owner = Owner::new("owner");
pub const LSTS: Map<String, Addr> = Map::new("lsts");
pub const VAULT_DENOM: Item<String> = Item::new("denom");
