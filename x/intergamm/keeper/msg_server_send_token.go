package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) SendToken(goCtx context.Context, msg *types.MsgSendToken) (*types.MsgSendTokenResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	fromAddress, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		return nil, err
	}

	seq, channel, portId, err := ms.k.SendToken(ctx, msg.ToZoneId, fromAddress, msg.ToAddress, msg.Coin)
	if err != nil {
		return nil, err
	}

	return &types.MsgSendTokenResponse{Seq: seq, Channel: channel, PortId: portId}, nil
}
