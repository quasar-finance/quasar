package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) RewardCollection(c context.Context, req *types.QueryGetRewardCollectionRequest) (*types.QueryGetRewardCollectionResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	var val types.RewardCollection
	ctx := sdk.UnwrapSDKContext(c)
	val, found := k.GetRewardCollection(ctx, req.EpochDay)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "reward collection not found")
	}
	return &types.QueryGetRewardCollectionResponse{RewardCollection: val}, nil
}
