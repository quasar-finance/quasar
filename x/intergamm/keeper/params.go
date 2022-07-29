package keeper

import (
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(k.OsmoTokenTransferChannels(ctx),
		k.DestToIntrZoneMap(ctx),
		k.IntrRcvrs(ctx))
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// OsmoTokenTransferChannels returns the  other chains token transfer channel to osmosis
func (k Keeper) OsmoTokenTransferChannels(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyOsmoTokenTransferChannels, &res)
	return
}

func (k Keeper) DestToIntrZoneMap(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyDestToIntrZoneMap, &res)
	return
}

// IntrRcvrs returns the intermediate receiver info for packet forwarding.
func (k Keeper) IntrRcvrs(ctx sdk.Context) (res []types.IntermediateReceiver) {
	k.paramstore.Get(ctx, types.KeyIntrRcvrs, &res)
	return
}
