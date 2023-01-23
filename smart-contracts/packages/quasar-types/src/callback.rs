use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Callback {
    BondResponse(BondResponse),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
// BondResponse is the response of a the primitive once the
pub struct BondResponse {
    /// the amount of tokens that were bonded
    pub share_amount: Uint128,
    // the id of this deposit
    pub bond_id: String,
}