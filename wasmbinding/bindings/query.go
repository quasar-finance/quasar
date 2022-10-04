package bindings

import (
	"github.com/cosmos/cosmos-sdk/types/query"
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query our position within a specific pool
	OsmosisPoolPosition *OsmosisPoolPositionRequest `json:"osmosis_pool_position,omitempty"`

	// Query a list of pool positions
	OsmosisAllPoolPositions *OsmosisAllPoolPositionsRequest `json:"osmosis_all_pool_positions,omitempty"`

	// Query the ranking of pools
	OsmosisPoolRanking *OsmosisPoolRankingRequest `json:"osmosis_pool_ranking,omitempty"`

	// Query pool info
	OsmosisPoolInfo *OsmosisPoolInfoRequest `json:"osmosis_pool_info,omitempty"`

	// Query all pool info
	OsmosisAllPoolInfo *OsmosisAllPoolInfoRequest `json:"osmosis_all_pool_info,omitempty"`

	// Query oracle prices
	OraclePrices *OraclePricesRequest `json:"oracle_prices,omitempty"`
}

type OsmosisPoolPositionRequest struct {
	PoolId string `json:"pool_id"`
}

type OsmosisAllPoolPositionsRequest struct {
	Pagination *query.PageRequest `json:"pagination,omitempty"`
}

type OsmosisPoolRankingRequest struct{}

type OsmosisPoolInfoRequest struct {
	PoolId string `json:"pool_id"`
}

type OsmosisPoolInfoResponse struct {
	PoolInfo *qoracletypes.PoolInfo `json:"pool_info,omitempty"`
}
type OsmosisAllPoolInfoRequest struct {
	Pagination *query.PageRequest `json:"pagination,omitempty"`
}

type OraclePricesRequest struct{}
