use std::default;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[cw_serde]
pub struct Hop {
    // the channel to reach the first destination chain
    channel: String,
    // the next hop to take to reach the actual destination chain
    next: Box<Hop>,
}


impl Hop {
    /// create a packet forwarder memo field from a route of hops
    // TODO to_memo needs to know what to do with receivers of chains it's hopping on
    fn to_memo(timeout: String, retries: i64, channel: String) -> Memo {
        todo!()
    }
}


#[cw_serde]
pub struct Memo {
    pub forward: Forward,
}

#[cw_serde]
pub struct Forward {
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: String,
    pub retries: i64,
    pub next: Box<Next>,
}

#[cw_serde]
pub enum Next {
    Forward(Forward),
    Other(Binary),
    None
}


const ROUTES: Map<Destination, Hop> = Map::new("routes");

// destination uses a special partialEq, so we don't derive it
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Destination(String);

impl PartialEq for Destination {
    // Destinination uses a case insensitive eq
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}


