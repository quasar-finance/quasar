package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) EpochLPInfo(c context.Context, req *types.QueryGetEpochLPInfoRequest) (*types.QueryGetEpochLPInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	var val types.EpochLPInfo
	/*
		ctx := sdk.UnwrapSDKContext(c)

		val, found := k.GetEpochLPInfo(ctx)
		if !found {
			return nil, status.Error(codes.InvalidArgument, "not found")
		}
	*/
	return &types.QueryGetEpochLPInfoResponse{EpochLPInfo: val}, nil
}
