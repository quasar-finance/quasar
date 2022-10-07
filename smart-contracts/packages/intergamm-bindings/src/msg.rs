use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Uint256, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the intergamm bindings
pub enum IntergammMsg {
    SendToken {
        destination_local_zone_id: String,
        receiver: String,
        coin: Coin,
    },
    TestScenario {
        scenario: String,
    },
    RegisterIcaOnZone {
        zone_id: String,
    },
    JoinPool {
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_out_amount: i64,
        token_in_maxs: Vec<Coin>,
    },
    ExitPool {
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_in_amount: i64,
        token_out_mins: Vec<Coin>,
    },
    LockTokens {
        connection_id: String,
        timeout_timestamp: u64,
        duration: u64,
        coins: Vec<Coin>,
    },
    JoinSwapExternAmountIn {
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    ExitSwapExternAmountOut {
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_in_amount: i64,
        token_out_mins: Coin,
    },
    BeginUnlocking {
        connection_id: String,
        timeout_timestamp: u64,
        id: u64,
        coins: Vec<Coin>
    }
}

impl IntergammMsg {}

impl From<IntergammMsg> for CosmosMsg<IntergammMsg> {
    fn from(msg: IntergammMsg) -> CosmosMsg<IntergammMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for IntergammMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AckValue {
    pub error: Option<String>,
    pub response: Option<AckResponse>,
}

// AckResponse is the response message received by an intergamm ack message, see quasarnode/x/intergamm/types for the corresponding types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AckResponse {
    JoinSwapExternAmountIn {
        #[serde(rename = "shareOutAmount")]
        share_out_amount: Uint256,
    },
    ExitSwapExternAmountOut {
        #[serde(rename = "shareInAmount")]
        share_in_amount: Uint256,
    },
    JoinSwapShareAmountOut {
        #[serde(rename = "tokenInAmount")]
        token_in_amount: Uint256,
    },
    ExitSwapShareAmountIn {
        #[serde(rename = "tokenOutAmount")]
        token_out_amount: Uint256,
    },
    LockTokens {
        #[serde(rename = "ID")]
        id: Uint64,
    },
    BeginUnlocking {
        #[serde(rename = "Success")]
        succes: bool,
    },
}
