package keeper

import (
	"context"
	"fmt"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting/exported"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) VestingAccounts(c context.Context, req *types.QueryVestingAccountsRequest) (*types.QueryVestingAccountsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	ctx := sdk.UnwrapSDKContext(c)
	store := ctx.KVStore(k.storeKey)
	accountsStore := prefix.NewStore(store, types.VestingAccountStoreKeyPrefix)

	//k.IterateVestingAccounts(ctx)
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
