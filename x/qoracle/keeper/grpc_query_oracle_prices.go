package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) OraclePrices(goCtx context.Context, req *types.QueryOraclePricesRequest) (*types.QueryOraclePricesResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	op := k.GetOraclePrices(ctx)

	return &types.QueryOraclePricesResponse{
		Prices:          op.Prices,
		UpdatedAtHeight: op.UpdatedAtHeight,
	}, nil
}
