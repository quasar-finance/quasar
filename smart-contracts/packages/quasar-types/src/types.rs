use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoinRatio {
    pub ratio: Vec<CoinWeight>   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoinWeight {
    pub denom: String,
    pub weight: Decimal
}

impl CoinRatio {
    pub fn get_normed_ratio(&self) -> Vec<CoinWeight> {
        let mut normed_ratio = self.ratio.clone();
        let mut total_weight = Decimal::zero();
        for coin_weight in &normed_ratio {
            total_weight += coin_weight.weight;
        }
        for coin_weight in &mut normed_ratio {
            coin_weight.weight /= total_weight;
        }
        normed_ratio
    }
}