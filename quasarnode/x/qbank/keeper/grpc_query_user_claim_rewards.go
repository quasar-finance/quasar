package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) UserClaimRewards(goCtx context.Context, req *types.QueryUserClaimRewardsRequest) (*types.QueryUserClaimRewardsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	k.Logger(ctx).Info(fmt.Sprintf("UserClaimRewards|%s\n", req.GetUserAcc()))
	var qcoins types.QCoins

	// TODO | AUDIT | Get the claimable amount from the orion vault.
	k.Logger(ctx).Info(fmt.Sprintf("UserClaimRewards|%s\n", qcoins.Coins.String()))

	return &types.QueryUserClaimRewardsResponse{Coins: qcoins}, nil
}
