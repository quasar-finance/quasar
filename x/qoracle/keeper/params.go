package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.BandchainParams(ctx),
		k.OracleAccounts(ctx),
		k.StableDenoms(ctx),
		k.OneHopDenomMap(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// BandchainIBCParams returns the BandchainIBCParams param
func (k Keeper) BandchainParams(ctx sdk.Context) (res types.BandchainParams) {
	k.paramstore.Get(ctx, types.KeyBandchainParams, &res)
	return
}

func (k Keeper) SetBandchainParams(ctx sdk.Context, params types.BandchainParams) {
	k.paramstore.Set(ctx, types.KeyBandchainParams, &params)
}

// OracleAccounts returns the OracleAccounts param
func (k Keeper) OracleAccounts(ctx sdk.Context) (res string) {
	k.paramstore.Get(ctx, types.KeyOracleAccounts, &res)
	return
}

// StableDenoms returns the StableDenoms param
func (k Keeper) StableDenoms(ctx sdk.Context) (res []string) {
	k.paramstore.Get(ctx, types.KeyStableDenoms, &res)
	return
}

// OneHopDenomMap returns the OneHopIbcDenomMapping param
func (k Keeper) OneHopDenomMap(ctx sdk.Context) (res []*types.OneHopIbcDenomMapping) {
	k.paramstore.Get(ctx, types.KeyOneHopDenomMap, &res)
	return
}
