package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.OracleAccounts(ctx),
		k.StableDenoms(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// OracleAccounts returns the OracleAccounts param
func (k Keeper) OracleAccounts(ctx sdk.Context) (res string) {
	k.paramstore.Get(ctx, types.KeyOracleAccounts, &res)
	return
}

// OracleAccounts returns the OracleAccounts param
func (k Keeper) StableDenoms(ctx sdk.Context) (res []string) {
	k.paramstore.Get(ctx, types.KeyOracleAccounts, &res)
	return
}
