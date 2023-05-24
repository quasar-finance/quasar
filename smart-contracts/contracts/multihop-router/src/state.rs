use std::fmt::Display;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, StdError, StdResult};
use cw_storage_plus::{KeyDeserialize, Map, PrimaryKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const ROUTES: Map<&RouteName, Route> = Map::new("routes");

#[cw_serde]
pub struct Route {
    pub channel: String,
    pub port: String,
    pub hop: Option<Hop>
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hop.is_some() {
            write!(f, "channel: {}, port: {}, (hop: {})", self.channel, self.port, self.hop.as_ref().unwrap())
        } else {
            write!(f, "channel: {}, port: {}", self.channel, self.port)
        }
    }
}

impl Route {
    pub fn new(channel: impl Into<String>, port: impl Into<String>, hop: Option<Hop>) -> Route {
        Route { channel: channel.into(), port: port.into(), hop }
    }
}

#[cw_serde]
pub struct Hop {
    // the channel to reach the first destination chain
    channel: String,
    // port will most likely be "transfer"
    port: String,
    // receiver is the receiver of the hop. If the chain has packet forward middelware properly integrated
    // the receiver is never relevant. If PFM is not properly integrated, the receiver will have the funds.
    // The users of the multihop router should ensure that the receiver works as intended
    receiver: String,
    // the next hop to take to reach the actual destination chain
    next: Option<Box<Hop>>,
}

impl Hop {
    pub fn new(
        channel: impl Into<String>,
        port: impl Into<String>,
        receiver: impl Into<String>,
        hop: Option<Hop>,
    ) -> Hop {
        Hop {
            channel: channel.into(),
            port: port.into(),
            receiver: receiver.into(),
            next: hop.map(Box::new),
        }
    }

    /// create a packet forwarder memo field from a route of hops
    /// receivers of the tokens on the intermediate chains
    pub fn to_memo(&self, timeout: String, retries: i64, actual_memo: Option<Binary>) -> Memo {
        Memo::new(self.to_forward(timeout, retries, actual_memo))
    }


    // wtf are these clones even
    fn to_forward(&self, timeout: String, retries: i64, actual_memo: Option<Binary>) -> Forward {
        Forward {
            receiver: self.receiver.clone(),
            port: self.port.clone(),
            channel: self.channel.clone(),
            timeout: timeout.clone(),
            retries,
            next: self.clone()
                .next
                .map_or(Box::new(Next::Actual(actual_memo.clone())), |val| {
                    Box::new(Next::Forward(val.to_forward(timeout, retries, actual_memo)))
                }),
        }
    }
}

impl Display for Hop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_some() {
            write!(f, "channel: {}, port: {}, receiver: {}, (next: {})", self.channel, self.port, self.receiver, self.next.as_ref().unwrap())
        } else {
            write!(f, "channel: {}, port: {}, receiver: {}", self.channel, self.port, self.receiver)
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
pub struct RouteName(pub String);

impl From<String> for RouteName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for RouteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for RouteName {
    // Destinination uses a case insensitive eq
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl<'a> PrimaryKey<'a> for RouteName {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<cw_storage_plus::Key> {
        self.0.key()
    }
}

impl KeyDeserialize for RouteName {
    type Output = RouteName;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(RouteName(
            String::from_utf8(value).map_err(StdError::invalid_utf8)?,
        ))
    }
}

impl<'a> PrimaryKey<'a> for &RouteName {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<cw_storage_plus::Key> {
        self.0.key()
    }
}

impl KeyDeserialize for &RouteName {
    type Output = RouteName;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(RouteName(
            String::from_utf8(value).map_err(StdError::invalid_utf8)?,
        ))
    }
}
