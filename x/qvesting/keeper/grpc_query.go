package keeper

import (
	"context"
	"fmt"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/store/mem"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting/exported"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"google.golang.org/grpc/codes"

	"google.golang.org/grpc/status"
)

var _ types.QueryServer = Keeper{}

func (k Keeper) Params(c context.Context, req *types.QueryParamsRequest) (*types.QueryParamsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(c)

	return &types.QueryParamsResponse{Params: k.GetParams(ctx)}, nil
}

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

func (k Keeper) VestingAccounts(c context.Context, req *types.QueryVestingAccountsRequest) (*types.QueryVestingAccountsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	ctx := sdk.UnwrapSDKContext(c)
	store := ctx.KVStore(k.storeKey)
	accountsStore := prefix.NewStore(store, types.VestingAccountStoreKeyPrefix)

	var accounts []*codectypes.Any
	pageRes, err := query.Paginate(accountsStore, req.Pagination, func(key, value []byte) error {
		addr := sdk.AccAddress(key)
		acct := k.accountKeeper.GetAccount(ctx, addr)
		vestingAcct, ok := acct.(exported.VestingAccount)
		if !ok {
			return fmt.Errorf("account is not vesting account: %s", addr.String())
		}

		any, err := codectypes.NewAnyWithValue(vestingAcct)
		if err != nil {
			return err
		}
		accounts = append(accounts, any)
		return nil
	})

	if err != nil {
		return nil, status.Errorf(codes.Internal, "paginate: %v", err)
	}

	return &types.QueryVestingAccountsResponse{Accounts: accounts, Pagination: pageRes}, err

}
