use cosmwasm_std::{StdResult, Uint128};
use cw20::Logo;
use cw_utils::Duration;
use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    // TODO write the instantiate msg
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
    // We can always deposit money into the strategy
    Deposit {
        owner: String,
    },
    // A request for a withdraw, this has to be a request and cannot be an immediate withdraw since funds might be locked
    Transfer {
        channel: String,
        to_address: String,
    },
    DepositAndLockTokens {
        channel: String,
        pool_id: u64,
        amount: Uint128,
        denom: String,
        share_out_min_amount: Uint128,
        lock_period: Uint128,
    },
}
