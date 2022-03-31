package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// AUDIT | Deprecated
// UserDenomLockupDeposit is used by the CLI and grpc query to fetch the denom deposit value of a give user with specific lockup duration.
func (k Keeper) UserDenomLockupDeposit(goCtx context.Context, req *types.QueryUserDenomLockupDepositRequest) (*types.QueryUserDenomLockupDepositResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	// _ = ctx

	k.Logger(ctx).Info(fmt.Sprintf("UserDenomLockupDeposit|%s|%s|%s\n", req.GetUserAcc(),
		req.GetDenom(), req.GetLockupType()))

	var tokens sdk.Coin
	return &types.QueryUserDenomLockupDepositResponse{Amount: tokens.Amount.Uint64()}, nil
}
