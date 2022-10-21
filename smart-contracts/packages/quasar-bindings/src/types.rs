#![allow(non_snake_case)]

use std::time::Duration;

use cosmwasm_std::{Coin, Decimal, Timestamp, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Vec<u8>,
    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    ///
    /// Since: cosmos-sdk 0.43
    pub reverse: bool,
}

impl PageRequest {
    pub fn with_key(key: Vec<u8>) -> Self {
        Self {
            key,
            offset: 0,
            limit: 0,
            count_total: false,
            reverse: false,
        }
    }

    pub fn with_offset(offset: u64) -> Self {
        Self {
            key: vec![],
            offset,
            limit: 0,
            count_total: false,
            reverse: false,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PageResponse {
    /// next_key is the key to be passed to PageRequest.key to
    /// query the next page most efficiently
    pub next_key: Vec<u8>,
    /// total is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    pub total: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DecCoin {
    pub amount: Decimal,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolMetrics {
    pub apy: Decimal,
    pub tvl: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolAsset {
    pub token: Coin,
    pub weight: Uint256,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SmoothWeightChangeParams {
    pub start_time: Timestamp,
    pub duration: Duration,
    pub pool_asset: Vec<PoolAsset>,
    pub target_pool_weights: Vec<PoolAsset>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub smooth_weight_change_params: Option<SmoothWeightChangeParams>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisBalancerPool {
    pub address: String,
    pub id: u64,
    pub pool_params: PoolParams,
    pub future_pool_governer: Option<String>,
    pub total_shares: Coin,
    pub pool_assets: Vec<PoolAsset>,
    pub total_weight: Uint256,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPool {
    pub pool_info: OsmosisBalancerPool,
    pub metrics: OsmosisPoolMetrics,
}