use cosmwasm_std::{Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Callback {
    BondResponse(BondResponse),
    StartUnbondResponse(StartUnbondResponse),
    UnbondResponse(UnbondResponse),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// BondResponse is the response of a the primitive once the funds are succesfully bonded
pub struct BondResponse {
    /// the amount of tokens that were bonded
    pub share_amount: Uint128,
    // the id of this deposit
    pub bond_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// UnbondResponse is the response of a primitive once shares succesfully start unbonding
pub struct StartUnbondResponse {
    pub unbond_id: String,
    pub unlock_time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UnbondResponse {
    pub unbond_id: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn callback_json() {
        let json = serde_json_wasm::to_string(&Callback::BondResponse(BondResponse { share_amount:Uint128::one(), bond_id: "my_id".to_string() })).unwrap();
        println!("{:?}", json);

    }
}
