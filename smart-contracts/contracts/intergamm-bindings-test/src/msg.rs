use cosmwasm_std::{Coin, Uint64};
use intergamm_bindings::msg::IntergammMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub callback_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendToken {
        destination_local_zone_id: String,
        receiver: String,
        coin: Coin,
    },
    SendTokenIbc {
        /// exisiting channel to send the tokens over
        channel_id: String,
        /// address on the remote chain to receive these tokens
        to_address: String,
        /// packet data only supports one coin
        /// https://github.com/cosmos/cosmos-sdk/blob/v0.40.0/proto/ibc/applications/transfer/v1/transfer.proto#L11-L20
        amount: Coin,
        // when packet times out, measured on remote chain, for now we hardcode the timeout
        // timeout: IbcTimeout,
    },
    Deposit {},
    RegisterIcaOnZone {
        zone_id: String,
    },
    JoinPool {
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_out_amount: i64,
        token_in_maxs: Vec<Coin>,
    },
    ExitPool {
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_in_amount: i64,
        token_out_mins: Vec<Coin>,
    },
    LockTokens {
        connection_id: String,
        timeout_timestamp: Uint64,
        duration: Uint64,
        coins: Vec<Coin>,
    },
    JoinSwapExternAmountIn {
        connection_id: String,
        pool_id: Uint64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    ExitSwapExternAmountOut {
        connection_id: String,
        timeout_timestamp: Uint64,
        pool_id: Uint64,
        share_in_amount: i64,
        token_out_mins: Coin,
    },
    BeginUnlocking {
        connection_id: String,
        timeout_timestamp: Uint64,
        id: Uint64,
        coins: Vec<Coin>,
    },
    TestIcaScenario {},
    Ack {
        sequence_number: u64,
        error: Option<String>,
        response: Option<intergamm_bindings::msg::AckResponse>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PendingAcks {},
    Acks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AcksResponse {
    pub acks: Vec<(u64, intergamm_bindings::msg::AckValue)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PendingAcksResponse {
    pub pending: Vec<(u64, IntergammMsg)>,
}
