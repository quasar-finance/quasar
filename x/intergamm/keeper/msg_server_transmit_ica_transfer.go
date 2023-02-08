package keeper

import (
	"context"

	ibcclienttypes "github.com/cosmos/ibc-go/v6/modules/core/02-client/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitICATransfer(goCtx context.Context, msg *types.MsgTransmitICATransfer) (*types.MsgTransmitICATransferResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	msgTransmitTimeoutTimestamp := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	icaTransferTimeoutTimestamp := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp*2
	icaTransferTimeoutHeight := ibcclienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	seq, channel, portId, err := ms.k.TransmitICATransfer(ctx, msg.IcaOwnerAddress, msgTransmitTimeoutTimestamp, msg.Coin, msg.ToAddress, icaTransferTimeoutHeight, icaTransferTimeoutTimestamp)
	if err != nil {
		return nil, err
	}

	return &types.MsgTransmitICATransferResponse{Seq: seq, Channel: channel, PortId: portId}, nil
}
