use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoinRatio {
    pub ratio: Vec<CoinWeight>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoinWeight {
    pub denom: String,
    pub weight: Decimal,
}

impl CoinRatio {
    // pub fn get_normed_ratio(&self) -> Result<Vec<CoinWeight>, Error> {
    //     let mut normed_ratio = self.ratio.clone();
    //     let mut total_weight = Decimal::zero();
    //     if self.ratio.is_empty() {
    //         return Err(Error::EmptyCoinRatio);
    //     }
    //     for coin_weight in &normed_ratio {
    //         total_weight = total_weight.checked_add(coin_weight.weight)?;
    //     }
    //     if total_weight.is_zero() {
    //         total_weight = Decimal::one();
    //     }
    //     for coin_weight in &mut normed_ratio {
    //         coin_weight.weight = coin_weight.weight.checked_div(total_weight)?;
    //     }
    //     Ok(normed_ratio)
    // }

    pub fn normalize(&mut self) -> Result<Vec<CoinWeight>, Error> {
        let mut total_weight = Decimal::zero();
        if self.ratio.is_empty() {
            return Err(Error::EmptyCoinRatio);
        }
        for coin_weight in self.ratio.iter() {
            total_weight = total_weight.checked_add(coin_weight.weight)?;
        }
        if total_weight.is_zero() {
            total_weight = Decimal::one();
        }
        for mut coin_weight in self.ratio.iter_mut() {
            coin_weight.weight = coin_weight.weight.checked_div(total_weight)?;
        }
        Ok(self.ratio.clone())
    }
}

use cosmwasm_std::{StdError, Storage};
use cw_storage_plus::{Deque, Item, Map, PrimaryKey};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
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
    fn should_load(&self, storage: &dyn Storage) -> Result<T, ContractError>;
}

// Implement trait ItemShouldLoad for Map
impl<'a, T> ItemShouldLoad<T> for Item<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage) -> Result<T, ContractError> {
        self.may_load(storage)?
            .ok_or(ContractError::KeyNotPresentInItem {})
    }
}

// Define trait MapShouldLoad
trait MapShouldLoad<K, T> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, ContractError>;
}

// Implement trait MapShouldLoad for Map
impl<'a, K, T> MapShouldLoad<K, T> for Map<'a, K, T>
where
    K: PrimaryKey<'a> + Clone,
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, ContractError> {
        self.may_load(storage, key)?
            .ok_or(ContractError::KeyNotPresentInMap {})
    }
}

// Define trait QueueShouldLoad
trait QueueShouldLoad<K, T> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, ContractError>;
}

// Implement trait QueueShouldLoad for Deque
impl<'a, T> QueueShouldLoad<u32, T> for Deque<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: u32) -> Result<T, ContractError> {
        self.get(storage, key)?
            .ok_or(ContractError::KeyNotPresentInDeque {})
    }
}
