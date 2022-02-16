package keeper

import (
	gammbalancertypes "github.com/abag/quasarnode/x/gamm/pool-models/balancer"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolPosition set poolPosition in the store
func (k Keeper) SetPoolInfo(ctx sdk.Context, poolID uint64, poolInfo gammbalancertypes.BalancerPool) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolInfoKBP)

	// TODO - Verify if you need to do local copy of poolInfo as there are pointers and slices inside
	b := k.cdc.MustMarshal(&poolInfo)
	// b, _ := poolInfo.Marshal()

	// store.Set([]byte{0}, b)
	key := types.CreatePoolInfoKey(poolID)
	store.Set(key, b)
}

// GetPoolPosition returns poolPosition
func (k Keeper) GetPoolInfo(ctx sdk.Context, poolID uint64) (poolInfo gammbalancertypes.BalancerPool, found bool) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolInfoKBP)
	key := types.CreatePoolInfoKey(poolID)
	b := store.Get(key)
	// b := store.Get([]byte{0})
	if b == nil {
		return poolInfo, false
	}
	// TODO - Verify if you need to do local copy of poolInfo as there are pointers and slices inside

	k.cdc.MustUnmarshal(b, &poolInfo)
	//val.Unmarshal()
	return poolInfo, true
}

// RemovePoolPosition removes poolPosition from the store
func (k Keeper) RemovePoolInfo(ctx sdk.Context, poolID uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolInfoKBP)
	key := types.CreatePoolInfoKey(poolID)
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	// store.Delete([]byte{0})
	store.Delete(key)
}

// KV STORE FOR APY RANKED POOL

// SetPoolPosition set poolPosition in the store
func (k Keeper) SetAPYRankedPool(ctx sdk.Context, pools types.SortedPools) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolAPYRankedKBP)

	// TODO - Verify if you need to do local copy of pools as it is a slice inside.
	b := k.cdc.MustMarshal(&pools)
	// store.Set([]byte{0}, b)
	key := types.CreateAPYRankedKey()
	store.Set(key, b)
}

// GetPoolPosition returns poolPosition
func (k Keeper) GetAPYRankedPoolInfo(ctx sdk.Context) (pools types.SortedPools, found bool) {
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolAPYRankedKBP)
	key := types.CreateAPYRankedKey()
	b := store.Get(key)
	// b := store.Get([]byte{0})
	if b == nil {
		return pools, false
	}

	// TODO - Verify if you need to do local copy of pools as it is a slice inside.
	k.cdc.MustUnmarshal(b, &pools)
	//val.Unmarshal()
	return pools, true
}

/*
// RemovePoolPosition removes poolPosition from the store
func (k Keeper) RemovePoolInfo(ctx sdk.Context, poolID uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.PoolInfoKBP)
	key := types.CreatePoolInfoKey(poolID)
	// store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKey))
	// store.Delete([]byte{0})
	store.Delete(key)
}
*/
