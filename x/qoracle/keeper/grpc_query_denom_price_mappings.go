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

func (k Keeper) DenomPriceMappings(goCtx context.Context, req *types.QueryDenomPriceMappingsRequest) (*types.QueryDenomPriceMappingsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyDenomPriceMappingPrefix)
	var mappings []types.DenomPriceMapping
	pageRes, err := query.Paginate(store, req.Pagination, func(key []byte, value []byte) error {
		var m types.DenomPriceMapping
		if err := k.cdc.Unmarshal(value, &m); err != nil {
			return err
		}

		mappings = append(mappings, m)
		return nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryDenomPriceMappingsResponse{
		Mappings:   mappings,
		Pagination: pageRes,
	}, nil
}
