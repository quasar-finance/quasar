use cosmwasm_std::{Binary, IbcChannel, IbcEndpoint, IbcOrder};

use osmosis_std_derive::CosmwasmExt;

use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::Error, ica::handshake::IcaMetadata};

/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcsAck {
    Result(Binary),
    Error(String),
}

/// MsgTransfer is the ibc 4.2 transfer point, to be used with ICA to send funds back to quasar chain
/// message MsgTransfer {
///     option (gogoproto.equal)           = false;
///     option (gogoproto.goproto_getters) = false;
///     // the port on which the packet will be sent
///     string source_port = 1 [(gogoproto.moretags) = "yaml:\"source_port\""];
///     // the channel by which the packet will be sent
///     string source_channel = 2 [(gogoproto.moretags) = "yaml:\"source_channel\""];
///     // the tokens to be transferred
///     cosmos.base.v1beta1.Coin token = 3 [(gogoproto.nullable) = false];
///     // the sender address
///     string sender = 4;
///     // the recipient address on the destination chain
///     string receiver = 5;
///     // Timeout height relative to the current block height.
///     // The timeout is disabled when set to 0.
///     ibc.core.client.v1.Height timeout_height = 6
///         [(gogoproto.moretags) = "yaml:\"timeout_height\"", (gogoproto.nullable) = false];
///     // Timeout timestamp in absolute nanoseconds since unix epoch.
///     // The timeout is disabled when set to 0.
///     uint64 timeout_timestamp = 7 [(gogoproto.moretags) = "yaml:\"timeout_timestamp\""];
///     // optional memo
///     string memo = 8;
///   }
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Message, CosmwasmExt)]
#[serde(rename_all = "snake_case")]
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransfer")]
pub struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: String,
    #[prost(string, tag = "2")]
    pub source_channel: String,
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<osmosis_std::types::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub sender: String,
    #[prost(string, tag = "5")]
    pub receiver: String,
    #[prost(message, optional, tag = "6")]
    pub timeout_height: Option<Height>,
    #[prost(uint64, optional, tag = "7")]
    pub timeout_timestamp: ::core::option::Option<u64>,
    #[prost(string, tag = "8")]
    pub memo: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Message)]
#[serde(rename_all = "snake_case")]
pub struct Height {
    #[prost(uint64, optional, tag = "1")]
    pub revision_number: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub revision_height: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Message)]
#[serde(rename_all = "snake_case")]
pub struct MsgTransferResponse {
    #[prost(uint64, tag = "1")]
    pub seq: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Icq {
        channel_ty: String,
    },
    Ica {
        channel_ty: IcaMetadata,
        counter_party_address: Option<String>,
    },
    Ics20 {
        channel_ty: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
    /// the channel type,
    pub channel_type: ChannelType,
    /// the channel handshake state
    pub handshake_state: HandshakeState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandshakeState {
    Init,
    TryOpen,
    Open,
    Closed,
}

pub fn enforce_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
    contract_version: &str,
    ordering: IbcOrder,
) -> Result<(), Error> {
    if channel.version != contract_version {
        return Err(Error::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }
    if let Some(version) = counterparty_version {
        if version != contract_version {
            return Err(Error::InvalidIbcVersion {
                version: version.to_string(),
            });
        }
    }
    if channel.order != ordering {
        return Err(Error::IncorrectIbcOrder {
            expected: ordering,
            got: channel.order.clone(),
        });
    }
    Ok(())
}
