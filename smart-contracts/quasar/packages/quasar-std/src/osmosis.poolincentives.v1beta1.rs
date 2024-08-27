#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct Params {
    /// minted_denom is the denomination of the coin expected to be minted by the
    /// minting module. Pool-incentives module doesn’t actually mint the coin
    /// itself, but rather manages the distribution of coins that matches the
    /// defined minted_denom.
    #[prost(string, tag = "1")]
    pub minted_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct DistrInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct DistrRecord {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct PoolToGauge {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub gauge_id: u64,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct PoolToGauges {
    #[prost(message, repeated, tag = "2")]
    pub pool_to_gauge: ::prost::alloc::vec::Vec<PoolToGauge>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryGaugeIdsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryGaugeIdsResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_ids_with_duration:
        ::prost::alloc::vec::Vec<query_gauge_ids_response::GaugeIdWithDuration>,
}
/// Nested message and enum types in `QueryGaugeIdsResponse`.
pub mod query_gauge_ids_response {
    #[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
    pub struct GaugeIdWithDuration {
        #[prost(uint64, tag = "1")]
        pub gauge_id: u64,
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
        #[prost(string, tag = "3")]
        pub gauge_incentive_percentage: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryDistrInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryDistrInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryLockableDurationsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryIncentivizedPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct IncentivizedPool {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub lockable_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<IncentivizedPool>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryExternalIncentiveGaugesRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.poolincentives.v1beta1.")]
pub struct QueryExternalIncentiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<super::super::incentives::Gauge>,
}
