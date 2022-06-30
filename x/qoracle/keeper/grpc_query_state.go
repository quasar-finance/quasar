package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) State(goCtx context.Context, req *types.QueryStateRequest) (*types.QueryStateResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)

	coinRatesState, err := k.GetCoinRatesState(ctx)
	if err != nil {
		return nil, err
	}
	return &types.QueryStateResponse{
		CoinRatesState: coinRatesState,
	}, nil
}
