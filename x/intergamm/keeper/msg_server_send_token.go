package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) SendToken(goCtx context.Context, token *types.MsgSendToken) (*types.MsgSendTokenResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	sender, err := sdk.AccAddressFromBech32(token.Sender)
	if err != nil {
		return nil, err
	}

	seq, err := ms.k.SendToken(ctx, token.DestinationLocalZoneId, sender, token.Receiver, *token.Coin)
	if err != nil {
		return nil, err
	}
	return &types.MsgSendTokenResponse{Seq: seq}, nil
}
