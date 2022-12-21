use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{CosmosMsg, IbcEndpoint, IbcPacket, Uint128};
use cw_storage_plus::{Item, Map};

use crate::helpers::{IbcMsgKind, MsgKind};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub lock_period: Uint128,
    pub pool_id: u64,
    pub pool_denom: String,
    pub denom: String
}

pub(crate) const CONFIG: Item<Config> = Item::new("tmp");

pub(crate) const REPLIES: Map<u64, MsgKind> = Map::new("replies");
pub(crate) const ICA_CHANNEL: Item<String> = Item::new("ica_channel");
pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");

pub(crate) const OUTSTANDING_FUNDS: Item<Uint128> = Item::new("outstanding_funds");
pub(crate) const PENDING_ACK: Map<u64, IbcMsgKind> = Map::new("pending_acks");
