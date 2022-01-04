package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/osmolpv/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) ReserveBalance(goCtx context.Context, req *types.QueryReserveBalanceRequest) (*types.QueryReserveBalanceResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Process the query
	// _ = ctx

	k.Logger(ctx).Info(fmt.Sprintf("QueryReserveBalance|%s\n",
		req.GetDenom()))

	coin := k.GetReserveBalance(ctx, req.GetDenom())

	return &types.QueryReserveBalanceResponse{Amount: coin.Amount.Uint64()}, nil
}
