package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) UserDenomEpochLockupDeposit(goCtx context.Context, req *types.QueryUserDenomEpochLockupDepositRequest) (*types.QueryUserDenomEpochLockupDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Process the query
	_ = ctx

	k.Logger(ctx).Info(fmt.Sprintf("UserDenomEpochLockupDeposit|%s|%s\n", req.GetUserAcc(),
		req.GetDenom()))

	tokens, found := k.GetUserDenomEpochLockupDepositAmount(ctx,
		req.GetUserAcc(), req.GetDenom(), req.EpochDay[0],
		types.LockupTypes(types.LockupTypes_value[req.GetLockupType()]))
	if !found {
		return nil, sdkerrors.ErrKeyNotFound
	}

	return &types.QueryUserDenomEpochLockupDepositResponse{Amount: tokens.Amount.Uint64()}, nil
}
