package wasmbinding

import (
	"strconv"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/cosmos/cosmos-sdk/types/query"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

type QueryPlugin struct {
	intergammKeeper *intergammkeeper.Keeper
	qoracleKeeper   *qoraclekeeper.Keeper
}

// NewQueryPlugin returns a reference to a new QueryPlugin.
func NewQueryPlugin(gk *intergammkeeper.Keeper, qk *qoraclekeeper.Keeper) *QueryPlugin {
	return &QueryPlugin{
		intergammKeeper: gk,
		qoracleKeeper:   qk,
	}
}

func (qp QueryPlugin) GetAllPools(ctx sdk.Context, pagination *query.PageRequest) ([]types.OsmosisPool, error) {
	wrappedContext := sdk.WrapSDKContext(ctx)
	pools, err := qp.qoracleKeeper.OsmosisPools(wrappedContext, &types.QueryOsmosisPoolsRequest{
		Pagination: pagination,
	})

	if err != nil {
		return nil, sdkerrors.Wrap(err, "failed to get pools")
	}

	return pools.Pools, nil
}

func (qp QueryPlugin) GetPool(ctx sdk.Context, poolID string) (*types.OsmosisPool, bool) {
	poolIdUint64, err := strconv.ParseUint(poolID, 10, 64)

	if err != nil {
		return nil, false
	}

	pool, found := qp.qoracleKeeper.GetOsmosisPool(ctx, poolIdUint64)

	if !found {
		return nil, false
	}

	return &pool, true
}

func (qp QueryPlugin) GetStablePrices(ctx sdk.Context) types.OraclePrices {
	// TODO: Can this ever error??
	return qp.qoracleKeeper.GetOraclePrices(ctx)
}
