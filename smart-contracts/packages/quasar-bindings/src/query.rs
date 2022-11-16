#![allow(non_snake_case)]

use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{DecCoin, OsmosisPool, PageRequest, PageResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom quer that can call into the quasar bindings
pub enum QuasarQuery {
    OsmosisPools { pagination: Option<PageRequest> },
    OsmosisPoolInfo { pool_id: String },
    OraclePrices {},
}

impl CustomQuery for QuasarQuery {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OsmosisPoolsResponse {
    pub pools: Option<Vec<OsmosisPool>>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OsmosisPoolResponse {
    pub pool: Option<OsmosisPool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct OraclePricesResponse {
    pub prices: Vec<DecCoin>,
    pub updated_at_height: i64,
}
