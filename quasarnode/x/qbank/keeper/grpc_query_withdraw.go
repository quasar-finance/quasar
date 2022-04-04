package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/cosmos/cosmos-sdk/types/query"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// WithdrawAll
// TODO | This function is to be removed.
func (k Keeper) WithdrawAll(c context.Context, req *types.QueryAllWithdrawRequest) (*types.QueryAllWithdrawResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	var withdraws []types.Withdraw
	ctx := sdk.UnwrapSDKContext(c)

	store := ctx.KVStore(k.storeKey)
	withdrawStore := prefix.NewStore(store, types.KeyPrefix(types.WithdrawKey))

	pageRes, err := query.Paginate(withdrawStore, req.Pagination, func(key []byte, value []byte) error {
		var withdraw types.Withdraw
		if err := k.cdc.Unmarshal(value, &withdraw); err != nil {
			return err
		}

		withdraws = append(withdraws, withdraw)
		return nil
	})

	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryAllWithdrawResponse{Withdraw: withdraws, Pagination: pageRes}, nil
}

// Withdraw
// TODO | AUDIT | This function to be removed.
func (k Keeper) Withdraw(c context.Context, req *types.QueryGetWithdrawRequest) (*types.QueryGetWithdrawResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(c)
	withdraw, found := k.GetWithdraw(ctx, req.Id)
	if !found {
		return nil, sdkerrors.ErrKeyNotFound
	}

	return &types.QueryGetWithdrawResponse{Withdraw: withdraw}, nil
}
