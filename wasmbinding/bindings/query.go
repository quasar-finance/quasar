package bindings

import (
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query our position within a specific pool
	QueryGetPoolPositionRequest *qoracletypes.QueryGetPoolPositionRequest `json:"query_get_pool_position_request,omitempty"`

	// Query a list of pool positions
	QueryAllPoolPositionsRequest *qoracletypes.QueryAllPoolPositionRequest `json:"query_pool_positions_request,omitempty"`

	// Query the ranking of pools
	QueryGetPoolRankingRequest *qoracletypes.QueryGetPoolRankingRequest `json:"query_pool_ranking_request,omitempty"`

	// Query pool info
	QueryGetPoolInfoRequest *qoracletypes.QueryGetPoolInfoRequest `json:"query_pool_info_request,omitempty"`

	// Query all pool info
	QueryAllPoolInfoRequest *qoracletypes.QueryAllPoolInfoRequest `json:"query_all_pool_info_request,omitempty"`

	// Query oracle prices
	QueryOraclePricesRequest *qoracletypes.QueryOraclePricesRequest `json:"query_oracle_prices_request,omitempty"`
}
