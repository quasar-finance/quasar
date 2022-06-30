package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) GetPortInfo(goCtx context.Context, req *types.QueryGetPortInfoRequest) (*types.QueryGetPortInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	pi, _ := k.GetPortDetail(ctx, req.DestinationChainID, req.PortID) // osmosis TODO to arguments.
	return &types.QueryGetPortInfoResponse{PortInfo: pi}, nil
}
