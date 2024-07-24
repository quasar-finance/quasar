// @generated
/// Gauge is an object that stores and distributes yields to recipients who
/// satisfy certain conditions. Currently gauges support conditions around the
/// duration for which a given denom is locked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// id is the unique ID of a Gauge
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// is_perpetual is a flag to show if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled, Intended for use with incentives that get refilled daily.
    #[prost(bool, tag = "2")]
    pub is_perpetual: bool,
    /// distribute_to is where the gauge rewards are distributed to.
    /// This is queried via lock duration or by timestamp
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins is the total amount of coins that have been in the gauge
    /// Can distribute multiple coin denoms
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// num_epochs_paid_over is the number of total epochs distribution will be
    /// completed over
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// filled_epochs is the number of epochs distribution has been completed on
    /// already
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    /// distributed_coins are coins that have been distributed already
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    /// List of incentivised durations that gauges will pay out to
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
// @@protoc_insertion_point(module)
