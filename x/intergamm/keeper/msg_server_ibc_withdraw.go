package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

func (k msgServer) SendIbcWithdraw(goCtx context.Context, msg *types.MsgSendIbcWithdraw) (*types.MsgSendIbcWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: logic before transmitting the packet

	// Construct the packet
	var packet types.IbcWithdrawPacketData

	packet.TransferPort = msg.TransferPort
	packet.TransferChannel = msg.TransferChannel
	packet.Receiver = msg.Receiver
	packet.Assets = msg.Assets

	// Transmit the packet
	err := k.TransmitIbcWithdrawPacket(
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

	return &types.MsgSendIbcWithdrawResponse{}, nil
}
