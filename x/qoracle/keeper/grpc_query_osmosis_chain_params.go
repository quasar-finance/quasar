package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) OsmosisChainParams(goCtx context.Context, req *types.QueryOsmosisChainParamsRequest) (*types.QueryOsmosisChainParamsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryOsmosisChainParamsResponse{
		EpochsInfo:          k.GetOsmosisEpochsInfo(ctx),
		LockableDurations:   k.GetOsmosisLockableDurations(ctx),
		MintParams:          k.GetOsmosisMintParams(ctx),
		MintEpochProvisions: k.GetOsmosisMintEpochProvisions(ctx).String(), // TODO: Investigate why this throws a marshal panic when MintEpochProvisions is of type sdk.Dec
		DistrInfo:           k.GetOsmosisDistrInfo(ctx),
	}, nil
}
