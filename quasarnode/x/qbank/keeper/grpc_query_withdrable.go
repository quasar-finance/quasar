package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) Withdrable(goCtx context.Context, req *types.QueryWithdrableRequest) (*types.QueryWithdrableResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Process the query

	coin := k.GetWithdrawableAmt(ctx, req.UserAccount, req.Denom)

	return &types.QueryWithdrableResponse{Coin: coin}, nil
}
