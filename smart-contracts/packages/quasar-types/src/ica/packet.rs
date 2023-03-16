use std::fmt;

use cosmos_sdk_proto::{ibc::applications::interchain_accounts::v1::CosmosTx, Any};
use cosmwasm_std::{to_binary, IbcMsg, IbcTimeout};
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
        data: to_binary(&packet)?,
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
    fn quasar_osmo_formats() {
        let join = MsgJoinSwapExternAmountIn {
            sender: "osmo1mhh5z5h6ja09pv7tc6rwqtz6zsrpx3geqx0636kf5wq0lggym4zsf5ffh8".to_string(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "stake".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };

        let expected = InterchainAccountPacketData::new(Type::ExecuteTx, vec![join.pack()], None);

        let qvec: Vec<u8> = vec![
            10, 136, 1, 10, 47, 47, 111, 115, 109, 111, 115, 105, 115, 46, 103, 97, 109, 109, 46,
            118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 74, 111, 105, 110, 83, 119, 97, 112,
            69, 120, 116, 101, 114, 110, 65, 109, 111, 117, 110, 116, 73, 110, 18, 85, 10, 63, 111,
            115, 109, 111, 49, 109, 104, 104, 53, 122, 53, 104, 54, 106, 97, 48, 57, 112, 118, 55,
            116, 99, 54, 114, 119, 113, 116, 122, 54, 122, 115, 114, 112, 120, 51, 103, 101, 113,
            120, 48, 54, 51, 54, 107, 102, 53, 119, 113, 48, 108, 103, 103, 121, 109, 52, 122, 115,
            102, 53, 102, 102, 104, 56, 16, 1, 26, 13, 10, 5, 117, 111, 109, 115, 111, 18, 4, 49,
            48, 48, 48, 34, 1, 49,
        ];

        let osmovec: Vec<u8> = vec![
            10, 136, 1, 10, 47, 47, 111, 115, 109, 111, 115, 105, 115, 46, 103, 97, 109, 109, 46,
            118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 74, 111, 105, 110, 83, 119, 97, 112,
            69, 120, 116, 101, 114, 110, 65, 109, 111, 117, 110, 116, 73, 110, 18, 85, 10, 63, 111,
            115, 109, 111, 49, 109, 104, 104, 53, 122, 53, 104, 54, 106, 97, 48, 57, 112, 118, 55,
            116, 99, 54, 114, 119, 113, 116, 122, 54, 122, 115, 114, 112, 120, 51, 103, 101, 113,
            120, 48, 54, 51, 54, 107, 102, 53, 119, 113, 48, 108, 103, 103, 121, 109, 52, 122, 115,
            102, 53, 102, 102, 104, 56, 16, 1, 26, 13, 10, 5, 117, 111, 109, 115, 111, 18, 4, 49,
            48, 48, 48, 34, 1, 49,
        ];
        assert_eq!(qvec, osmovec);
        for (i, val) in expected.data.iter().enumerate() {
            assert_eq!(val, &qvec[i], "loc {i:?}")
        }
        assert_eq!(qvec, expected.data)
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
    fn deserialize_from_wire() {
        let join = MsgJoinSwapExternAmountIn {
            sender: "osmo1al8z78h82cmvt3mc9mha5fpk68njkexe6khgmk8kutvqk0sam7dsdeknyd".to_string(),
            pool_id: 1,
            token_in: Some(Coin {
                denom: "uosmo".to_string(),
                amount: "1000".to_string(),
            }),
            share_out_min_amount: "1".to_string(),
        };
        let encoded = CosmosTx {
            messages: vec![join.pack()],
        }
        .encode_to_vec();
        let msg: Vec<u8> = vec![
            10, 136, 1, 10, 47, 47, 111, 115, 109, 111, 115, 105, 115, 46, 103, 97, 109, 109, 46,
            118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 74, 111, 105, 110, 83, 119, 97, 112,
            69, 120, 116, 101, 114, 110, 65, 109, 111, 117, 110, 116, 73, 110, 18, 85, 10, 63, 111,
            115, 109, 111, 49, 97, 108, 56, 122, 55, 56, 104, 56, 50, 99, 109, 118, 116, 51, 109,
            99, 57, 109, 104, 97, 53, 102, 112, 107, 54, 56, 110, 106, 107, 101, 120, 101, 54, 107,
            104, 103, 109, 107, 56, 107, 117, 116, 118, 113, 107, 48, 115, 97, 109, 55, 100, 115,
            100, 101, 107, 110, 121, 100, 16, 1, 26, 13, 10, 5, 117, 111, 109, 115, 111, 18, 4, 49,
            48, 48, 48, 34, 1, 49,
        ];
        for (i, val) in encoded.iter().enumerate() {
            assert_eq!(val, &msg[i], "loc {i:?}")
        }
        assert_eq!(encoded, msg);
        let _join_msg: MsgJoinSwapExternAmountIn = prost::Message::decode::<&[u8]>(&msg).unwrap();
        // assert_eq!(, join_msg);
        // println!("{:?}", join_msg)
    }

    #[test]
    fn test_token_ser_de() {
        assert_tokens(&Type::Unspecified, &[Token::Str("TYPE_UNSPECIFIED")]);
        assert_tokens(&Type::ExecuteTx, &[Token::Str("TYPE_EXECUTE_TX")])
    }
}
