package keeper

import (
	"context"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) PoolPositionAll(c context.Context, req *types.QueryAllPoolPositionRequest) (*types.QueryAllPoolPositionResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	var poolPositions []types.PoolPosition
	ctx := sdk.UnwrapSDKContext(c)

	store := ctx.KVStore(k.storeKey)
	poolPositionStore := prefix.NewStore(store, types.PoolPositionKBP)

	pageRes, err := query.Paginate(poolPositionStore, req.Pagination, func(key []byte, value []byte) error {
		var poolPosition types.PoolPosition
		if err := k.cdc.Unmarshal(value, &poolPosition); err != nil {
			return err
		}

		poolPositions = append(poolPositions, poolPosition)
		return nil
	})

	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryAllPoolPositionResponse{PoolPosition: poolPositions, Pagination: pageRes}, nil
}

func (k Keeper) PoolPosition(c context.Context, req *types.QueryGetPoolPositionRequest) (*types.QueryGetPoolPositionResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	val, found := k.GetPoolPosition(
		ctx,
		req.PoolId,
	)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "not found")
	}

	return &types.QueryGetPoolPositionResponse{PoolPosition: val}, nil
}
