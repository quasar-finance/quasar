use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use mars_owner::Owner;

pub const STRIDE_ORACLE: Item<Addr> = Item::new("stride_oracle");
pub const OWNER: Owner = Owner::new("owner");
