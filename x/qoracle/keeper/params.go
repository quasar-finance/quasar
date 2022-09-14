package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.BandchainParams(ctx),
		k.OsmosisParams(ctx),
		k.GetDenomPriceMappings(ctx),
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

func (k Keeper) OsmosisParams(ctx sdk.Context) (res types.OsmosisParams) {
	k.paramstore.Get(ctx, types.KeyOsmosisParams, &res)
	return
}

func (k Keeper) SetOsmosisParams(ctx sdk.Context, params types.OsmosisParams) {
	k.paramstore.Set(ctx, types.KeyOsmosisParams, &params)
}

func (k Keeper) GetDenomPriceMappings(ctx sdk.Context) (res []types.DenomPriceMapping) {
	k.paramstore.Get(ctx, types.KeyDenomPriceMappings, &res)
	return
}

func (k Keeper) SetDenomPriceMappings(ctx sdk.Context, mappings []types.DenomPriceMapping) {
	k.paramstore.Set(ctx, types.KeyDenomPriceMappings, &mappings)
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
