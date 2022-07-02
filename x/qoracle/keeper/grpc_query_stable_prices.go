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

func (k Keeper) StablePrices(goCtx context.Context, req *types.QueryStablePricesRequest) (*types.QueryStablePricesResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyStablePricesPrefix)

	var prices sdk.DecCoins
	pageRes, err := query.Paginate(store, req.Pagination, func(key []byte, value []byte) error {
		var price sdk.Dec
		if err := price.Unmarshal(value); err != nil {
			return err
		}

		prices = append(prices, sdk.NewDecCoinFromDec(string(key), price))
		return nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryStablePricesResponse{
		Prices:     prices,
		Pagination: pageRes,
	}, nil
}
