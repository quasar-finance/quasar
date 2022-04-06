package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

func (k msgServer) SendIbcExitPool(goCtx context.Context, msg *types.MsgSendIbcExitPool) (*types.MsgSendIbcExitPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: logic before transmitting the packet

	// Construct the packet
	var packet types.IbcExitPoolPacketData

	packet.PoolId = msg.PoolId
	packet.ShareInAmount = msg.ShareInAmount
	packet.TokenOutMins = msg.TokenOutMins

	// Transmit the packet
	err := k.TransmitIbcExitPoolPacket(
		ctx,
		packet,
		msg.Port,
		msg.ChannelID,
		clienttypes.ZeroHeight(),
		msg.TimeoutTimestamp,
	)
	if err != nil {
		return nil, err
	}

	return &types.MsgSendIbcExitPoolResponse{}, nil
}
