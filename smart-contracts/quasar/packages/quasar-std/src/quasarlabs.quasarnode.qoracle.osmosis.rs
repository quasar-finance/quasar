#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct Params {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Identifier of the epoch that we trigger the icq request
    #[prost(string, tag = "2")]
    pub epoch_identifier: ::prost::alloc::string::String,
    /// Identifier of authorized channel that we are allowed to send/receive packets
    #[prost(string, tag = "3")]
    pub authorized_channel: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height. The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "4")]
    pub packet_timeout_height:
        ::core::option::Option<cosmos_sdk_proto::ibc::core::client::v1::Height>,
    /// Timeout timestamp relative to counterparty chain current time. The timeout is disabled when set to 0.
    #[prost(uint64, tag = "5")]
    pub packet_timeout_timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct OsmosisRequestState {
    #[prost(uint64, tag = "1")]
    pub packet_sequence: u64,
    #[prost(bool, tag = "2")]
    pub acknowledged: bool,
    #[prost(bool, tag = "3")]
    pub failed: bool,
    #[prost(int64, tag = "4")]
    pub updated_at_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct IncentivizedPools {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<
        super::super::super::super::osmosis::poolincentives::v1beta1::IncentivizedPool,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct EpochsInfo {
    #[prost(message, repeated, tag = "1")]
    pub epochs_info:
        ::prost::alloc::vec::Vec<super::super::super::super::osmosis::epochs::v1beta1::EpochInfo>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryStateResponse {
    #[prost(message, optional, tag = "2")]
    pub params_request_state: ::core::option::Option<OsmosisRequestState>,
    #[prost(message, optional, tag = "3")]
    pub incentivized_pools_state: ::core::option::Option<OsmosisRequestState>,
    #[prost(message, optional, tag = "4")]
    pub pools_state: ::core::option::Option<OsmosisRequestState>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryChainParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryChainParamsResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs_info:
        ::prost::alloc::vec::Vec<super::super::super::super::osmosis::epochs::v1beta1::EpochInfo>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub lockable_durations: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, optional, tag = "3")]
    pub mint_params:
        ::core::option::Option<super::super::super::super::osmosis::mint::v1beta1::Params>,
    #[prost(bytes = "vec", tag = "4")]
    pub mint_epoch_provisions: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub distr_info: ::core::option::Option<
        super::super::super::super::osmosis::poolincentives::v1beta1::DistrInfo,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryIncentivizedPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<
        super::super::super::super::osmosis::poolincentives::v1beta1::IncentivizedPool,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryPoolsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qoracle.osmosis.")]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<super::super::super::super::osmosis::gamm::v1beta1::Pool>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
