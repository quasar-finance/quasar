use crate::msg::{AckValue, IntergammMsg};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");

pub const PENDINGACKS: Map<u64, IntergammMsg> = Map::new("pending_acks");

pub const ACKS: Map<u64, AckValue> = Map::new("acks");

pub const CALLBACKADDRESS: Item<Addr> = Item::new("callback_address");
