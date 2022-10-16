use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::{
    query::{OraclePricesResponse, OsmosisPoolInfoResponse, OsmosisPoolsResponse, QuasarQuery},
    types::PageRequest,
};

/// This is a helper wrapper to easily use our custom queries
pub struct QuasarQuerier<'a> {
    querier: &'a QuerierWrapper<'a, QuasarQuery>,
}

impl<'a> QuasarQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<QuasarQuery>) -> Self {
        QuasarQuerier { querier }
    }

    pub fn osmosis_pools(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<OsmosisPoolsResponse> {
        let query = QuasarQuery::OsmosisPools { pagination };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn osmosis_pool_info(&self, pool_id: String) -> StdResult<OsmosisPoolInfoResponse> {
        let query = QuasarQuery::OsmosisPoolInfo { pool_id };
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }

    pub fn oracle_prices(&self) -> StdResult<OraclePricesResponse> {
        let query = QuasarQuery::OraclePrices {};
        let request: QueryRequest<QuasarQuery> = QuasarQuery::into(query);
        self.querier.query(&request)
    }
}
