package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// TotalClaimed fetch the total amount of tokens fetch so far by the given user.
func (k Keeper) TotalClaimed(goCtx context.Context, req *types.QueryTotalClaimedRequest) (*types.QueryTotalClaimedResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	val, _ := k.GetUserClaimedAmt(ctx, req.UserAcc, req.VaultID)
	return &types.QueryTotalClaimedResponse{Coins: val}, nil
}
