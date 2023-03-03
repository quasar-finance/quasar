use cosmwasm_std::{Decimal};
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
    pub fn get_normed_ratio(&self) -> Result<Vec<CoinWeight>, Error> {
        let mut normed_ratio = self.ratio.clone();
        let mut total_weight = Decimal::zero();
        if self.ratio.is_empty() {
            return Err(Error::EmptyCoinRatio);
        }
        for coin_weight in &normed_ratio {
            total_weight = total_weight.checked_add(coin_weight.weight)?;
        }
        if total_weight.is_zero() {
            total_weight = Decimal::one();
        }
        for coin_weight in &mut normed_ratio {
            coin_weight.weight = coin_weight.weight.checked_div(total_weight)?;
        }
        Ok(normed_ratio)
    }
}
