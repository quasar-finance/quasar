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
    // pool_denom is the denom of the gamm pool on osmosis; eg gamm/pool/1
    pub pool_denom: String,
    // the denom here has to be the denom of the asset on quasar,
    // after proper hops, it should be the correct denom on osmosis
    pub denom: String,
}

pub(crate) const CONFIG: Item<Config> = Item::new("tmp");

pub(crate) const REPLIES: Map<u64, MsgKind> = Map::new("replies");
// Currently we only support one ICA channel to a single destination
pub(crate) const ICA_CHANNEL: Item<String> = Item::new("ica_channel");
pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");
pub(crate) const PENDING_ACK: Map<u64, IbcMsgKind> = Map::new("pending_acks");
