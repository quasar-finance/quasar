package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) RegisterICAOnZone(goCtx context.Context, msg *types.MsgRegisterICAOnZone) (*types.MsgRegisterICAOnZoneResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	err := ms.k.RegisterICAOnZoneId(ctx, msg.ZoneId, msg.OwnerAddress)
	if err != nil {
		return nil, err
	}

	return &types.MsgRegisterICAOnZoneResponse{}, nil
}
