package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) (params types.Params) {
	k.paramstore.GetParamSet(ctx, &params)
	return params
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// OracleAccounts returns the OracleAccounts param
func (k Keeper) OracleAccounts(ctx sdk.Context) string {
	return k.GetParams(ctx).OracleAccounts
}

// StableDenoms returns the StableDenoms param
func (k Keeper) StableDenoms(ctx sdk.Context) []string {
	return k.GetParams(ctx).StableDenoms
}

// OneHopDenomMap returns the OneHopDenomMap param
func (k Keeper) OneHopDenomMap(ctx sdk.Context) []*types.OneHopIbcDenomMapping {
	return k.GetParams(ctx).OneHopDenomMap
}
