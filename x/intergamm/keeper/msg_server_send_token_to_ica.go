package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) SendTokenToICA(goCtx context.Context, msg *types.MsgSendTokenToICA) (*types.MsgSendTokenToICAResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	fromAddress, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		return nil, err
	}

	seq, err := ms.k.SendTokenToICA(ctx, msg.ToZoneId, fromAddress, msg.Coin)
	if err != nil {
		return nil, err
	}

	return &types.MsgSendTokenToICAResponse{Seq: seq}, nil
}
