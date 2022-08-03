use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Timestamp, Uint64};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the intergamm bindings
pub enum IntergammMsg {
    MsgSendToken {
        creator: String,
        destination_local_zone_id: String,
        sender: String,
        receiver: String,
        coin: Coin
    },
    TransmitIbcJoinPool {
        creator: String,
        connection_id: String,
        timeout_timestamp: Timestamp,
        pool_id: Uint64,
    }


}

impl IntergammMsg {

}

impl From<IntergammMsg> for CosmosMsg<IntergammMsg> {
    fn from(msg: IntergammMsg) -> CosmosMsg<IntergammMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for IntergammMsg {}