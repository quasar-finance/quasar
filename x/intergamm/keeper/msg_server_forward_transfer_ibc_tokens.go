package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (s msgServer) ForwardTransferIbcTokens(goCtx context.Context, msg *types.MsgForwardTransferIbcTokens) (*types.MsgForwardTransferIbcTokensResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if err := s.k.ForwardTransferIbcTokens(
		ctx,
		msg.SourcePort,
		msg.SourceChannel,
		msg.Token,
		sdk.AccAddress(msg.Creator),
		msg.ForwardTransferPort,
		msg.ForwardTransferChannel,
		msg.IntermediateReceiver,
		msg.Receiver,
		msg.TimeoutHeight,
		msg.TimeoutTimestamp,
	); err != nil {
		return nil, err
	}

	return &types.MsgForwardTransferIbcTokensResponse{}, nil
}
