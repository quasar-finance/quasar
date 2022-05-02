package keeper

import (
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.Enabled(ctx),
		k.MinOrionEpochDenomDollarDeposit(ctx),
		k.OrionEpochIdentifier(ctx),
		k.WhiteListedDenomsInOrion(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// Enabled returns the value of qbank module enabled param
func (k Keeper) Enabled(ctx sdk.Context) (res bool) {
	k.paramstore.Get(ctx, types.KeyEnabled, &res)
	return
}

// MinOrionEpochDenomDollarDeposit returns the value of min epoch dollar deposit amount
func (k Keeper) MinOrionEpochDenomDollarDeposit(ctx sdk.Context) (res sdk.Dec) {
	k.paramstore.Get(ctx, types.KeyMinOrionEpochDenomDollarDeposit, &res)
	return
}

// OrionEpochIdentifier returns the OrionEpochIdentifier  param
func (k Keeper) OrionEpochIdentifier(ctx sdk.Context) (res string) {
	k.paramstore.Get(ctx, types.KeyOrionEpochIdentifier, &res)
	return
}

// WhiteListedDenomsInOrion returns the []types.WhiteListedDenomInOrion param
func (k Keeper) WhiteListedDenomsInOrion(ctx sdk.Context) (res []types.WhiteListedDenomInOrion) {
	k.paramstore.Get(ctx, types.KeyWhiteListedDenomsInOrion, &res)
	return
}
