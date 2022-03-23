package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetEpochLPInfo set epochLPInfo in the store.
// Caller should provide the updated value of epochLPInfo object.
func (k Keeper) SetEpochLPInfo(ctx sdk.Context, epochLPInfo types.EpochLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLPInfoKBP)
	key := types.EpochDayKey(epochLPInfo.EpochDay)
	b := k.cdc.MustMarshal(&epochLPInfo)
	store.Set(key, b)
}

// GetEpochLPInfo returns epochLPInfo
func (k Keeper) GetEpochLPInfo(ctx sdk.Context, epochDay uint64) (val types.EpochLPInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLPInfoKBP)
	key := types.EpochDayKey(epochDay)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveEpochLPInfo removes epochLPInfo from the store
func (k Keeper) RemoveEpochLPInfo(ctx sdk.Context, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLPInfoKBP)
	key := types.EpochDayKey(epochDay)
	store.Delete(key)
}

// SetEpochDayInfo set epochLPInfo in the store.
// Caller should provide the correct value of EpochDayInfoKBP object.
func (k Keeper) SetEpochDayInfo(ctx sdk.Context, epochLPInfo types.EpochDayInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochDayInfoKBP)
	key := types.EpochDayKey(epochLPInfo.EpochDay)
	b := k.cdc.MustMarshal(&epochLPInfo)
	store.Set(key, b)
}

// GetEpochDayInfo returns epochLPInfo
func (k Keeper) GetEpochDayInfo(ctx sdk.Context, epochDay uint64) (val types.EpochDayInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochDayInfoKBP)
	key := types.EpochDayKey(epochDay)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}
