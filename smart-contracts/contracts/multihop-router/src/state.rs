use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const ROUTES: Map<Destination, Hop> = Map::new("routes");

#[cw_serde]
pub struct Hop {
    // the channel to reach the first destination chain
    channel: String,
    // port will most likely be "transfer"
    port: String,
    // the next hop to take to reach the actual destination chain
    next: Option<Box<Hop>>,
}

impl Hop {
    /// create a packet forwarder memo field from a route of hops
    // TODO to_memo needs to know what to do with receivers of chains it's hopping on
    pub fn to_memo(self, timeout: String, retries: i64, actual_memo: Option<Binary>) -> Memo {
        Memo::new(self.to_forward(timeout, retries, actual_memo))
    }

    fn to_forward(self, timeout: String, retries: i64, actual_memo: Option<Binary>) -> Forward {
        // TODO what do we do with receiver here
        Forward {
            receiver: todo!(),
            port: self.port,
            channel: self.channel,
            timeout,
            retries,
            next: self
                .next
                .map_or(Box::new(Next::Actual(actual_memo)), |val| {
                    Box::new(Next::Forward(val.to_forward(timeout, retries, actual_memo)))
                }),
        }
    }
}

// in the case of our multihop router, a memo is a set forwarding steps with an actual memo field attached for the host chan
#[cw_serde]
pub struct Memo {
    pub forward: Forward,
}

impl Memo {
    pub fn new(forward: Forward) -> Memo {
        Memo { forward }
    }
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
    Actual(Option<Binary>),
}

// destination uses a special partialEq, so we don't derive it
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Destination(String);

impl PartialEq for Destination {
    // Destinination uses a case insensitive eq
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}
