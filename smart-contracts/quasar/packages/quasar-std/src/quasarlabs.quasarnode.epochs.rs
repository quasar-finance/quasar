#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct EpochInfo {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(int64, tag = "4")]
    pub current_epoch: i64,
    #[prost(message, optional, tag = "5")]
    pub current_epoch_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "6")]
    pub epoch_counting_started: bool,
    #[prost(int64, tag = "8")]
    pub current_epoch_start_height: i64,
}
/// GenesisState defines the epochs module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct QueryEpochsInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.epochs.")]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag = "1")]
    pub current_epoch: i64,
}
