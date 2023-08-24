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
	"strconv"

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

// VestingAccounts queries and returns a list of vesting accounts present in the chain.
//
// The function fetches accounts from two main creation sources:
// 1. Genesis: Vesting accounts that were set up during the genesis of the chain with auth module.
// 2. Custom qVesting module: Vesting accounts created after the genesis using the custom module.
//
// To achieve this, the function leverages the GetAllAccounts method from the auth module's keeper.
// This method fetches all accounts (including both vesting and non-vesting accounts) from the auth module's store,
// which inherently includes accounts from both the genesis and any created afterward. The function then filters out
// only the vesting accounts and paginates the results based on the provided query request.
func (k Keeper) VestingAccounts(c context.Context, req *types.QueryVestingAccountsRequest) (*types.QueryVestingAccountsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	ctx := sdk.UnwrapSDKContext(c)
	allAccounts := k.accountKeeper.GetAllAccounts(ctx)

	var filteredVestingAccounts []exported.VestingAccount
	for _, acc := range allAccounts {
		// Trying to assert exported.VestingAccount
		vestingAcct, ok := acc.(exported.VestingAccount)
		if ok {
			filteredVestingAccounts = append(filteredVestingAccounts, vestingAcct)
		}
	}

	paginatedVestingAccounts, pageRes, err := paginateSlice(filteredVestingAccounts, req.Pagination)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "paginate: %v", err)
	}

	var vestingAccounts []*codectypes.Any
	for _, acct := range paginatedVestingAccounts {
		any, err := codectypes.NewAnyWithValue(acct)
		if err != nil {
			return nil, err
		}
		vestingAccounts = append(vestingAccounts, any)
	}

	return &types.QueryVestingAccountsResponse{Accounts: vestingAccounts, Pagination: pageRes}, nil
}

// QVestingAccounts queries and returns a list of vesting accounts present in the qvesting module filtering out the ones created with auth module.
func (k Keeper) QVestingAccounts(c context.Context, req *types.QueryQVestingAccountsRequest) (*types.QueryQVestingAccountsResponse, error) {
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

	return &types.QueryQVestingAccountsResponse{Accounts: accounts, Pagination: pageRes}, err
}

// SpendableBalances implements a gRPC query handler for retrieving an account's spendable balances.
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

// SpendableSupply provides an aggregated overview of the total of the spendable balances across all accounts for a given denom.
func (k Keeper) SpendableSupply(ctx context.Context, req *types.QuerySpendableSupplyRequest) (*types.QuerySpendableSupplyResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	sdkCtx := sdk.UnwrapSDKContext(ctx)
	resAmount := sdk.NewInt(0)

	// Iterate vesting accounts passing a callback function to invoke
	err := k.iterateVestingAccounts(sdkCtx, func(addr sdk.AccAddress) error {
		resAmount = resAmount.Add(k.bankKeeper.SpendableCoins(sdkCtx, addr).AmountOf(req.Denom))

		return nil
	})
	if err != nil {
		return nil, err
	}

	return &types.QuerySpendableSupplyResponse{Amount: sdk.Coin{Denom: req.Denom, Amount: resAmount}}, nil
}

// VestingLockedSupply returns the total amount of locked supply for a given denomination across all vesting accounts.
func (k Keeper) VestingLockedSupply(ctx context.Context, req *types.QueryVestingLockedSupplyRequest) (*types.QueryVestingLockedSupplyResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "empty request")
	}

	sdkCtx := sdk.UnwrapSDKContext(ctx)
	resAmount := sdk.NewInt(0)

	// Iterate vesting accounts passing a callback function to invoke
	err := k.iterateVestingAccounts(sdkCtx, func(addr sdk.AccAddress) error {
		vestingAcct := k.accountKeeper.GetAccount(sdkCtx, addr).(exported.VestingAccount)

		// Get the locked coins for this vesting account,
		// Get the amount of the requested denom from the locked coins
		// Add the locked amount to the result
		lockedCoins := vestingAcct.LockedCoins(sdkCtx.BlockTime())
		lockedAmount := lockedCoins.AmountOf(req.Denom)
		resAmount = resAmount.Add(lockedAmount)

		return nil
	})
	if err != nil {
		return nil, err
	}

	return &types.QueryVestingLockedSupplyResponse{Amount: sdk.Coin{Denom: req.Denom, Amount: resAmount}}, nil
}

// paginateSlice paginates a slice given the pagination params
func paginateSlice(accounts []exported.VestingAccount, req *query.PageRequest) (paginated []exported.VestingAccount, pageRes *query.PageResponse, err error) {
	total := len(accounts)

	if req.Limit == 0 {
		req.CountTotal = true
	}

	if len(req.Key) > 0 {
		var startIdx int
		startIdx, err = strconv.Atoi(string(req.Key))
		if err != nil {
			return nil, nil, err
		}
		if startIdx >= total {
			return nil, nil, status.Errorf(codes.InvalidArgument, "start index out of range")
		}
		accounts = accounts[startIdx:]
	} else if req.Offset > uint64(total) {
		return nil, nil, status.Errorf(codes.InvalidArgument, "offset out of range")
	} else {
		accounts = accounts[req.Offset:]
	}

	if req.Limit > 0 && int(req.Limit) <= len(accounts) {
		accounts = accounts[:req.Limit]
	}

	if req.Reverse {
		for i, j := 0, len(accounts)-1; i < j; i, j = i+1, j-1 {
			accounts[i], accounts[j] = accounts[j], accounts[i]
		}
	}

	nextKey := ""
	if len(accounts) < total {
		nextKey = strconv.Itoa(len(accounts))
	}

	pageRes = &query.PageResponse{
		NextKey: []byte(nextKey),
		Total:   uint64(total),
	}

	if req.CountTotal {
		pageRes.Total = uint64(total)
	}

	return accounts, pageRes, nil
}
