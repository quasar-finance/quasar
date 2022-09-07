use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, serde::Serialize, Deserialize, JsonSchema, Debug)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct GaugeAPY {
   pub gaugeId: u64,
   pub duration: String,
   pub aPY: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolMetrics {
    pub highestAPY: String,
    pub tVL: String,
    pub gaugeAPYs: Vec<GaugeAPY>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolPosition {
   pub poolId: String,
   pub metrics: PoolMetrics,
   pub lastUpdatedTime: u64,
   pub creator: String,
}
