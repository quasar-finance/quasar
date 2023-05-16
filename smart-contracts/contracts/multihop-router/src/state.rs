use std::default;

use cosmwasm_std::Binary;
use cw_storage_plus::Map;
use serde::{Serialize, Deserialize};


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


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memo {
    pub forward: Forward,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forward {
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: String,
    pub retries: i64,
    pub next: Box<Next>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Next {
    Forward(Forward),
    Other(Binary),
    #[default]
    None
}


const ROUTES: Map<Destination, Hop> = Map::new("routes");


pub struct Destination(String);

impl PartialEq for Destination {
    // Destinination uses a case insensitive eq
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}


