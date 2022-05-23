package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) TransferIbcTokens(goCtx context.Context, msg *types.MsgTransferIbcTokens) (*types.MsgTransferIbcTokensResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx

	return &types.MsgTransferIbcTokensResponse{}, nil
}
