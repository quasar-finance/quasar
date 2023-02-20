package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
)

var _ types.QueryServer = Keeper{}

// Params implements the Query/Params gRPC method
func (q Keeper) Params(c context.Context, _ *types.QueryParamsRequest) (*types.QueryParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)
	params := q.GetParams(ctx)

	return &types.QueryParamsResponse{
		Params: params,
	}, nil
}

func (q Keeper) State(c context.Context, _ *types.QueryStateRequest) (*types.QueryStateResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)

	return &types.QueryStateResponse{
		CoinRatesState: q.GetCoinRatesState(ctx),
	}, nil
}

func (q Keeper) PriceList(c context.Context, _ *types.QueryPriceListRequest) (*types.QueryPriceListResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)
	pl := q.GetPriceList(ctx)

	return &types.QueryPriceListResponse{
		Prices:    pl.Prices,
		UpdatedAt: pl.UpdatedAt,
	}, nil
}
