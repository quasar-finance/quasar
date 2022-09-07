use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::query::{OsmosisPoolPositionResponse, QuasarQuery};

/// This is a helper wrapper to easily use our custom queries
pub struct QuasarQuerier<'a> {
    querier: &'a QuerierWrapper<'a, QuasarQuery>,
}

impl<'a> QuasarQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<QuasarQuery>) -> Self {
        QuasarQuerier { querier }
    }

    pub fn osmosis_pool(&self, pool_id: String) -> StdResult<OsmosisPoolPositionResponse> {
        let query = QuasarQuery::OsmosisPoolPosition { pool_id };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    // pub fn full_denom(
    //     &self,
    //     creator_addr: String,
    //     subdenom: String,
    // ) -> StdResult<FullDenomResponse> {
    //     let full_denom_query = QuasarQuery::FullDenom {
    //         creator_addr,
    //         subdenom,
    //     };
    //     let request: QueryRequest<QuasarQuery> = QuasarQuery::into(full_denom_query);
    //     self.querier.query(&request)
    // }

    // pub fn arithmetic_twap(
    //     &self,
    //     id: u64,
    //     quote_asset_denom: String,
    //     base_asset_denom: String,
    //     start_time: i64,
    //     end_time: i64,
    // ) -> StdResult<ArithmeticTwapResponse> {
    //     let arithmetic_twap_query = QuasarQuery::ArithmeticTwap {
    //         id,
    //         quote_asset_denom,
    //         base_asset_denom,
    //         start_time,
    //         end_time,
    //     };
    //     let request: QueryRequest<QuasarQuery> = QuasarQuery::into(arithmetic_twap_query);
    //     self.querier.query(&request)
    // }

    // pub fn arithmetic_twap_to_now(
    //     &self,
    //     id: u64,
    //     quote_asset_denom: String,
    //     base_asset_denom: String,
    //     start_time: i64,
    // ) -> StdResult<ArithmeticTwapToNowResponse> {
    //     let arithmetic_twap_to_now_query = QuasarQuery::ArithmeticTwapToNow {
    //         id,
    //         quote_asset_denom,
    //         base_asset_denom,
    //         start_time,
    //     };
    //     let request: QueryRequest<QuasarQuery> = QuasarQuery::into(arithmetic_twap_to_now_query);
    //     self.querier.query(&request)
    // }
}
