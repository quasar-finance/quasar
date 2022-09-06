use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Timestamp, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the quasar bindings
pub enum QuasarMsg {
    TestScenario {
        creator: String,
        scenario: String,
    },
    SendToken {
        creator: String,
        destination_local_zone_id: String,
        sender: String,
        receiver: String,
        coin: Coin,
    },
    OsmosisJoinPool {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_out_amount: i64,
        token_in_maxs: Vec<Coin>,
    },
    OsmosisExitPool {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_in_amount: i64,
        token_out_mins: Vec<Coin>,
    },
    OsmosisLockTokens {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        duration: u64,
        coins: Vec<Coin>,
    },
    OsmosisBeginUnlocking {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        id: u64,
        coins: Vec<Coin>,
    },
    OsmosisJoinSwapExternAmountIn {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_out_min_amount: i64,
        token_in: Coin,
    },
    OsmosisExitSwapExternAmountOut {
        creator: String,
        connection_id: String,
        timeout_timestamp: u64,
        pool_id: u64,
        share_in_amount: i64,
        token_out_mins: Coin,
    },
}

impl QuasarMsg {}

impl From<QuasarMsg> for CosmosMsg<QuasarMsg> {
    fn from(msg: QuasarMsg) -> CosmosMsg<QuasarMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for QuasarMsg {}
