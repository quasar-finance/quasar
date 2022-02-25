package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v2/modules/core/02-client/types"
)

func (k msgServer) SendIbcJoinPool(goCtx context.Context, msg *types.MsgSendIbcJoinPool) (*types.MsgSendIbcJoinPoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: logic before transmitting the packet

	// Construct the packet
	var packet types.IbcJoinPoolPacketData

	packet.PoolId = msg.PoolId
	packet.ShareOutAmount = msg.ShareOutAmount
	packet.TokenInMaxs = msg.TokenInMaxs

	// Transmit the packet
	err := k.TransmitIbcJoinPoolPacket(
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

	return &types.MsgSendIbcJoinPoolResponse{}, nil
}
