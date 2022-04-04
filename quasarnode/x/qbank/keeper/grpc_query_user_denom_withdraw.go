package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// UserDenomWithdraw is used by CLI and grpc query to fetch user's denom withdrable amount.
// Currently this function is returning withdrable amount.
func (k Keeper) UserDenomWithdraw(goCtx context.Context, req *types.QueryUserDenomWithdrawRequest) (*types.QueryUserDenomWithdrawResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	k.Logger(ctx).Info(fmt.Sprintf("UserDenomWithdraw|%s|%s\n", req.GetUserAcc(), req.GetDenom()))

	token := k.GetWithdrawableAmt(ctx, req.UserAcc, req.Denom)
	return &types.QueryUserDenomWithdrawResponse{Amount: token.Amount.Uint64()}, nil
}
