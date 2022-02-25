package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v2/modules/core/02-client/types"
)

func (k msgServer) SendIbcCreatePool(goCtx context.Context, msg *types.MsgSendIbcCreatePool) (*types.MsgSendIbcCreatePoolResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: logic before transmitting the packet

	// Construct the packet
	var packet types.IbcCreatePoolPacketData

	packet.Assets = msg.PoolAssets
	packet.Params = msg.PoolParams
	packet.FuturePoolGovernor = msg.FuturePoolGovernor

	// Transmit the packet
	err := k.TransmitIbcCreatePoolPacket(
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

	return &types.MsgSendIbcCreatePoolResponse{}, nil
}
