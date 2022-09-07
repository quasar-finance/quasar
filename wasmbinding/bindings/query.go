package bindings

import (
	"github.com/cosmos/cosmos-sdk/types/query"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query our position within a specific pool
	OsmosisPoolPosition *OsmosisPoolPosition `json:"osmosis_pool_position,omitempty"`

	// Query a list of pool positions
	OsmosisAllPoolPositions *OsmosisAllPoolPositions `json:"osmosis_all_pool_positions,omitempty"`

	// Query the ranking of pools
	OsmosisPoolRanking *OsmosisPoolRanking `json:"osmosis_pool_ranking,omitempty"`

	// Query pool info
	OsmosisPoolInfo *OsmosisPoolInfo `json:"osmosis_pool_info,omitempty"`

	// Query all pool info
	OsmosisAllPoolInfo *OsmosisAllPoolInfo `json:"osmosis_all_pool_info,omitempty"`

	// Query oracle prices
	OraclePrices *OraclePrices `json:"oracle_prices,omitempty"`
}

type OsmosisPoolPosition struct {
	PoolId string `json:"pool_id"`
}

type OsmosisAllPoolPositions struct {
	Pagination *query.PageRequest `json:"pagination,omitempty"`
}

type OsmosisPoolRanking struct{}

type OsmosisPoolInfo struct {
	PoolId string `json:"pool_id"`
}

type OsmosisAllPoolInfo struct {
	Pagination *query.PageRequest `json:"pagination,omitempty"`
}

type OraclePrices struct{}
