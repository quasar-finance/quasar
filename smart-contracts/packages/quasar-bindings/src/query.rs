#![allow(non_snake_case)]

use cosmwasm_std::{CustomQuery, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{
    DecCoin, OsmosisPoolInfo, PageRequest, PageResponse, PoolPosition, PoolRanking,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom quer that can call into the quasar bindings
pub enum QuasarQuery {
    OsmosisPools { pagination: Option<PageRequest> },
    OsmosisPoolInfo { pool_id: String },
    OraclePrices {},
}

impl CustomQuery for QuasarQuery {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolsResponse {
    pub pools: Vec<OsmosisPoolInfo>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OsmosisPoolInfoResponse {
    pub pool_info: Option<OsmosisPoolInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OraclePricesResponse {
    pub prices: Vec<DecCoin>,
    pub updatedAtHeight: i64,
}
