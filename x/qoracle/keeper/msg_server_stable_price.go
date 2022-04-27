package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) StablePrice(goCtx context.Context, msg *types.MsgStablePrice) (*types.MsgStablePriceResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	price := sdk.MustNewDecFromStr(msg.Price)
	if price.IsNil() || price.IsNegative() {
		return nil, types.ErrInvalidStablePrice
	}
	// AUDIT TODO : oracle account validation to be added.

	k.SetStablePrice(ctx, msg.Denom, price)

	return &types.MsgStablePriceResponse{}, nil
}
