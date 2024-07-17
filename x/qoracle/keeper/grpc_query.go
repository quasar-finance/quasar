package keeper

import (
	"context"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"cosmossdk.io/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

var _ types.QueryServer = Keeper{}

func (q Keeper) Params(c context.Context, req *types.QueryParamsRequest) (*types.QueryParamsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	return &types.QueryParamsResponse{Params: q.GetParams(ctx)}, nil
}

/*
func (q Keeper) DenomMappings(c context.Context, req *types.QueryDenomMappingsRequest) (*types.QueryDenomMappingsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	var mappings []types.DenomSymbolMapping
	prefixStore := prefix.NewStore(ctx.KVStore(q.memKey), types.KeyDenomSymbolMappingPrefix)
	pageRes, err := query.Paginate(prefixStore, req.Pagination, func(key []byte, value []byte) error {
		var mapping types.DenomSymbolMapping
		err := q.cdc.Unmarshal(value, &mapping)
		if err != nil {
			return err
		}

		mappings = append(mappings, mapping)
		return nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryDenomMappingsResponse{
		Mappings:   mappings,
		Pagination: pageRes,
	}, nil
}
*/

/*
	func (q Keeper) DenomPrices(c context.Context, req *types.QueryDenomPricesRequest) (*types.QueryDenomPricesResponse, error) {
		if req == nil {
			return nil, status.Error(codes.InvalidArgument, "invalid request")
		}
		ctx := sdk.UnwrapSDKContext(c)

		var prices []sdk.DecCoin
		prefixStore := prefix.NewStore(ctx.KVStore(q.memKey), types.KeyMemDenomPricePrefix)
		pageRes, err := query.Paginate(prefixStore, req.Pagination, func(key []byte, value []byte) error {
			var price sdk.Dec
			err := price.Unmarshal(value)
			if err != nil {
				return err
			}

			prices = append(prices, sdk.NewDecCoinFromDec(string(key), price))
			return nil
		})
		if err != nil {
			return nil, status.Error(codes.Internal, err.Error())
		}

		updatedAt, err := q.GetDenomPricesUpdatedAt(ctx)
		if err != nil {
			return nil, status.Error(codes.Internal, err.Error())
		}

		return &types.QueryDenomPricesResponse{
			Prices:     prices,
			UpdatedAt:  updatedAt,
			Pagination: pageRes,
		}, nil
	}
*/

func (q Keeper) Pools(c context.Context, req *types.QueryPoolsRequest) (*types.QueryPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid nil request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	var pools []types.Pool
	prefixStore := prefix.NewStore(ctx.KVStore(q.storeKey), types.KeyMemPoolPrefix)
	pageRes, err := query.FilteredPaginate(prefixStore, req.Pagination, func(key []byte, value []byte, accumulate bool) (bool, error) {
		var pool types.Pool
		err := q.cdc.Unmarshal(value, &pool)
		if err != nil {
			return false, err
		}

		if !IsDenomInPool(pool, req.Denom) {
			return false, nil
		}

		if accumulate {
			pools = append(pools, pool)
		}
		return true, nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryPoolsResponse{
		Pools:      pools,
		Pagination: pageRes,
	}, nil
}
