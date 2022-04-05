package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) LpStat(c context.Context, req *types.QueryGetLpStatRequest) (*types.QueryGetLpStatResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	// TODO | AUDIT
	/*
		ctx := sdk.UnwrapSDKContext(c)

		val, found := k.GetLpStat(ctx)
		if !found {
			return nil, status.Error(codes.InvalidArgument, "not found")
		}
	*/
	var val types.LpStat
	return &types.QueryGetLpStatResponse{LpStat: val}, nil
}
