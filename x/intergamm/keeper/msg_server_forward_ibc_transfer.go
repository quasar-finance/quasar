package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (s msgServer) ForwardIbcTransfer(goCtx context.Context, msg *types.MsgForwardIbcTransfer) (*types.MsgForwardIbcTransferResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	_, err := s.k.TransmitForwardIbcTransfer(ctx,
		msg.Creator,
		msg.ConnectionId,
		msg.TimeoutTimestamp,
		msg.TransferPort,
		msg.TransferChannel,
		msg.Token,
		msg.ForwardTransferPort,
		msg.ForwardTransferChannel,
		msg.IntermediateReceiver,
		msg.Receiver,
		msg.TransferTimeoutHeight,
		msg.TransferTimeoutTimestamp)
	if err != nil {
		return nil, err
	}

	return &types.MsgForwardIbcTransferResponse{}, nil
}
