// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// GenesisState defines the qoracle module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag = "4")]
    pub osmosis_genesis_state: ::core::option::Option<OsmosisGenesisState>,
}
/// OsmosisGenesisState defines the qoracle osmosis submodule's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsmosisGenesisState {
    #[prost(string, tag = "1")]
    pub port: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<osmosis::Params>,
}
/// Pool defines the generalized structure of a liquidity pool coming from any source chain to qoracle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    /// The identifier of this pool in the source chain
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// List of assets with their current volume in pool
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Total volume locked in the pool
    #[prost(bytes = "vec", tag = "3")]
    pub tvl: ::prost::alloc::vec::Vec<u8>,
    /// Annual percentage yield of the pool
    #[prost(bytes = "vec", tag = "4")]
    pub apy: ::prost::alloc::vec::Vec<u8>,
    /// Raw data of pool structure stored in the source chain
    #[prost(message, optional, tag = "5")]
    pub raw: ::core::option::Option<::prost_types::Any>,
    /// Last time this pool was updated
    #[prost(message, optional, tag = "6")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryPoolsRequest is request type for the Query/Pools RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    /// denom filters the pools by their denom. If empty, pools with any denom returned.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPoolsResponse is response type for the Query/Pools RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
// @@protoc_insertion_point(module)
