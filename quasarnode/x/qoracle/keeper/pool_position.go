package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolPosition set poolPosition in the store
func (k Keeper) SetPoolPosition(ctx sdk.Context, poolID uint64, poolPosition types.PoolPosition) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolPositionKBP)
	b := k.cdc.MustMarshal(&poolPosition)
	// store.Set([]byte{0}, b)
	key := types.CreatePoolPositionKey(poolID)
	store.Set(key, b)
}

// GetPoolPosition returns poolPosition
func (k Keeper) GetPoolPosition(ctx sdk.Context, poolID uint64) (val types.PoolPosition, found bool) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolPositionKBP)
	key := types.CreatePoolPositionKey(poolID)
	b := store.Get(key)
	// b := store.Get([]byte{0})
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemovePoolPosition removes poolPosition from the store
func (k Keeper) RemovePoolPosition(ctx sdk.Context, poolID uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolPositionKBP)
	key := types.CreatePoolPositionKey(poolID)
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	// store.Delete([]byte{0})
	store.Delete(key)
}
