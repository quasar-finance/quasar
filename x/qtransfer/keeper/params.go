package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qtransfer/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.WasmHooksEnabled(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramSpace.SetParamSet(ctx, &params)
}

// WasmHooksEnabled returns whether wasm hooks are enabled
func (k Keeper) WasmHooksEnabled(ctx sdk.Context) (res bool) {
	k.paramSpace.Get(ctx, types.KeyWasmHooksEnabled, &res)
	return res
}
