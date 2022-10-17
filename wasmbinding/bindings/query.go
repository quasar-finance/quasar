package bindings

import (
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
)

// OsmosisQuery contains osmosis custom queries.
// See https://github.com/osmosis-labs/osmosis-bindings/blob/main/packages/bindings/src/query.rs
type QuasarQuery struct {
	// Query all pools
	OsmosisPools *OsmosisPoolsRequest `json:"osmosis_pools,omitempty"`

	// Query pool info
	OsmosisPoolInfo *OsmosisPoolInfoRequest `json:"osmosis_pool_info,omitempty"`

	// Query oracle prices
	OraclePrices *OraclePricesRequest `json:"oracle_prices,omitempty"`
}

type OsmosisPoolsRequest struct {
	Pagination *query.PageRequest `json:"pagination,omitempty"`
}

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
