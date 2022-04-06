package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) FeeData(c context.Context, req *types.QueryGetFeeDataRequest) (*types.QueryGetFeeDataResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	// ctx := sdk.UnwrapSDKContext(c)

	// val, found := k.GetFeeData(ctx)
	//if !found {
	//	return nil, status.Error(codes.InvalidArgument, "not found")
	//}
	var val types.FeeData
	return &types.QueryGetFeeDataResponse{FeeData: val}, nil
}
