package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolInfo set a specific poolInfo in the store from its index
func (k Keeper) SetPoolInfo(ctx sdk.Context, poolInfo types.PoolInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolInfoKeyPrefix))
	b := k.cdc.MustMarshal(&poolInfo)
	store.Set(types.PoolInfoKey(
		poolInfo.PoolId,
	), b)
}

// GetPoolInfo returns a poolInfo from its index
func (k Keeper) GetPoolInfo(
	ctx sdk.Context,
	poolId string,

) (val types.PoolInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolInfoKeyPrefix))

	b := store.Get(types.PoolInfoKey(
		poolId,
	))
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemovePoolInfo removes a poolInfo from the store
func (k Keeper) RemovePoolInfo(
	ctx sdk.Context,
	poolId string,

) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolInfoKeyPrefix))
	store.Delete(types.PoolInfoKey(
		poolId,
	))
}

// GetAllPoolInfo returns all poolInfo
func (k Keeper) GetAllPoolInfo(ctx sdk.Context) (list []types.PoolInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolInfoKeyPrefix))
	iterator := sdk.KVStorePrefixIterator(store, []byte{})

	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		var val types.PoolInfo
		k.cdc.MustUnmarshal(iterator.Value(), &val)
		list = append(list, val)
	}

	return
}
