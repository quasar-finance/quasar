package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.DenomToNativeZoneIdMap(ctx),
		k.CompleteZoneInfoMap(ctx))
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

func (k Keeper) DenomToNativeZoneIdMap(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyDenomToNativeZoneIdMap, &res)
	return
}

// CompleteZoneInfoMap returns a map containing IBC routing info among all zones.
func (k Keeper) CompleteZoneInfoMap(ctx sdk.Context) (res map[string]types.ZoneCompleteInfo) {
	k.paramstore.Get(ctx, types.KeyCompleteZoneInfoMap, &res)
	return
}
