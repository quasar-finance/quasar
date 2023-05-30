package keeper

import (
	"context"
	"github.com/cosmos/cosmos-sdk/store/mem"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// SpendableBalances implements a gRPC query handler for retrieving an account's
// spendable balances.
func (k Keeper) SpendableBalances(ctx context.Context, req *types.QuerySpendableBalancesRequest) (*types.QuerySpendableBalancesResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	addr, err := sdk.AccAddressFromBech32(req.Address)
	if err != nil {
		return nil, status.Errorf(codes.InvalidArgument, "invalid address: %s", err.Error())
	}

	sdkCtx := sdk.UnwrapSDKContext(ctx)

	spendable := k.bankKeeper.SpendableCoins(sdkCtx, addr)

	memStore := mem.NewStore()
	for i, coin := range spendable {
		memStore.Set(sdk.Uint64ToBigEndian(uint64(i)), k.cdc.MustMarshal(&coin))
	}

	var paginatedSpendable sdk.Coins
	pageRes, err := query.Paginate(memStore, req.Pagination, func(key []byte, value []byte) error {
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		paginatedSpendable = append(paginatedSpendable, coin)
		return nil
	})

	if err != nil {
		return nil, status.Errorf(codes.InvalidArgument, "paginate: %v", err)
	}

	return &types.QuerySpendableBalancesResponse{
		Balances:   paginatedSpendable,
		Pagination: pageRes,
	}, nil
}
