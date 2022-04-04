package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) IbcTransfer(goCtx context.Context, msg *types.MsgIbcTransfer) (*types.MsgIbcTransferResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	err := k.TransmitIbcTransfer(ctx,
		msg.Creator,
		msg.ConnectionId,
		msg.TimeoutTimestamp,
		msg.TransferPort,
		msg.TransferChannel,
		msg.Token,
		msg.Receiver,
		msg.TransferTimeoutHeight,
		msg.TransferTimeoutTimestamp)
	if err != nil {
		return nil, err
	}

	return &types.MsgIbcTransferResponse{}, nil
}
