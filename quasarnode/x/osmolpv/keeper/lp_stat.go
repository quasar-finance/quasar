package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetLpStat set lpStat in the store with given epochday and types.LpStat
func (k Keeper) SetLpStat(ctx sdk.Context, epochday uint64, lpStat types.LpStat) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	b := k.cdc.MustMarshal(&lpStat)
	store.Set(byteKey, b)
}

// GetLpStat returns lpStat of a given epochday
func (k Keeper) GetLpStat(ctx sdk.Context, epochday uint64) (val types.LpStat, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	b := store.Get(byteKey)
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveLpStat removes lpStat from the store of a given epochday
func (k Keeper) RemoveLpStat(ctx sdk.Context, epochday uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	store.Delete(byteKey)
}
