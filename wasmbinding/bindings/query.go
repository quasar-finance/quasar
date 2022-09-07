package bindings

import (
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query our position within a specific pool
	OsmosisPoolPosition *OsmosisPoolPosition `json:"osmosis_pool_position,omitempty"`

	// Query a list of pool positions
	OsmosisAllPoolPositions *qoracletypes.QueryAllPoolPositionRequest `json:"query_pool_positions_request,omitempty"`

	// Query the ranking of pools
	OsmosisPoolRanking *qoracletypes.QueryGetPoolRankingRequest `json:"query_pool_ranking_request,omitempty"`

	// Query pool info
	OsmosisPoolInfo *qoracletypes.QueryGetPoolInfoRequest `json:"query_pool_info_request,omitempty"`

	// Query all pool info
	OsmosisAllPoolInfo *qoracletypes.QueryAllPoolInfoRequest `json:"query_all_pool_info_request,omitempty"`

	// Query oracle prices
	OraclePrices *qoracletypes.QueryOraclePricesRequest `json:"query_oracle_prices_request,omitempty"`
}

type OsmosisPoolPosition struct {
	PoolId string `json:"pool_id"`
}
