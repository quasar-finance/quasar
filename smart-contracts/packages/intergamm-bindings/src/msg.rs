use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Uint256, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the intergamm bindings
pub enum IntergammMsg {
    SendToken {
        creator: String,
        destination_local_zone_id: String,
        sender: String,
        receiver: String,
        coin: Coin,
    },
    TestScenario {
        creator: String,
        scenario: String,
    },
    RegisterInterchainAccount {
        creator: String,
        connection_id: String,
    },
    JoinPool {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_out_amount: i64,
        token_in_maxs: Vec<Coin>,
    },
    ExitPool {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_in_amount: i64,
        token_out_mins: Vec<Coin>,
    },
    LockTokens {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        duration: Uint64,
        coins: Vec<Coin>,
    },
    JoinSwapExternAmountIn {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    ExitSwapExternAmountOut {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_in_amount: i64,
        token_out_mins: Coin,
    },
    BeginUnlocking {
        creator: String,
        connection_id: String,
        timeout_timestamp: Uint64,
        id: Uint64,
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
