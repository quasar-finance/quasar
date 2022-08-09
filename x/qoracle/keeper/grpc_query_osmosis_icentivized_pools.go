package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) OsmosisIncentivizedPools(goCtx context.Context, req *types.QueryOsmosisIncentivizedPoolsRequest) (*types.QueryOsmosisIncentivizedPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryOsmosisIncentivizedPoolsResponse{
		IncentivizedPools: k.GetOsmosisIncentivizedPools(ctx),
	}, nil
}
