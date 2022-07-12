package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) AddDenomPriceMapping(goCtx context.Context, msg *types.MsgAddDenomPriceMapping) (*types.MsgAddDenomPriceMappingResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if k.authority != msg.Creator {
		return nil, types.ErrUnauthorized
	}

	err := k.AddDenomMapping(ctx, msg.Mapping)
	if err != nil {
		return nil, err
	}
	return &types.MsgAddDenomPriceMappingResponse{}, nil
}
