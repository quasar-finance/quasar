package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) LpPosition(c context.Context, req *types.QueryGetLpPositionRequest) (*types.QueryGetLpPositionResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	// ctx := sdk.UnwrapSDKContext(c)
	var val types.LpPosition
	// TODO | AUDIT
	/*
		val, found := k.GetLpPosition(ctx)
		if !found {
			return nil, status.Error(codes.InvalidArgument, "not found")
		}
	*/
	return &types.QueryGetLpPositionResponse{LpPosition: val}, nil
}
