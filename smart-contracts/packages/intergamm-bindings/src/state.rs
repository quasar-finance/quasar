use crate::msg::IntergammMsg;
use cosmwasm_std::{Response, StdError};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const REPLIES: Map<u64, IntergammMsg> = Map::new("intergamm-replies");
