use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_asset::AssetInfo;

use crate::state::AirdropConfig;

#[cw_serde]
pub struct InstantiateMsg {
    /// funding address to send back funds to
    pub funding_or_refund_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// admin contains all the messages that can be executed by admin permission only
    Admin(AdminExecuteMsg),

    /// claim airdrop is for the users to execute a specific airdrop id
    ClaimAirdrop(Uint128),
}

#[cw_serde]
pub enum AdminExecuteMsg {
    /// adds a new airdrop config given by the admin
    AddAirdropConfig(AirdropConfig),

    /// updates airdrop config given by the admin
    UpdateAirdropConfig {
        airdrop_id: Uint128,
        airdrop_config: AirdropConfig,
    },

    /// add users to the airdrop with the given amounts
    AddUsers {
        airdrop_id: Uint128,
        users: Vec<Addr>,
        amounts: Vec<Uint128>,
    },

    /// add single user to the airdrop with the given amount
    AddUser {
        airdrop_id: Uint128,
        user: Addr,
        amount: Uint128,
    },

    /// remove a list of users from an airdrop
    RemoveUsers {
        airdrop_id: Uint128,
        users: Vec<Addr>,
    },

    /// remove a user from an airdrop
    RemoveUser { airdrop_id: Uint128, user: Addr },

    /// sends back the remaining funds to the quasar funding address
    WithdrawFunds { airdrop_id: Uint128 },
}
