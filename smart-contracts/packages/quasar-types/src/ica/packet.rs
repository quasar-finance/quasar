use std::fmt;

use cosmos_sdk_proto::{ibc::applications::interchain_accounts::v1::CosmosTx, Any};
use cosmwasm_std::{to_json_binary, IbcMsg, IbcTimeout};
use prost::{bytes::Buf, Message};
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::error::Error;

use super::traits::Pack;

pub fn ica_send<T>(msg: T, channel_id: String, timeout: IbcTimeout) -> Result<IbcMsg, Error>
where
    T: Pack,
{
    let packet = InterchainAccountPacketData::new(Type::ExecuteTx, vec![msg.pack()], None);

    Ok(IbcMsg::SendPacket {
        channel_id,
        data: to_json_binary(&packet)?,
        timeout,
    })
}

// TODO implement serialize for ICA packets to serialize type as string to json
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct InterchainAccountPacketData {
    #[serde(rename = "type")]
    pub r#type: Type,
    pub data: Vec<u8>,
    pub memo: String,
}

impl InterchainAccountPacketData {
    pub fn new(r#type: Type, msgs: Vec<Any>, memo: Option<String>) -> InterchainAccountPacketData {
        InterchainAccountPacketData {
            r#type,
            data: CosmosTx { messages: msgs }.encode_to_vec(),
            memo: memo.unwrap_or_else(|| "".into()),
        }
    }
}

/// Type defines a classification of message issued from a controller chain to its associated interchain accounts
/// host
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Default zero value enumeration
    Unspecified = 0,
    /// Execute a transaction on an interchain accounts host chain
    ExecuteTx = 1,
}

impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::ExecuteTx => "TYPE_EXECUTE_TX",
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str_name())
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TypeVisitor {})
    }
}

struct TypeVisitor;

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "either TYPE_UNSPECIFIED or TYPE_EXECUTE_TX")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s == Type::Unspecified.as_str_name() {
            Ok(Type::Unspecified)
        } else if s == Type::ExecuteTx.as_str_name() {
            Ok(Type::ExecuteTx)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

#[derive(Message)]
pub struct AckBody {
    #[prost(bytes = "vec", tag = "1")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}

impl AckBody {
    pub fn to_any(self) -> Result<Any, Error> {
        let res: Any = Message::decode(self.body.as_ref())?;
        Ok(res)
    }

    pub fn from_bytes<B>(buf: B) -> Result<Self, Error>
    where
        B: Buf,
    {
        let val = Message::decode(buf)?;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ica::traits::{Pack, Unpack};
    use cosmwasm_std::{Binary, IbcAcknowledgement};
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin,
        osmosis::gamm::v1beta1::{MsgJoinSwapExternAmountIn, MsgJoinSwapExternAmountInResponse},
    };
    use serde_test::{assert_tokens, Token};

    #[test]
    fn ack_de() {
        let raw_ack = IbcAcknowledgement::new(Binary::from_base64(
        "CkcKLy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dKb2luU3dhcEV4dGVybkFtb3VudEluEhQKEjQ5MzgwNTU0OTg1NDc5OTQ5Mg==",
    ).unwrap());
        let ack = AckBody::from_bytes(raw_ack.data.as_ref())
            .unwrap()
            .to_any()
            .unwrap();
        let resp = MsgJoinSwapExternAmountInResponse::unpack(ack).unwrap();
        assert_eq!(resp.share_out_amount, "493805549854799492".to_string())
    }

    #[test]
    fn ica_packet_ser_de() {
        let join = MsgJoinSwapExternAmountIn {
            sender: "somebody".to_string(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "uosmo".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };

        let pkt =
            InterchainAccountPacketData::new(Type::ExecuteTx, vec![join.clone().pack()], None);
        assert_eq!(
            pkt.data,
            CosmosTx {
                messages: vec![join.pack()]
            }
            .encode_to_vec()
        )
    }

    #[test]
    fn deserialize_join_swap() {
        let join = MsgJoinSwapExternAmountIn {
            sender: "somebody".to_string(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "uosmo".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };
        let msg = join.encode_to_vec();
        let join_msg: MsgJoinSwapExternAmountIn = prost::Message::decode::<&[u8]>(&msg).unwrap();
        assert_eq!(join, join_msg)
    }

    #[test]
    fn test_token_ser_de() {
        assert_tokens(&Type::Unspecified, &[Token::Str("TYPE_UNSPECIFIED")]);
        assert_tokens(&Type::ExecuteTx, &[Token::Str("TYPE_EXECUTE_TX")])
    }
}
