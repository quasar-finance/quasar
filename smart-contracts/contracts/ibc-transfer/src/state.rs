use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


use cw_storage_plus::{Map};

use crate::helpers::{IbcMsgKind, MsgKind};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum Origin {
    Sample,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Config {
    pub default_timeout: u64,
}

pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");

pub(crate) const REPLIES: Map<u64, MsgKind> = Map::new("replies");

pub(crate) const PENDING_ACK: Map<u64, IbcMsgKind> = Map::new("pending_acks");

