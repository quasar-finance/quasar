/// Minter represents the minting state.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct Minter {
    /// epoch_provisions represent rewards for the current epoch.
    #[prost(string, tag = "1")]
    pub epoch_provisions: ::prost::alloc::string::String,
}
/// WeightedAddress represents an address with a weight assigned to it.
/// The weight is used to determine the proportion of the total minted
/// tokens to be minted to the address.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct WeightedAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
/// DistributionProportions defines the distribution proportions of the minted
/// denom. In other words, defines which stakeholders will receive the minted
/// denoms and how much.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct DistributionProportions {
    /// staking defines the proportion of the minted mint_denom that is to be
    /// allocated as staking rewards.
    #[prost(string, tag = "1")]
    pub staking: ::prost::alloc::string::String,
    /// pool_incentives defines the proportion of the minted mint_denom that is
    /// to be allocated as pool incentives.
    #[prost(string, tag = "2")]
    pub pool_incentives: ::prost::alloc::string::String,
    /// developer_rewards defines the proportion of the minted mint_denom that is
    /// to be allocated to developer rewards address.
    #[prost(string, tag = "3")]
    pub developer_rewards: ::prost::alloc::string::String,
    /// community_pool defines the proportion of the minted mint_denom that is
    /// to be allocated to the community pool.
    #[prost(string, tag = "4")]
    pub community_pool: ::prost::alloc::string::String,
}
/// Params holds parameters for the x/mint module.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct Params {
    /// mint_denom is the denom of the coin to mint.
    #[prost(string, tag = "1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// genesis_epoch_provisions epoch provisions from the first epoch.
    #[prost(string, tag = "2")]
    pub genesis_epoch_provisions: ::prost::alloc::string::String,
    /// epoch_identifier mint epoch identifier e.g. (day, week).
    #[prost(string, tag = "3")]
    pub epoch_identifier: ::prost::alloc::string::String,
    /// reduction_period_in_epochs the number of epochs it takes
    /// to reduce the rewards.
    #[prost(int64, tag = "4")]
    pub reduction_period_in_epochs: i64,
    /// reduction_factor is the reduction multiplier to execute
    /// at the end of each period set by reduction_period_in_epochs.
    #[prost(string, tag = "5")]
    pub reduction_factor: ::prost::alloc::string::String,
    /// distribution_proportions defines the distribution proportions of the minted
    /// denom. In other words, defines which stakeholders will receive the minted
    /// denoms and how much.
    #[prost(message, optional, tag = "6")]
    pub distribution_proportions: ::core::option::Option<DistributionProportions>,
    /// weighted_developer_rewards_receivers is the address to receive developer
    /// rewards with weights assignedt to each address. The final amount that each
    /// address receives is: epoch_provisions *
    /// distribution_proportions.developer_rewards * Address's Weight.
    #[prost(message, repeated, tag = "7")]
    pub weighted_developer_rewards_receivers: ::prost::alloc::vec::Vec<WeightedAddress>,
    /// minting_rewards_distribution_start_epoch start epoch to distribute minting
    /// rewards
    #[prost(int64, tag = "8")]
    pub minting_rewards_distribution_start_epoch: i64,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryEpochProvisionsRequest is the request type for the
/// Query/EpochProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct QueryEpochProvisionsRequest {}
/// QueryEpochProvisionsResponse is the response type for the
/// Query/EpochProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message, ::quasar_std_derive::CosmwasmExt)]
#[proto_message(type_url = "/osmosis.mint.v1beta1.")]
pub struct QueryEpochProvisionsResponse {
    /// epoch_provisions is the current minting per epoch provisions value.
    #[prost(bytes = "vec", tag = "1")]
    pub epoch_provisions: ::prost::alloc::vec::Vec<u8>,
}
