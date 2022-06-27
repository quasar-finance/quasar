package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(k.PerfFeePer(ctx),
		k.MgmtFeePer(ctx),
		k.Enabled(ctx),
		k.LpEpochId(ctx),
		k.WhiteListedPools(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// PerfFeePer returns the value of per-fee in sdk.Dec
func (k Keeper) PerfFeePer(ctx sdk.Context) (res sdk.Dec) {
	k.paramstore.Get(ctx, types.KeyPerfFeePer, &res)
	return
}

// MgmtFeePer returns the value of per-fee in sdk.Dec
func (k Keeper) MgmtFeePer(ctx sdk.Context) (res sdk.Dec) {
	k.paramstore.Get(ctx, types.KeyMgmtFeePer, &res)
	return
}

// LpEpochId returns the value of per-fee in sdk.Dec
func (k Keeper) LpEpochId(ctx sdk.Context) (res string) {
	k.paramstore.Get(ctx, types.KeyLpEpochId, &res)
	return
}

// Enabled returns the value of Orion vault enabled param in bool
func (k Keeper) Enabled(ctx sdk.Context) (res bool) {
	k.paramstore.Get(ctx, types.KeyEnabled, &res)
	return
}

func (k Keeper) WhiteListedPools(ctx sdk.Context) (res []uint64) {
	k.paramstore.Get(ctx, types.KeyWhiteListedPools, &res)
	return
}
