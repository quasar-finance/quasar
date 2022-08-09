package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) OsmosisIcentivizedPools(goCtx context.Context, req *types.QueryOsmosisIcentivizedPoolsRequest) (*types.QueryOsmosisIcentivizedPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryOsmosisIcentivizedPoolsResponse{
		IncentivizedPools: k.GetOsmosisIncentivizedPools(ctx),
	}, nil
}
