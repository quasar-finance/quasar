use crate::msg::IntergammMsg;
use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");

pub const CALLBACKADDRESS: Item<Addr> = Item::new("callback_address");