package keeper

import (
	"context"
	"fmt"

	"github.com/quasarlabs/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// UserDeposit is used by the CLI and grpc query to fetch the current value of the users deposited amount.
func (k Keeper) UserDeposit(goCtx context.Context, req *types.QueryUserDepositRequest) (*types.QueryUserDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	k.Logger(ctx).Info(fmt.Sprintf("UserDeposit|%s\n", req.GetUserAcc()))

	qcoins, found := k.GetUserDepositAmt(ctx, req.GetUserAcc())
	if !found {
		return nil, sdkerrors.ErrKeyNotFound
	}
	k.Logger(ctx).Info(fmt.Sprintf("UserDepositAmount|%s\n", qcoins.Coins.String()))
	return &types.QueryUserDepositResponse{Coins: qcoins}, nil
}
