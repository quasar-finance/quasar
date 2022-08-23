use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Timestamp, Uint64};
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
    MsgTransmitIbcJoinPool {
        creator: String,
        scenario: String,
    },
    RegisterInterchainAccount {
        creator: String,
        connection_id: String,
    },
    JoinSwapExternAmountIn {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
}

impl IntergammMsg {}

impl From<IntergammMsg> for CosmosMsg<IntergammMsg> {
    fn from(msg: IntergammMsg) -> CosmosMsg<IntergammMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for IntergammMsg {}
