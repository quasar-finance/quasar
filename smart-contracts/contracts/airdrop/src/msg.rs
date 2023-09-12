use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_asset::AssetInfo;

use crate::state::AirdropConfig;

#[cw_serde]
pub struct InstantiateMsg {
    /// owner is the admin address
    pub owner: Option<String>,
    /// funding address to send back funds to
    pub quasar_funding_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Admin(AdminExecuteMsg),

    ClaimAirdrop { airdrop_id: Uint128 },
}

#[cw_serde]
pub enum AdminExecuteMsg {
    AddAirdropConfig(AirdropConfig),

    UpdateAirdropConfig {
        airdrop_id: Uint128,
        airdrop_config: AirdropConfig,
    },

    AddUsers {
        airdrop_id: Uint128,
        users: Vec<Addr>,
        amounts: Vec<AssetInfo>,
    },

    AddUser {
        airdrop_id: Uint128,
        user: Addr,
        amount: AssetInfo,
    },

    RemoveUser {
        airdrop_id: Uint128,
        user: Addr,
    },

    RemoveUsers {
        airdrop_id: Uint128,
        users: Vec<Addr>,
    },
}
