use cosmwasm_std::{Response, StdError};
use cw_storage_plus::{Map, Item};
use crate::msg::IntergammMsg;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status {
    Unopened,
    Init,
    Open,
    Closed,
}

pub const STATUS: Item<Status> = Item::new("intergamm-status");

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");