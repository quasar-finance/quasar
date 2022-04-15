package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) ExitPool(goCtx context.Context, msg *types.MsgExitPool) (*types.MsgExitPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	err := k.TransmitIbcExitPool(ctx,
		msg.Creator,
		msg.ConnectionId,
		msg.TimeoutTimestamp,
		msg.PoolId,
		msg.ShareInAmount,
		msg.TokenOutMins)
	if err != nil {
		return nil, err
	}

	return &types.MsgExitPoolResponse{}, nil
}
