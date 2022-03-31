package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// AUDIT | Deprecated
// DepositAll is used by the CLI and grpc query to fetch the list of all active deposit in the qbank.
func (k Keeper) DepositAll(c context.Context, req *types.QueryAllDepositRequest) (*types.QueryAllDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	var deposits []types.Deposit
	ctx := sdk.UnwrapSDKContext(c)

	store := ctx.KVStore(k.storeKey)
	//depositStore := prefix.NewStore(store, types.KeyPrefix(types.DepositKey))
	depositStore := prefix.NewStore(store, types.DepositKBP)

	pageRes, err := query.Paginate(depositStore, req.Pagination, func(key []byte, value []byte) error {
		var deposit types.Deposit
		if err := k.cdc.Unmarshal(value, &deposit); err != nil {
			return err
		}

		deposits = append(deposits, deposit)
		return nil
	})

	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryAllDepositResponse{Deposit: deposits, Pagination: pageRes}, nil
}

// AUDIT | Deprecated
// Deposit is used by the CLI and grpc query to fetch specific deposit object with the deposit ID.
func (k Keeper) Deposit(c context.Context, req *types.QueryGetDepositRequest) (*types.QueryGetDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(c)
	_ = ctx

	return &types.QueryGetDepositResponse{}, nil
}
