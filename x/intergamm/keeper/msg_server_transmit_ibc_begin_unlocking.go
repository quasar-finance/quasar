package keeper

import (
	"context"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcBeginUnlocking(goCtx context.Context, unlocking *types.MsgTransmitIbcBeginUnlocking) (*types.MsgTransmitIbcBeginUnlockingResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	owner := unlocking.Creator
	seq, err := ms.k.TransmitIbcBeginUnlocking(ctx, owner, unlocking.GetConnectionId(), unlocking.GetTimeoutTimestamp(), unlocking.GetId(), unlocking.GetCoins())
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcBeginUnlockingResponse{Seq: seq}, nil
}
