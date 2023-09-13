use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::state::AirdropConfig;

#[cw_serde]
pub struct InstantiateMsg {
    /// every airdrop contains a description of it
    pub airdrop_description: String,
    /// token amount to be airdropped
    pub airdrop_amount: Uint128,
    /// token denom to be airdropped
    pub airdrop_denom: Uint128,
    /// total claimed amount, zero initially
    pub total_claimed: Uint128,
    /// starting time from which users can claim airdrop
    pub start_height: u64,
    /// end time after which users cannot claim airdrop
    pub end_height: u64,
    /// flag to enable and disable claims for the given airdrop in case of any emergency
    pub claim_enabled: bool,
    /// total amount of unclaimed tokens, equal to airdrop_tokens_amount
    pub unclaimed_tokens: Uint128,
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

    /// remove a list of users from an airdrop
    RemoveUsers(Vec<String>),

    /// sends back the remaining funds to the quasar funding address
    WithdrawFunds(),
}
