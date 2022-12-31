use cosmwasm_std::{Binary, IbcChannel, IbcEndpoint, IbcOrder};
use schemars::JsonSchema;
use serde::{
    de::Error,
    de::{self, Visitor},
    Deserialize, Serialize,
};

use crate::{
    error,
    ica::handshake::{CounterPartyIcaMetadata, IcaMetadata},
    icq::ICQ_VERSION,
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
) -> Result<(), error::Error> {
    if channel.version != contract_version {
        return Err(error::Error::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }
    if let Some(version) = counterparty_version {
        if version != contract_version {
            return Err(error::Error::InvalidIbcVersion {
                version: version.to_string(),
            });
        }
    }
    if channel.order != ordering {
        return Err(error::Error::IncorrectIbcOrder {
            expected: ordering,
            got: channel.order.clone(),
        });
    }
    Ok(())
}

/// ChannelType supports different versions of a contracts ibc version. In the case of ICA, the counterparty metadata
/// needs to be handled seperately by the contract
#[derive(Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Icq,
    // TODO should we remove counter_pary_address from ICA and restrain it to the metadata? Or add an optional
    // CounterPartyIcaMetadata, removing it would allow us to simply serialize the ica metadata
    Ica { metadata: IcaMetadata },
    Ics20,
}

impl Serialize for ChannelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ChannelType::Icq {} => serializer.serialize_str(ICQ_VERSION),
            ChannelType::Ica { metadata } => metadata.serialize(serializer),
            ChannelType::Ics20 {} => serializer.serialize_str("ics20-1"),
        }
    }
}

impl<'de> Deserialize<'de> for ChannelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ChannelTypeVisitor {})
    }
}

struct ChannelTypeVisitor {}

impl<'de> Visitor<'de> for ChannelTypeVisitor {
    type Value = ChannelType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a valid ibc version, any of icq-1, ics20-1 or a valid ica metadata"
        )
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        Ok(ChannelType::Ica {
            metadata: IcaMetadata::deserialize(de::value::MapAccessDeserializer::new(map))?,
        })
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // we first check whether it's icq or ics20
        if s == ICQ_VERSION {
            Ok(ChannelType::Icq)
        } else if s == "ics20-1" {
            Ok(ChannelType::Ics20)
        } else {
            Err(de::Error::unknown_variant(s, &["icq-1", "ics20-1"]))
            // if the string we are visiting is none of the above, we have an unknown value, ica is handled as
        }
    }
}


#[cfg(test)]
mod tests {
    use osmosis_std::types::cosmos::bank::v1beta1::Metadata;
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn channel_type_icq_ser_de() {
        assert_tokens(&ChannelType::Icq, &[Token::Str("icq-1")]);
    }

    #[test]
    fn channel_type_ica_ser_de() {
        let metadata =
            IcaMetadata::with_connections("connection-0".to_string(), "connection-0".to_string());
        assert_tokens(
            &ChannelType::Ica { metadata },
            &[
                Token::Struct {
                    name: "IcaMetadata",
                    len: 5,
                },
                Token::Str("version"),
                Token::UnitVariant {
                    name: "Version",
                    variant: "ics27-1",
                },
                Token::Str("encoding"),
                Token::UnitVariant {
                    name: "Encoding",
                    variant: "proto3",
                },
                Token::Str("tx_type"),
                Token::UnitVariant {
                    name: "TxType",
                    variant: "sdk_multi_msg",
                },
                Token::Str("controller_connection_id"),
                Token::Some,
                Token::Str("connection-0"),
                Token::Str("host_connection_id"),
                Token::Some,
                Token::Str("connection-0"),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn channel_type_ics_ser_de() {
        assert_tokens(&ChannelType::Ics20, &[Token::Str("ics20-1")]);
    }
}
