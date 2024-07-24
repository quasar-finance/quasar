use cosmos_sdk_proto::tendermint::abci::{RequestQuery, ResponseQuery};

/// CosmosQuery contains a list of tendermint ABCI query requests. It should be used when sending queries to an SDK host chain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosQuery {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<RequestQuery>,
}

/// CosmosResponse contains a list of tendermint ABCI query responses. It should be used when receiving responses from an SDK host chain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ResponseQuery>,
}
