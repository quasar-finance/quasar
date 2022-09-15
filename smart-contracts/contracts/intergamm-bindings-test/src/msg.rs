use cosmwasm_std::{Coin, Uint256, Uint64};
use intergamm_bindings::msg::IntergammMsg;
// use cosmwasm_std::{Coin, IbcTimeout, Uint64, Uint256};
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
    RegisterInterchainAccount {
        connection_id: String,
    },
    JoinSwapExternAmountIn {
        connection_id: String,
        pool_id: Uint64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    TestIcaScenario {},
    Ack {
        sequence_number: u64,
        error: Option<String>,
        response: Option<intergamm_bindings::msg::AckResponse>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PendingAcks {},
    Acks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AcksResponse {
    pub acks: Vec<(u64, intergamm_bindings::msg::AckValue)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PendingAcksResponse {
    pub pending: Vec<(u64, IntergammMsg)>,
}
