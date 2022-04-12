package keeper

import (
	"context"
	"fmt"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// UserClaimRewards is used by CLI and grpc queries to
func (k Keeper) UserClaimRewards(goCtx context.Context, req *types.QueryUserClaimRewardsRequest) (*types.QueryUserClaimRewardsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)
	k.Logger(ctx).Info(fmt.Sprintf("UserClaimRewards|%s\n", req.GetUserAcc()))
	qcoins, _ := k.GetUserClaimAmt(ctx, req.UserAcc, oriontypes.ModuleName)
	k.Logger(ctx).Info(fmt.Sprintf("UserClaimRewards|%s\n", qcoins.Coins.String()))
	return &types.QueryUserClaimRewardsResponse{Coins: qcoins}, nil
}
