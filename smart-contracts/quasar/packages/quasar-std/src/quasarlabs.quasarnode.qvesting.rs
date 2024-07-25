/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct Params {}
/// GenesisState defines the qvesting module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySpendableBalancesRequest defines the gRPC request structure for querying
/// an account's spendable balances.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QuerySpendableBalancesRequest {
    /// address is the address to query spendable balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySpendableBalancesResponse defines the gRPC response structure for querying
/// an account's spendable balances.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QuerySpendableBalancesResponse {
    /// balances is the spendable balances of all the coins.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryVestingAccountsRequest is the request type for the Query/Accounts RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryVestingAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVestingAccountsResponse is the response type for the Query/Accounts RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryVestingAccountsResponse {
    /// accounts are the existing vesting accounts
    /// repeated google.protobuf.Any accounts = 1 [(cosmos_proto.accepts_interface) = "VestingAccount"];
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryVestingLockedSupplyRequest is the request type for the Query/VestingLockedSupply RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryVestingLockedSupplyRequest {
    /// denom is the coin denom to query locked supply for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryVestingAccountsResponse is the response type for the Query/VestingLockedSupply RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct QueryVestingLockedSupplyResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "4")]
    pub start_time: i64,
    #[prost(int64, tag = "5")]
    pub end_time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.qvesting.")]
pub struct MsgCreateVestingAccountResponse {}
