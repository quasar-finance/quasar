package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) UserDenomWithdraw(goCtx context.Context, req *types.QueryUserDenomWithdrawRequest) (*types.QueryUserDenomWithdrawResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	// _ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("UserDenomWithdraw|%s|%s\n", req.GetUserAcc(), req.GetDenom()))
	var token sdk.Coin

	// TODO - Get current withdrawable denom amount from Orion vault
	return &types.QueryUserDenomWithdrawResponse{Amount: token.Amount.Uint64()}, nil
}
