package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) JoinPool(goCtx context.Context, msg *types.MsgJoinPool) (*types.MsgJoinPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	err := k.TransmitIbcJoinPool(ctx,
		msg.Creator,
		msg.ConnectionId,
		msg.TimeoutTimestamp,
		msg.PoolId,
		msg.ShareOutAmount,
		msg.TokenInMaxs)
	if err != nil {
		return nil, err
	}

	return &types.MsgJoinPoolResponse{}, nil
}
