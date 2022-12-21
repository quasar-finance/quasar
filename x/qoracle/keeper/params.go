package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.GetDenomPriceMappings(ctx),
		k.OneHopDenomMap(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

func (k Keeper) GetDenomPriceMappings(ctx sdk.Context) (res []types.DenomPriceMapping) {
	k.paramstore.Get(ctx, types.KeyDenomPriceMappings, &res)
	return
}

func (k Keeper) SetDenomPriceMappings(ctx sdk.Context, mappings []types.DenomPriceMapping) {
	k.paramstore.Set(ctx, types.KeyDenomPriceMappings, &mappings)
}

// OneHopDenomMap returns the OneHopIbcDenomMapping param
func (k Keeper) OneHopDenomMap(ctx sdk.Context) (res []*types.OneHopIbcDenomMapping) {
	k.paramstore.Get(ctx, types.KeyOneHopDenomMap, &res)
	return
}
