use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRequest {
    pub denom: String,
    pub amount: Uint128,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct IcaState {
    pub channel: String,
    pub counter_party_address: String
}

// TODO is u128 too much/ insufficient?, this might cause errors on overlapping keys, could also be handled as a full queue error
pub(crate) const WITHDRAW_QUEUE: Map<u128, WithdrawRequest> = Map::new("withdraw_queue");
pub(crate) const OUTSTANDING_FUNDS: Item<Uint128> = Item::new("outstanding_funds");
pub(crate) const ICA_STATE: Item<IcaState> = Item::new("ica_state");
