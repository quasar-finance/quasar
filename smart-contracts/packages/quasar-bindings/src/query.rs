#![allow(non_snake_case)]

use cosmwasm_std::{CustomQuery, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{DecCoin, PageRequest, PoolInfo, PoolPosition, PoolRanking};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom quer that can call into the quasar bindings
pub enum QuasarQuery {
    OsmosisPoolPosition { pool_id: String },
    OsmosisAllPoolPositions { pagination: Option<PageRequest> },
    OsmosisPoolRanking {},
    OsmosisPoolInfo { pool_id: String },
    OsmosisAllPoolInfo { pagination: Option<PageRequest> },
    OraclePrices {},
}

impl CustomQuery for QuasarQuery {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolPositionResponse {
    pub poolPosition: PoolPosition,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisAllPoolPositionsResponse {
    pub poolPositions: Vec<PoolPosition>,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolRankingResponse {
    pub poolRanking: PoolRanking,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolInfoResponse {
    pub poolInfo: PoolInfo,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisAllPoolInfoResponse {
    pub poolInfo: Vec<PoolInfo>,
    pub pagination: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OraclePricesResponse {
    pub prices: Vec<DecCoin>,
    pub updatedAtHeight: i64,
}
