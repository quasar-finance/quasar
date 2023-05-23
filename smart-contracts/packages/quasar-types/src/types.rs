use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use cosmwasm_std::Storage;
use cw_storage_plus::{Item, Map, PrimaryKey};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

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

pub trait ItemShouldLoad<T, E> {
    fn should_load(&self, storage: &dyn Storage) -> Result<T, E>;
}

impl<'a, T> ItemShouldLoad<T, Error> for Item<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage) -> Result<T, Error> {
        self.may_load(storage)?.ok_or(Error::ItemIsEmpty {
            item: String::from_utf8(self.as_slice().to_vec())?,
        })
    }
}

pub trait MapShouldLoad<K, T, E> {
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, E>;
}

impl<'a, K, T> MapShouldLoad<K, T, Error> for Map<'a, K, T>
where
    K: PrimaryKey<'a> + Clone,
    T: Serialize + DeserializeOwned,
{
    fn should_load(&self, storage: &dyn Storage, key: K) -> Result<T, Error> {
        self.may_load(storage, key.clone())?
            .ok_or(Error::KeyNotPresentInMap {
                key: key.joined_key(),
                map: String::from_utf8(self.namespace().to_vec())?,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::mock_dependencies, Uint128};

    #[test]
    fn test_item_should_load_err() {
        let mut deps = mock_dependencies();

        const ITEM: Item<String> = Item::new("item");
        let res = ITEM.should_load(deps.as_mut().storage).unwrap_err();
        assert_eq!(
            res,
            Error::ItemIsEmpty {
                item: "item".to_string()
            }
        );
    }

    #[test]
    fn test_item_should_load_works() {
        let mut deps = mock_dependencies();

        const ITEM: Item<String> = Item::new("item");

        ITEM.save(deps.as_mut().storage, &"value".to_string())
            .unwrap();

        assert_eq!(
            ITEM.should_load(deps.as_mut().storage).unwrap(),
            "value".to_string()
        );
    }

    #[test]
    fn test_map_should_load_works() {
        let mut deps = mock_dependencies();

        const MAP: Map<&str, Uint128> = Map::new("map");

        MAP.save(deps.as_mut().storage, &0.to_string(), &Uint128::one())
            .unwrap();

        assert_eq!(
            MAP.should_load(deps.as_mut().storage, "0").unwrap(),
            Uint128::one()
        );
    }

    #[test]
    fn test_map_should_load_err() {
        let deps = mock_dependencies();
        const MAP: Map<&str, Uint128> = Map::new("map");

        let res = MAP
            .should_load(deps.as_ref().storage, "testing")
            .unwrap_err();
        assert_eq!(
            res,
            Error::KeyNotPresentInMap {
                key: "testing".into(),
                map: "map".to_string()
            }
        );
    }

    #[test]
    fn test_map_should_load_err_with_0u64() {
        let deps = mock_dependencies();
        const MAP: Map<u64, Uint128> = Map::new("map");

        let res = MAP.should_load(deps.as_ref().storage, 10).unwrap_err();
        assert_eq!(
            res,
            Error::KeyNotPresentInMap {
                key: 10u64.to_be_bytes().to_vec(),
                map: "map".to_string()
            }
        );
    }
}
