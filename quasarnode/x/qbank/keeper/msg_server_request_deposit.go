package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx

	return &types.MsgRequestDepositResponse{}, nil
}
