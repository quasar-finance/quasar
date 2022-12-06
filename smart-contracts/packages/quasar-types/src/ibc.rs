use cosmwasm_std::{Binary, IbcChannel, IbcEndpoint, IbcOrder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    ica::{IcaMetadata},
};

/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcsAck {
    Result(Binary),
    Error(String),
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
