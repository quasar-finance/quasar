package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) RegisterICAOnDenomNativeZone(goCtx context.Context, msg *types.MsgRegisterICAOnDenomNativeZone) (*types.MsgRegisterICAOnDenomNativeZoneResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	err := ms.k.RegisterICAOnDenomNativeZone(ctx, msg.Denom, msg.OwnerAddress)
	if err != nil {
		return nil, err
	}

	return &types.MsgRegisterICAOnDenomNativeZoneResponse{}, nil
}
