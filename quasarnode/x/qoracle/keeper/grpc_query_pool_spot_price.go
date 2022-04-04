package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) PoolSpotPriceAll(c context.Context, req *types.QueryAllPoolSpotPriceRequest) (*types.QueryAllPoolSpotPriceResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	var poolSpotPrices []types.PoolSpotPrice
	ctx := sdk.UnwrapSDKContext(c)

	store := ctx.KVStore(k.storeKey)
	poolSpotPriceStore := prefix.NewStore(store, types.KeyPrefix(types.PoolSpotPriceKeyPrefix))

	pageRes, err := query.Paginate(poolSpotPriceStore, req.Pagination, func(key []byte, value []byte) error {
		var poolSpotPrice types.PoolSpotPrice
		if err := k.cdc.Unmarshal(value, &poolSpotPrice); err != nil {
			return err
		}

		poolSpotPrices = append(poolSpotPrices, poolSpotPrice)
		return nil
	})

	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryAllPoolSpotPriceResponse{PoolSpotPrice: poolSpotPrices, Pagination: pageRes}, nil
}

func (k Keeper) PoolSpotPrice(c context.Context, req *types.QueryGetPoolSpotPriceRequest) (*types.QueryGetPoolSpotPriceResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	val, found := k.GetPoolSpotPrice(
		ctx,
		req.PoolId,
		req.DenomIn,
		req.DenomOut,
	)
	if !found {
		return nil, status.Error(codes.InvalidArgument, "not found")
	}

	return &types.QueryGetPoolSpotPriceResponse{PoolSpotPrice: val}, nil
}
