use cosmwasm_std::{StdResult, Uint128};

use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub lock_period: Uint128,
    pub pool_id: u64,
    pub pool_denom: String,
    pub denom: String,
    pub local_denom: String,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Channels {}, // TODO add all wanted queries
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Deposit {},
    TransferJoinLock {
        channel: String,
        to_address: String,
    },
    DepositAndLockTokens {
        pool_id: u64,
        amount: Uint128,
        denom: String,
        share_out_min_amount: Uint128,
    },
}

// VaultResponse is the response of a the primitive back to a vault after a deposit
pub struct VaultResponse {
    pub claim_amount: Option<Uint128>,
}

// UpdateClaim is the response of a the primitive back to a vault after a claim has succesfully been converted to a share
pub struct UpdateClaim {
    pub share_amount: Uint128,
}
