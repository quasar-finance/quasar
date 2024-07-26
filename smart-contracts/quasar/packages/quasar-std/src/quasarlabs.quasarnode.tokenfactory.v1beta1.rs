/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct DenomAuthorityMetadata {
    /// Can be empty for no admin, or a valid osmosis address
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// Params defines the parameters for the tokenfactory module.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub denom_creation_fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// if denom_creation_fee is an empty array, then this field is used to add more gas consumption
    /// to the base cost.
    /// <https://github.com/CosmWasm/token-factory/issues/11>
    #[prost(uint64, tag = "2")]
    pub denom_creation_gas_consume: u64,
}
/// GenesisState defines the tokenfactory module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct GenesisState {
    /// params defines the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct GenesisDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomAuthorityMetadataRequest defines the request structure for the
/// DenomAuthorityMetadata gRPC query.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryDenomAuthorityMetadataRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomAuthorityMetadataResponse defines the response structure for the
/// DenomAuthorityMetadata gRPC query.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryDenomAuthorityMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryDenomsFromCreatorRequest defines the request structure for the
/// DenomsFromCreator gRPC query.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// QueryDenomsFromCreatorRequest defines the response structure for the
/// DenomsFromCreator gRPC query.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCreateDenom defines the message structure for the CreateDenom gRPC service
/// method. It allows an account to create a new denom. It requires a sender
/// address and a sub denomination. The (sender_address, sub_denomination) tuple
/// must be unique and cannot be re-used.
///
/// The resulting denom created is defined as
/// <factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is
/// originally set to be the creator, but this can be changed later. The token
/// denom does not indicate the current admin.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag = "1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint
/// more of a token.  For now, we only support minting to the sender account
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub mint_to_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgMintResponse {}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.  For now, we only support burning from the sender account.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub burn_from_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgBurnResponse {}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgChangeAdmin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgChangeAdminResponse defines the response structure for an executed
/// MsgChangeAdmin message.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgChangeAdminResponse {}
/// MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set
/// the denom's bank metadata
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgSetDenomMetadata {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<cosmos_sdk_proto::cosmos::bank::v1beta1::Metadata>,
}
/// MsgSetDenomMetadataResponse defines the response structure for an executed
/// MsgSetDenomMetadata message.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/quasarlabs.quasarnode.tokenfactory.v1beta1.")]
pub struct MsgSetDenomMetadataResponse {}
