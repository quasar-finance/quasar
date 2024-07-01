use cw_storage_plus::Item;
use mars_owner::Owner;

pub const LST_DENOM: Item<String> = Item::new("lst_denom");
pub const VAULT: Owner = Owner::new("owner");
