package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// Withdrawable is used by CLI and grpc query to fetch user's denom withdrawable amount.
func (k Keeper) Withdrawable(goCtx context.Context, req *types.QueryWithdrawableRequest) (*types.QueryWithdrawableResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)
	coin := k.GetWithdrawableAmt(ctx, req.UserAccount, req.Denom)
	return &types.QueryWithdrawableResponse{Coin: coin}, nil
}
