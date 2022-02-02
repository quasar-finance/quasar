package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) UserWithdraw(goCtx context.Context, req *types.QueryUserWithdrawRequest) (*types.QueryUserWithdrawResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	// _ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("UserWithdraw|%s\n", req.GetUserAcc()))
	// TODO - Get current withdrawable amount from Orion vault

	var qcoins types.QCoins
	k.Logger(ctx).Info(fmt.Sprintf("UserDepositAmount|%s\n", qcoins.Coins.String()))
	return &types.QueryUserWithdrawResponse{Coins: qcoins}, nil
}
