package keeper

import (
	"context"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcExitPool(goCtx context.Context, pool *types.MsgTransmitIbcExitPool) (*types.MsgTransmitIbcExitPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	owner := pool.Creator
	seq, err := ms.k.TransmitIbcBeginUnlocking(ctx, owner, pool.GetConnectionId(), pool.GetTimeoutTimestamp(), pool.GetPoolId(), pool.GetTokenOutMins())
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcExitPoolResponse{Seq: seq}, nil
}
