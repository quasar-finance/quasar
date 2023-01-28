use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IbcLock describes the current state of the contract
/// Upon locking the contract, all current deposits and withdraws are going to be handled, Incoming withdraws are gathered once again gathered into a queue.
/// Once the contract unlocks, if the queue has any deposits and/or withdraws, the contract locks and starts handling all current queries
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IbcLock {
    Locked,
    Unlocked,
}
