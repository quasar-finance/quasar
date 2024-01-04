use cosmos_sdk_proto::tendermint::abci::{RequestQuery, ResponseQuery};
use cosmwasm_std::{Binary, IbcOrder};
use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const ICQ_VERSION: &str = "icq-1";
pub const ICQ_ORDERING: IbcOrder = IbcOrder::Unordered;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InterchainQueryPacketData {
    pub data: Vec<u8>,
}

/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct InterchainQueryPacketAck {
    pub result: Binary,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct InterchainQueryPacketAckData {
    pub data: Binary,
}

/// CosmosQuery contains a list of tendermint ABCI query requests. It should be used when sending queries to an SDK host chain.
/// Currently, CosmosQuery is not part of the cosmos proto package, thus defined manually
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosQuery {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<RequestQuery>,
}

/// CosmosResponse contains a list of tendermint ABCI query responses. It should be used when receiving responses from an SDK host chain.
/// Currently, CosmosResponse is not part of the cosmos proto package, thus defined manually
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ResponseQuery>,
}

impl InterchainQueryPacketAck {
    pub fn new(data: Binary) -> Self {
        InterchainQueryPacketAck { result: todo!() }
    }

    // pub fn data(&self) -> &Binary {
    //     &self.result.data
    // }

    // pub fn into_inner(self) -> Binary {
    //     self.result.data
    // }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    requests: Vec<RequestQuery>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            requests: Vec::new(),
        }
    }

    pub fn add_request(mut self, data: prost::bytes::Bytes, path: String) -> Self {
        self.requests.push(RequestQuery {
            data,
            path,
            height: 0,
            prove: false,
        });
        self
    }

    pub fn encode(self) -> Vec<u8> {
        CosmosQuery {
            requests: self.requests,
        }
        .encode_to_vec()
    }

    pub fn encode_pkt(self) -> InterchainQueryPacketData {
        InterchainQueryPacketData {
            data: self.encode(),
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn single_query_works() {
        let req = RequestQuery {
            data: vec![1, 0, 1, 0].into(),
            path: "/cosmos.bank.v1beta1.Query/AllBalances".into(),
            height: 0,
            prove: false,
        };

        let data = Query::new().add_request(req.data.clone(), req.path.clone());

        assert_eq!(
            data,
            Query {
                requests: vec![req.clone()]
            }
        );
        assert_eq!(
            data.encode(),
            CosmosQuery {
                requests: vec![req]
            }
            .encode_to_vec()
        )
    }

    #[test]
    pub fn multiple_query_works() {
        let req1 = RequestQuery {
            data: vec![1, 0, 1, 0].into(),
            path: "/cosmos.bank.v1beta1.Query/AllBalances".into(),
            height: 0,
            prove: false,
        };
        let req2 = RequestQuery {
            data: vec![1, 0, 0, 0].into(),
            path: "/cosmos.bank.v1beta1.Query/Balance".into(),
            height: 0,
            prove: false,
        };

        let data = Query::new()
            .add_request(req1.data.clone(), req1.path.clone())
            .add_request(req2.data.clone(), req2.path.clone());

        assert_eq!(
            data,
            Query {
                requests: vec![req1.clone(), req2.clone()]
            }
        );
        assert_eq!(
            data.encode(),
            CosmosQuery {
                requests: vec![req1, req2]
            }
            .encode_to_vec()
        )
    }
}
