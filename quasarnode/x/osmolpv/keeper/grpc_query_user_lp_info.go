package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) UserLPInfo(c context.Context, req *types.QueryGetUserLPInfoRequest) (*types.QueryGetUserLPInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	var val types.UserLPInfo
	/*
		ctx := sdk.UnwrapSDKContext(c)

		val, found := k.GetUserLPInfo(ctx)
		if !found {
			return nil, status.Error(codes.InvalidArgument, "not found")
		}
	*/
	return &types.QueryGetUserLPInfoResponse{UserLPInfo: val}, nil
}
