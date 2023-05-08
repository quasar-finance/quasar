use cosmwasm_std::{Decimal, Uint128};
use std::fmt::Debug;

/// ShareDistributor is the trait describing the logic behind distributing shares within a quasar vault.
/// A share distributor does not allow for preferential treatment of certain addresses. Preferential
/// treatment has to be done at contract level.
/// deposit_funds() and withdraw_funds() should be reversible at the same state.
pub trait Curve: Debug {
    /// price returns the current price from the curve. Equal to f(x) on the curve
    /// The state of the curve should be updated afterwards by the caller
    fn price(&self, supply: &Uint128) -> Decimal;
    /// deposit() calculates the amount of shares that should be given out in exchange for deposit
    /// amount of tokens. Equal to F(x)
    /// The state of the curve should be updated afterwards by the caller
    fn deposit(&self, deposit: &Uint128) -> Uint128;
    /// withdraw() calculates the amount of funds that should be returned in exchange for
    /// shares amount of shares under the current state in perfect circumstances. equal to F^-1(x)
    /// The state of the curve should be updated afterwards by the caller
    fn withdraw(&self, shares: &Uint128) -> Uint128;
}

use cosmwasm_std::{StdError, Storage};
use cw_storage_plus::{Deque, Item, Map, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Key not present in Item")]
    KeyNotPresentInItem {},
    #[error("Key not present in Map")]
    KeyNotPresentInMap {},
    #[error("Key not present in Deque")]
    KeyNotPresentInDeque {},
    #[error(transparent)]
    StdError(#[from] StdError),
}

// Define trait ItemShouldLoad
pub trait ItemShouldLoad<T> {
    fn should_load(&self, storage: &dyn Storage) -> Result<T, Error>;
}

// Implement trait ItemShouldLoad for Map
impl<'a, T> ItemShouldLoad<T> for Item<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage) -> Result<T, Error> {
        self.may_load(storage)?.ok_or(Error::KeyNotPresentInItem {})
    }
}

// Define trait MapShouldLoad
trait MapShouldLoad<K, T> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, Error>;
}

// Implement trait MapShouldLoad for Map
impl<'a, K, T> MapShouldLoad<K, T> for Map<'a, K, T>
where
    K: PrimaryKey<'a> + Clone,
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, Error> {
        self.may_load(storage, key)?
            .ok_or(Error::KeyNotPresentInMap {})
    }
}

// Define trait QueueShouldLoad
trait QueueShouldLoad<K, T> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, Error>;
}

// Implement trait QueueShouldLoad for Deque
impl<'a, T> QueueShouldLoad<u32, T> for Deque<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: u32) -> Result<T, Error> {
        self.get(storage, key)?
            .ok_or(Error::KeyNotPresentInDeque {})
    }
}
