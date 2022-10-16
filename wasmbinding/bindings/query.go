package bindings

import (
	"github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query the ranking of pools
	OsmosisRankedPools *OsmosisRankedPoolsRequest `json:"osmosis_pool_ranking,omitempty"`

	// Query all pools
	OsmosisPools *OsmosisPoolsRequest `json:"osmosis_pools,omitempty"`

	// Query pool info
	OsmosisPoolInfo *OsmosisPoolInfoRequest `json:"osmosis_pool_info,omitempty"`

	// Query oracle prices
	OraclePrices *OraclePricesRequest `json:"oracle_prices,omitempty"`
}

type OsmosisRankedPoolsRequest struct{}

type OsmosisPoolsRequest struct{}

// type OsmosisPoolsResponse struct {
// 	Pools []types.OsmosisPool `json:"pools"`
// }

type OsmosisPoolInfoRequest struct {
	PoolId string `json:"pool_id"`
}

type OsmosisPoolInfoResponse struct {
	PoolInfo *balancer.Pool `json:"pool_info,omitempty"`
}

type OraclePricesRequest struct{}
