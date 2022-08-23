package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer)RegisterInterchainAccount(goCtx context.Context, register *types.MsgRegisterInterchainAccount) (*types.MsgRegisterInterchainAccountResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	creator, err := sdk.AccAddressFromBech32(register.Creator)
	if err != nil {
		return nil, err
	}

	err = ms.k.RegisterInterchainAccount(ctx, register.ConnectionId, creator.String())
	if err != nil {
		return nil, err
	}
	return &types.MsgRegisterInterchainAccountResponse{}, nil
}