use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{CosmosMsg, IbcPacket, Uint128, IbcEndpoint};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRequest {
    pub denom: String,
    pub amount: Uint128,
    pub owner: String,
}

// TODO we can probably make the use if IbcEndpoints a bit saver
pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");

// TODO is u128 too much/ insufficient?, this might cause errors on overlapping keys, could also be handled as a full queue error
pub(crate) const WITHDRAW_QUEUE: Map<u128, WithdrawRequest> = Map::new("withdraw_queue");
pub(crate) const OUTSTANDING_FUNDS: Item<Uint128> = Item::new("outstanding_funds");
pub(crate) const REPLIES: Map<u64, IbcPacket> = Map::new("replies");
pub(crate) const PENDING_ACK: Map<u64, IbcPacket> = Map::new("pending_acks");
