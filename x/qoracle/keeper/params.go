package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.GetDenomPriceMappings(ctx),
		k.GetPriceListExpDuration(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramSpace.SetParamSet(ctx, &params)
}

func (k Keeper) GetDenomPriceMappings(ctx sdk.Context) (res []types.DenomPriceMapping) {
	k.paramSpace.Get(ctx, types.KeyDenomPriceMappings, &res)
	return
}

func (k Keeper) SetDenomPriceMappings(ctx sdk.Context, mappings []types.DenomPriceMapping) {
	k.paramSpace.Set(ctx, types.KeyDenomPriceMappings, &mappings)
}

// GetPacketTimeoutTimestamp retrieves the price list expiration duration from the param space
func (k Keeper) GetPriceListExpDuration(ctx sdk.Context) (res uint64) {
	k.paramSpace.Get(ctx, types.KeyDenomPricesExpDuration, &res)
	return
}
