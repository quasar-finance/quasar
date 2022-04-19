package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) LpEpochPairs(goCtx context.Context, req *types.QueryLpEpochPairsRequest) (*types.QueryLpEpochPairsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	var leps []types.LpEpochPair
	leps = k.GetAllLpEpochPairList(ctx)

	return &types.QueryLpEpochPairsResponse{LpEpochPairs: leps}, nil
}
