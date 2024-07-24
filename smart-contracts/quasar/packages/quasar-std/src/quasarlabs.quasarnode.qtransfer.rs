// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(bool, tag = "1")]
    pub wasm_hooks_enabled: bool,
}
/// GenesisState defines the qtransfer module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
// @@protoc_insertion_point(module)
