package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (s msgServer) CreatePool(goCtx context.Context, msg *types.MsgCreatePool) (*types.MsgCreatePoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	_, err := s.k.TransmitIbcCreatePool(ctx,
		msg.Creator,
		msg.ConnectionId,
		msg.TimeoutTimestamp,
		msg.PoolParams,
		msg.PoolAssets,
		msg.FuturePoolGovernor)
	if err != nil {
		return nil, err
	}

	return &types.MsgCreatePoolResponse{}, nil
}
