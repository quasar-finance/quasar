use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::state::AirdropConfig;

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
    AddUsers {
        users: Vec<String>,
        amounts: Vec<Uint128>,
    },

    /// updates the existing users with the given address and amounts
    SetUsers {
        users: Vec<String>,
        amounts: Vec<Uint128>,
    },

    /// remove a list of users from an airdrop
    RemoveUsers(Vec<String>),

    /// sends back the remaining funds to the quasar funding address
    WithdrawFunds(String),
}
