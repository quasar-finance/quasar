use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{CosmosMsg, IbcEndpoint, IbcPacket, Uint128};
use cw_storage_plus::{Item, Map};

use crate::helpers::{IbcMsgKind, MsgKind};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRequest {
    pub denom: String,
    pub amount: Uint128,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Tmp {
    pub lock_period: Uint128,
}

// TODO replace this tmp storage once usage is ironed out
// some temporary storage
pub(crate) const REPLACEME: Item<Tmp> = Item::new("tmp");

pub(crate) const REPLIES: Map<u64, MsgKind> = Map::new("replies");

pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");

// TODO is u128 too much/ insufficient?, this might cause errors on overlapping keys, could also be handled as a full queue error
pub(crate) const WITHDRAW_QUEUE: Map<u128, WithdrawRequest> = Map::new("withdraw_queue");
pub(crate) const OUTSTANDING_FUNDS: Item<Uint128> = Item::new("outstanding_funds");
pub(crate) const PENDING_ACK: Map<u64, IbcMsgKind> = Map::new("pending_acks");
