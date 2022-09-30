use crate::msg::{IntergammMsg, AckValue};
use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");

pub const PENDINGACKS: Map<u64, IntergammMsg> = Map::new("pending_acks");

pub const ACKS: Map<u64, AckValue> = Map::new("acks");

pub const CALLBACKADDRESS: Item<Addr> = Item::new("callback_address");