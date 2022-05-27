package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (s msgServer) TransferIbcTokens(goCtx context.Context, msg *types.MsgTransferIbcTokens) (*types.MsgTransferIbcTokensResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if err := s.k.TransferIbcTokens(
		ctx,
		msg.SourcePort,
		msg.SourceChannel,
		msg.Token,
		sdk.AccAddress(msg.Creator),
		msg.Receiver,
		msg.TimeoutHeight,
		msg.TimeoutTimestamp,
	); err != nil {
		return nil, err
	}

	return &types.MsgTransferIbcTokensResponse{}, nil
}
