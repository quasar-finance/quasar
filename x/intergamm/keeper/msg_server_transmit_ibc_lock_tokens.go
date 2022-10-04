package keeper

import (
	"context"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcLockTokens(goCtx context.Context, tokens *types.MsgTransmitIbcLockTokens) (*types.MsgTransmitIbcLockTokensResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	owner := tokens.Creator
	seq, channel, err := ms.k.TransmitIbcLockTokens(ctx, owner, tokens.GetConnectionId(), tokens.GetTimeoutTimestamp(), tokens.GetDuration(), tokens.GetCoins())
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcLockTokensResponse{Seq: seq, Channel: channel}, nil
}
