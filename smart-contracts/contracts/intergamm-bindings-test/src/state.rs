use intergamm_bindings::msg::IntergammMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::{Map,Item};

use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const ACKTRIGGERED: Item<u128> = Item::new("ack_triggered");
pub enum Status {
    Succes,
    // TODO saving the error as part of the sequence might not be desirable for the future
    Error{reason: ContractError}, 
    InProgress,
}

pub struct Ack {
    pub status: Status,
    pub msg: IntergammMsg
}

pub const ACKS: Map<Uint64, Ack> = Map::new("acks");

pub const STATE: Item<State> = Item::new("state");
