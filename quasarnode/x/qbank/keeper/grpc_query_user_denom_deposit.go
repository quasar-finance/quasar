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

// UserDenomDeposit is used by the CLI and grpc query to fetch the denom deposit value of a give user.
func (k Keeper) UserDenomDeposit(goCtx context.Context,
	req *types.QueryUserDenomDepositRequest) (*types.QueryUserDenomDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	k.Logger(ctx).Info(fmt.Sprintf("UserDenomDeposit|%s|%s\n", req.GetUserAcc(),
		req.GetDenom()))
	tokens, found := k.GetUserDenomDepositAmt(ctx,
		req.GetUserAcc(), req.GetDenom())
	if !found {
		return nil, sdkerrors.ErrKeyNotFound
	}

	return &types.QueryUserDenomDepositResponse{Amount: tokens.Amount.Uint64()}, nil
}
