package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetEpochLPInfo set epochLPInfo in the store.
// Latest/Current market value of the orions as lp tokens.
func (k Keeper) SetEpochLPInfo(ctx sdk.Context, epochLPInfo types.EpochLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLPInfoKBP)
	key := types.EpochDayKey(epochLPInfo.EpochDay)
	b := k.cdc.MustMarshal(&epochLPInfo)
	store.Set(key, b)
}

// GetEpochLPInfo returns epochLPInfo.
// Caller need to use this for the purpose of calculating equivalent orion receipts used in a given epochday.
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
