use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum Token {
    Cw20 { address: Addr},
    Native{ denom: String }
}