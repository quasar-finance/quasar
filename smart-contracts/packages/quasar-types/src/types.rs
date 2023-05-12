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
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Item {} is empty", item)]
    ItemIsEmpty { item: String },
    #[error("Key {} is not present in map {}", key, map)]
    KeyNotPresentInMap { key: String, map: String },
    #[error("Queue {} is empty", queue)]
    QueueIsEmpty { queue: String },
    #[error("{0}")]
    Std(#[from] StdError),
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::GenericErr {
            msg: err.to_string(),
        }
    }
}

// Define trait ItemShouldLoad
pub trait ItemShouldLoad<T, E> {
    fn should_load(&self, storage: &dyn Storage) -> Result<T, E>;
}

// Implement trait ItemShouldLoad for Item
impl<'a, T> ItemShouldLoad<T, ContractError> for Item<'_, T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    fn should_load(&self, storage: &dyn Storage) -> Result<T, ContractError> {
        let namespace_str = String::from_utf8_lossy(self.as_slice()).into();
        self.may_load(storage)?.ok_or(ContractError::ItemIsEmpty {
            item: namespace_str,
        })
    }
}

// Implement trait ItemShouldLoad for Item (static lifetime)
// impl<T> ItemShouldLoad<T, ContractError> for Item<'static, T>
// where
//     T: Serialize + DeserializeOwned + Debug,
// {
//     fn should_load(&self, storage: &dyn Storage) -> Result<T, ContractError> {
//         let namespace_str = String::from_utf8_lossy(self.as_slice()).into();
//         self.may_load(storage)?.ok_or(ContractError::ItemIsEmpty {
//             item: namespace_str,
//         })
//     }
// }

// Define trait MapShouldLoad
pub trait MapShouldLoad<K, T, E> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, E>;
}

// Implement trait MapShouldLoad for Map
impl<'a, K, T> MapShouldLoad<K, T, ContractError> for Map<'a, K, T>
where
    K: PrimaryKey<'a> + Clone + Display,
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, ContractError> {
        let namespace_str = String::from_utf8_lossy(self.namespace()).into();
        self.may_load(storage, key.clone())?
            .ok_or(ContractError::KeyNotPresentInMap {
                key: key.to_string(),
                map: namespace_str,
            })
    }
}

// Define trait QueueShouldLoad
pub trait QueueShouldLoad<T, E> {
    fn should_pop_front(&self, storage: &mut dyn Storage) -> Result<T, E>;
}

// Implement trait QueueShouldLoad for Deque
impl<'a, T> QueueShouldLoad<T, ContractError> for Deque<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_pop_front(&self, storage: &mut dyn Storage) -> Result<T, ContractError> {
        self.pop_front(storage)?.ok_or(ContractError::QueueIsEmpty {
            // TODO: this is hardcoded as I can't access the namespace of the queue
            queue: "test".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Uint128};
    use cosmwasm_std::{Addr, Storage};

    use super::*;
    const RETURNING: Map<u64, Uint128> = Map::new("returning");
    const DEPOSITOR: Item<Addr> = Item::new("depositor");

    // check if sender is the admin
    pub fn check_depositor(
        storage: &mut dyn Storage,
        sender: &Addr,
    ) -> Result<bool, ContractError> {
        let depositor = DEPOSITOR.should_load(storage)?;
        Ok(&depositor == sender)
    }

    #[test]
    fn test_item_admin_with_depositor() {
        let mut deps = mock_dependencies();
        let sender1 = Addr::unchecked("alice");
        let sender2 = Addr::unchecked("eve");

        DEPOSITOR.save(deps.as_mut().storage, &sender1).unwrap();
        assert!(check_depositor(deps.as_mut().storage, &sender1).unwrap());
        assert_eq!(check_depositor(deps.as_mut().storage, &sender1), Ok(true));
        assert_eq!(check_depositor(deps.as_mut().storage, &sender2), Ok(false));
    }

    #[test]
    fn test_item_admin_without_depositor() {
        let mut deps = mock_dependencies();
        let sender1 = Addr::unchecked("alice");

        assert_eq!(
            check_depositor(deps.as_mut().storage, &sender1).unwrap_err(),
            ContractError::ItemIsEmpty {
                item: "depositor".to_string(),
            }
        );
    }

    #[test]
    fn test_map_key_exists() {
        let mut deps = mock_dependencies();

        RETURNING
            .save(deps.as_mut().storage, 0, &Uint128::one())
            .unwrap();
        assert_eq!(
            RETURNING.should_load(deps.as_mut().storage, 0).unwrap(),
            Uint128::one()
        );
    }

    #[test]
    fn test_map_key_doesnt_exist() {
        let mut deps = mock_dependencies();
        let err = RETURNING.should_load(deps.as_mut().storage, 0).unwrap_err();
        assert_eq!(
            err,
            ContractError::KeyNotPresentInMap {
                key: 0.to_string(),
                map: "returning".to_string()
            }
        );
    }
}
