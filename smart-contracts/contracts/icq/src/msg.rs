use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::ChannelInfo;
use cosmwasm_std::Binary;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InitMsg {
    /// Default timeout for icq packets, specified in seconds
    pub default_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MigrateMsg {
    pub default_gas_limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Query(ICQQueryMsg),
    QueryBalance {
        address: String,
        denom: String,
        channel: String,
    },
    QueryAllBalance {
        address: String,
        channel: String,
    },
    QueryMint {
        channel: String,
    },
}

/// This is the message we accept via Receive
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ICQQueryMsg {
    /// The local channel to send the packets on
    pub channel: String,
    pub requests: Vec<RequestQueryJSON>,
    // How long the packet lives in seconds. If not specified, use default_timeout
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RequestQueryJSON {
    pub data: Binary,
    pub path: String,
    pub height: i64,
    pub prove: bool,
}

// ResponseQuery does not contain all of the response fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ResponseQuery {
    pub key: Binary,
    pub value: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InterchainQueryPacketData {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return the port ID bound by this contract. Returns PortResponse
    Port {},
    /// Show all channels we have connected to. Return type is ListChannelsResponse.
    ListChannels {},
    /// Returns the details of the name channel, error if not created.
    /// Return type: ChannelResponse.
    Channel { id: String },
    /// Show the Config. Returns ConfigResponse (currently including admin as well)
    Config {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ListChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ChannelResponse {
    /// Information on the channel's connection
    pub info: ChannelInfo,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct PortResponse {
    pub port_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub default_timeout: u64,
}
