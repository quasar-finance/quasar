use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::{query::{OsmosisPoolPositionResponse, QuasarQuery, OsmosisAllPoolPositionsResponse, OsmosisPoolRankingResponse, OsmosisPoolInfoResponse, OsmosisAllPoolInfoResponse, OraclePricesResponse}, types::PageRequest};

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

    pub fn all_osmosis_pools(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<OsmosisAllPoolPositionsResponse> {
        let query = QuasarQuery::OsmosisAllPoolPositions { pagination };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn osmosis_pool_ranking(&self) -> StdResult<OsmosisPoolRankingResponse> {
        let query = QuasarQuery::OsmosisPoolRanking {};
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn osmosis_pool_info(&self, pool_id: String) -> StdResult<OsmosisPoolInfoResponse> {
        let query = QuasarQuery::OsmosisPoolInfo { pool_id };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn all_osmosis_pool_info(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<OsmosisAllPoolInfoResponse> {
        let query = QuasarQuery::OsmosisAllPoolInfo { pagination };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn oracle_prices(&self) -> StdResult<OraclePricesResponse> {
        let query = QuasarQuery::OraclePrices {};
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }
}
