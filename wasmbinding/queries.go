package wasmbinding

import (
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
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

func (qp QueryPlugin) GetParams(ctx sdk.Context) types.Params {
	// TODO: Can this ever error??
	return qp.qoracleKeeper.GetParams(ctx)
}

func (qp QueryPlugin) GetPoolPosition(ctx sdk.Context, poolID string) (*types.PoolPosition, error) {
	pool, found := qp.qoracleKeeper.GetPoolPosition(ctx, poolID)
	if !found {
		return nil, wasmvmtypes.InvalidResponse{Err: "failed to find pool for poolID: " + poolID}
	}

	return &pool, nil
}

func (qp QueryPlugin) GetAllPoolPosition(ctx sdk.Context) []types.PoolPosition {
	// TODO: Can this ever error??
	return qp.qoracleKeeper.GetAllPoolPosition(ctx)
}

func (qp QueryPlugin) GetPoolRanking(ctx sdk.Context) (*types.PoolRanking, error) {
	poolRanking, found := qp.qoracleKeeper.GetPoolRanking(ctx)
	if !found {
		return nil, wasmvmtypes.InvalidResponse{Err: "failed to find pool ranking"}
	}

	return &poolRanking, nil
}

func (qp QueryPlugin) GetPoolInfo(ctx sdk.Context, poolID string) (*types.PoolInfo, error) {
	pool, found := qp.qoracleKeeper.GetPoolInfo(ctx, poolID)
	if !found {
		return nil, wasmvmtypes.InvalidResponse{Err: "failed to find pool for poolID: " + poolID}
	}

	return &pool, nil
}

func (qp QueryPlugin) GetAllPoolInfo(ctx sdk.Context) []types.PoolInfo {
	// TODO: Can this ever error??
	return qp.qoracleKeeper.GetAllPoolInfo(ctx)
}

func (qp QueryPlugin) GetOraclePrices(ctx sdk.Context) types.OraclePrices {
	// TODO: Can this ever error??
	return qp.qoracleKeeper.GetOraclePrices(ctx)
}
