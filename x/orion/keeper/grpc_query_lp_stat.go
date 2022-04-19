package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) LpStat(c context.Context, req *types.QueryGetLpStatRequest) (*types.QueryGetLpStatResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)
	val, found := k.GetLpStat(ctx, req.EpochDay)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "lp stat not found")
	}
	return &types.QueryGetLpStatResponse{LpStat: val}, nil
}
