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

func (k Keeper) PoolInfoAll(c context.Context, req *types.QueryAllPoolInfoRequest) (*types.QueryAllPoolInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	var poolInfos []types.PoolInfo
	ctx := sdk.UnwrapSDKContext(c)

	store := ctx.KVStore(k.storeKey)
	poolInfoStore := prefix.NewStore(store, types.PoolInfoKBP)

	pageRes, err := query.Paginate(poolInfoStore, req.Pagination, func(key []byte, value []byte) error {
		var poolInfo types.PoolInfo
		if err := k.cdc.Unmarshal(value, &poolInfo); err != nil {
			return err
		}

		poolInfos = append(poolInfos, poolInfo)
		return nil
	})

	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryAllPoolInfoResponse{PoolInfo: poolInfos, Pagination: pageRes}, nil
}

func (k Keeper) PoolInfo(c context.Context, req *types.QueryGetPoolInfoRequest) (*types.QueryGetPoolInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	val, found := k.GetPoolInfo(
		ctx,
		req.PoolId,
	)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "not found")
	}

	return &types.QueryGetPoolInfoResponse{PoolInfo: val}, nil
}
