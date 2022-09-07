use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{PageRequest, PoolPosition};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
/// A number of Custom quer that can call into the quasar bindings
pub enum QuasarQuery {
    OsmosisPoolPosition { pool_id: String },
    OsmosisAllPoolPositions { pagination: PageRequest },
    OsmosisPoolRanking {},
    OsmosisPoolInfo { pool_id: String },
    OsmosisAllPoolInfo { pagination: PageRequest },
    OraclePrices {},
}

impl CustomQuery for QuasarQuery {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OsmosisPoolPositionResponse {
    pub pool_position: PoolPosition,
}
