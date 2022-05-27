package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (s msgServer) RegisterAccount(goCtx context.Context, msg *types.MsgRegisterAccount) (*types.MsgRegisterAccountResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if err := s.k.RegisterInterchainAccount(ctx, msg.ConnectionId, msg.Creator); err != nil {
		return nil, err
	}

	return &types.MsgRegisterAccountResponse{}, nil
}
