use cosmwasm_std::{StdResult, Uint128};

use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub lock_period: u64,
    pub pool_id: u64,
    pub pool_denom: String,
    pub local_denom: String,
    pub base_denom: String,
    pub quote_denom: String,
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
    Bond {
        id: String,
    },
    StartUnbond {
        id: String,
        share_amount: Uint128,
    },
    Unbond {
        id: String,
        share_amount: Uint128,
    },
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
