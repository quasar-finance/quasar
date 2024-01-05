use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::state::{AirdropConfig, UserInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: AirdropConfig,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// admin contains all the messages that can be executed by admin permission only
    Admin(AdminExecuteMsg),

    /// claim airdrop is for the users to execute a specific airdrop id
    ClaimAirdrop(),
}

#[cw_serde]
pub enum AdminExecuteMsg {
    /// updates airdrop config given by the admin
    UpdateAirdropConfig(AirdropConfig),

    /// add users to the airdrop with the given amounts
    AddUsers { users: Vec<User> },

    /// updates the existing users with the given address and amounts
    SetUsers { users: Vec<User> },

    /// remove a list of users from an airdrop
    RemoveUsers(Vec<String>),

    /// sends back the remaining funds to the quasar funding address
    WithdrawFunds(String),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// airdrop config shows the current config set on the contract
    #[returns(ConfigResponse)]
    AirdropConfigQuery {},

    #[returns(UserInfoResponse)]
    UserInfoQuery { user: String },

    #[returns(ContractStateResponse)]
    ContractStateQuery {},

    #[returns(SanityCheckResponse)]
    SanityCheckQuery {},

    #[returns(UsersStatsResponse)]
    UsersStatsQuery {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub airdrop_config: AirdropConfig,
}

#[cw_serde]
pub struct UserInfoResponse {
    pub user_info: UserInfo,
}

#[cw_serde]
pub struct ContractStateResponse {
    pub airdrop_config: AirdropConfig,
    pub user_info: Vec<(String, UserInfo)>,
}

#[cw_serde]
pub struct SanityCheckResponse {
    pub response: bool,
}

#[cw_serde]
pub struct UsersStatsResponse {
    pub claimed_users_count: u64,
    pub unclaimed_users_count: u64,
    pub total_users_count: u64,
}

#[cw_serde]
pub struct User {
    pub address: String,
    pub amount: Uint128,
}
