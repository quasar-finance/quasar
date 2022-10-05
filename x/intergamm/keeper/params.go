package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.QuasarDenomToNativeZoneIdMap(ctx),
		k.OsmosisDenomToQuasarDenomMap(ctx),
		k.CompleteZoneInfoMap(ctx))
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

func (k Keeper) QuasarDenomToNativeZoneIdMap(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyQuasarDenomToNativeZoneIdMap, &res)
	return
}

func (k Keeper) OsmosisDenomToQuasarDenomMap(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyOsmosisDenomToQuasarDenomMap, &res)
	return
}

// CompleteZoneInfoMap returns a map containing IBC routing info among all zones.
func (k Keeper) CompleteZoneInfoMap(ctx sdk.Context) (res map[string]types.ZoneCompleteInfo) {
	k.paramstore.Get(ctx, types.KeyCompleteZoneInfoMap, &res)
	return
}
