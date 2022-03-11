package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetEpochLPInfo set epochLPInfo in the store.
// Caller should provide the updated value of epochLPInfo object.
func (k Keeper) SetEpochLPInfo(ctx sdk.Context, epochLPInfo types.EpochLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.CreateLPPositionEpochLPInfoKey(epochLPInfo.EpochDay)
	b := k.cdc.MustMarshal(&epochLPInfo)
	store.Set(key, b)
}

// GetEpochLPInfo returns epochLPInfo
func (k Keeper) GetEpochLPInfo(ctx sdk.Context, epochDay uint64) (val types.EpochLPInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.CreateLPPositionEpochLPInfoKey(epochDay)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveEpochLPInfo removes epochLPInfo from the store
func (k Keeper) RemoveEpochLPInfo(ctx sdk.Context, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.CreateLPPositionEpochLPInfoKey(epochDay)
	store.Delete(key)
}
