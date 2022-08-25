use cosmwasm_std::{Coin, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendToken {
        destination_local_zone_id: String,
    },
    Deposit {},
    RegisterInterchainAccount {
        connection_id: String,
    },
    JoinSinglePool {
        connection_id: String,
        pool_id: Uint64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    TestIcaScenario {},
    AckTriggered {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AckTriggered {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AckTriggeredResponse {
    pub state: u128,
}
