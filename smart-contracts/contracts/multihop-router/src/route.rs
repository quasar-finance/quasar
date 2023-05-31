use std::fmt::Display;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, StdError, StdResult};
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A Route represents the route to take to a certain chain. Each route might be unique for a certain asset.
/// A complete usable case of a route includes the asset to send to the destination, since a different asset
/// might need to take a different route
/// A complete Route is then the kv-pair of the Map Routes, namely <RouteId, Route>
#[cw_serde]
pub struct Route {
    // the channel to use in an ibc transfer from the current chain  
    pub channel: String,
    // the port to use, this is most likely always "transfer"
    pub port: String,
    // any potential hops needed to get to the current chain. These hops are dependend on the associated assets
    pub hop: Option<Hop>,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hop.is_some() {
            write!(
                f,
                "channel: {}, port: {}, (hop: {})",
                self.channel,
                self.port,
                self.hop.as_ref().unwrap()
            )
        } else {
            write!(f, "channel: {}, port: {}", self.channel, self.port)
        }
    }
}

impl Route {
    pub fn new(channel: impl Into<String>, port: impl Into<String>, hop: Option<Hop>) -> Route {
        Route {
            channel: channel.into(),
            port: port.into(),
            hop,
        }
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
            next: self
                .clone()
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
            write!(
                f,
                "channel: {}, port: {}, receiver: {}, (next: {})",
                self.channel,
                self.port,
                self.receiver,
                self.next.as_ref().unwrap()
            )
        } else {
            write!(
                f,
                "channel: {}, port: {}, receiver: {}",
                self.channel, self.port, self.receiver
            )
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

#[cw_serde]
pub struct RouteId {
    pub destination: Destination,
    pub asset: String,
}

impl Display for RouteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "destination: {}, asset: {}",
            self.destination, self.asset
        )
    }
}

impl<'a> PrimaryKey<'a> for RouteId {
    type Prefix = Destination;

    type SubPrefix = ();

    type Suffix = String;

    type SuperSuffix = (Destination, String);

    fn key(&self) -> Vec<Key> {
        let mut keys = self.destination.key();
        keys.extend(self.asset.key());
        keys
    }
}

impl KeyDeserialize for RouteId {
    type Output = RouteId;

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self::Output> {
        let mut tu = value.split_off(2);
        let t_len = parse_length(&value)?;
        let u = tu.split_off(t_len);

        Ok(RouteId {
            destination: Destination::from_vec(tu)?,
            asset: String::from_vec(u)?,
        })
    }
}

impl<'a> PrimaryKey<'a> for &RouteId {
    type Prefix = Destination;

    type SubPrefix = ();

    type Suffix = String;

    type SuperSuffix = (Destination, String);

    fn key(&self) -> Vec<Key> {
        let mut keys = self.destination.key();
        keys.extend(self.asset.key());
        keys
    }
}

impl KeyDeserialize for &RouteId {
    type Output = RouteId;

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self::Output> {
        let mut tu = value.split_off(2);
        let t_len = parse_length(&value)?;
        let u = tu.split_off(t_len);

        Ok(RouteId {
            destination: Destination::from_vec(tu)?,
            asset: String::from_vec(u)?,
        })
    }
}

fn parse_length(value: &[u8]) -> StdResult<usize> {
    Ok(u16::from_be_bytes(
        value
            .try_into()
            .map_err(|_| StdError::generic_err("Could not read 2 byte length"))?,
    )
    .into())
}

// destination uses a special partialEq, so we don't derive it
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Destination(pub String);

impl From<String> for Destination {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Destination {
    // Destinination uses a case insensitive eq
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl<'a> Prefixer<'a> for Destination {
    fn prefix(&self) -> Vec<Key> {
        self.0.prefix()
    }
}

impl<'a> PrimaryKey<'a> for Destination {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<cw_storage_plus::Key> {
        self.0.key()
    }
}

impl KeyDeserialize for Destination {
    type Output = Destination;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(Destination(
            String::from_utf8(value).map_err(StdError::invalid_utf8)?,
        ))
    }
}

impl<'a> PrimaryKey<'a> for &Destination {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<cw_storage_plus::Key> {
        self.0.key()
    }
}

impl KeyDeserialize for &Destination {
    type Output = Destination;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(Destination(
            String::from_utf8(value).map_err(StdError::invalid_utf8)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use cw_storage_plus::PrimaryKey;

    use super::*;

    prop_compose! {
        fn route_id()(dst in any::<String>(), asset in any::<String>()) -> RouteId {
            RouteId { destination: Destination(dst), asset }
        }
    }

    proptest! {
        #[test]
        fn route_id_key_ser_de(id in route_id()) {
            let keys = id.joined_key();
            let route_id = RouteId::from_vec(keys).unwrap();
            prop_assert_eq!(id, route_id)
        }
    }

    proptest! {
        #[test]
        fn route_id_borrow_key_ser_de(id in route_id()) {
            let b_id = &id;
            let keys = b_id.joined_key();
            let route_id = &RouteId::from_vec(keys).unwrap();
            prop_assert_eq!(b_id, route_id)
        }
    }
}