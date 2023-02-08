package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcJoinPool(goCtx context.Context, pool *types.MsgTransmitIbcJoinPool) (*types.MsgTransmitIbcJoinPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	owner := owner
	seq, channel, portId, err := ms.k.TransmitIbcBeginUnlocking(ctx, owner, pool.GetConnectionId(), pool.GetTimeoutTimestamp(), pool.GetPoolId(), pool.GetTokenInMaxs())
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcJoinPoolResponse{Seq: seq, Channel: channel, PortId: portId}, nil
}
