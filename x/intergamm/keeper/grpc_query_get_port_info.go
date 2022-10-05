package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) GetPortInfo(goCtx context.Context, req *types.QueryGetPortInfoRequest) (*types.QueryGetPortInfoResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	pi, _ := k.GetPortDetail(ctx, req.DestinationChainID, req.PortID)
	return &types.QueryGetPortInfoResponse{PortInfo: pi}, nil
}
