package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) LpPosition(c context.Context, req *types.QueryGetLpPositionRequest) (*types.QueryGetLpPositionResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)
	var val types.LpPosition

	val, found := k.GetLpIdPosition(ctx, req.LpId)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "lp not found")
	}

	return &types.QueryGetLpPositionResponse{LpPosition: val}, nil
}

func (k Keeper) LpEpochPairs(goCtx context.Context, req *types.QueryLpEpochPairsRequest) (*types.QueryLpEpochPairsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	leps := k.GetAllLpEpochPairList(ctx)

	return &types.QueryLpEpochPairsResponse{LpEpochPairs: leps}, nil
}

func (k Keeper) LpStat(c context.Context, req *types.QueryGetLpStatRequest) (*types.QueryGetLpStatResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)
	val, found := k.GetLpStat(ctx, req.EpochDay)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "lp stat not found")
	}
	return &types.QueryGetLpStatResponse{LpStat: val}, nil
}

func (k Keeper) ListActiveLps(goCtx context.Context, req *types.QueryListActiveLpsRequest) (*types.QueryListActiveLpsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	currEpochDay := uint64(k.epochsKeeper.GetEpochInfo(ctx, k.LpEpochId(ctx)).CurrentEpoch)
	lpIds := k.GetActiveLpIDList(ctx, currEpochDay)

	return &types.QueryListActiveLpsResponse{LpIds: lpIds}, nil
}
