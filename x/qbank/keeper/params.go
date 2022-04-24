package keeper

import (
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.MinOrionEpochDollarDeposit(ctx),
		k.WhiteListedDenomsInOrion(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// MinOrionEpochDollarDeposit returns the value of min epoch dollar deposit amount
func (k Keeper) MinOrionEpochDollarDeposit(ctx sdk.Context) (res sdk.Dec) {
	k.paramstore.Get(ctx, types.KeyMinOrionEpochDollarDeposit, &res)
	return
}

// WhiteListedDenomsInOrion returns the []types.WhiteListedDenomInOrion param
func (k Keeper) WhiteListedDenomsInOrion(ctx sdk.Context) (res []types.WhiteListedDenomInOrion) {
	k.paramstore.Get(ctx, types.KeyWhiteListedDenomsInOrion, &res)
	return
}
