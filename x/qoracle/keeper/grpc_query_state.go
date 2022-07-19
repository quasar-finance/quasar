package keeper

import (
	"context"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) State(goCtx context.Context, req *types.QueryStateRequest) (*types.QueryStateResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryStateResponse{
		CoinRatesState:            k.GetCoinRatesState(ctx),
		OsmosisParamsRequestState: k.GetOsmosisParamsRequestState(ctx),
	}, nil
}
