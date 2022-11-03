use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::IbcEndpoint;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("icq_config");

/// static info on one channel that doesn't change
pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");

pub const QUERY_RESULT_COUNTER: Item<u64> = Item::new("query_result_counter");

// pending queries lets us register a sequence number on a specific channel and set a corresponding enum.
// using the Origin enum, we can write a function for the callback on acknowledgement.
// we want to use an enum and some form of state here so we support callbacks for multiple queries batches together
// and different callbacks for the same set of queries from a different origin
pub const PENDING_QUERIES: Map<(u64, &str), Origin> = Map::new("pending_queries");

pub const REPLIES: Map<u64, Origin> = Map::new("replies");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum Origin {
    Sample,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub default_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
    /// the address of the ica on the counterparty chain
    pub address: String,
}
