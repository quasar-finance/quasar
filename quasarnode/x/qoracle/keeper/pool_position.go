package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolPosition set a specific poolPosition in the store from its index
func (k Keeper) SetPoolPosition(ctx sdk.Context, poolPosition types.PoolPosition) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKeyPrefix))
	b := k.cdc.MustMarshal(&poolPosition)
	store.Set(types.PoolPositionKey(
		poolPosition.PoolId,
	), b)
}

// GetPoolPosition returns a poolPosition from its index
func (k Keeper) GetPoolPosition(
	ctx sdk.Context,
	poolId string,

) (val types.PoolPosition, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKeyPrefix))

	b := store.Get(types.PoolPositionKey(
		poolId,
	))
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemovePoolPosition removes a poolPosition from the store
func (k Keeper) RemovePoolPosition(
	ctx sdk.Context,
	poolId string,

) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKeyPrefix))
	store.Delete(types.PoolPositionKey(
		poolId,
	))
}

// GetAllPoolPosition returns all poolPosition
func (k Keeper) GetAllPoolPosition(ctx sdk.Context) (list []types.PoolPosition) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolPositionKeyPrefix))
	iterator := sdk.KVStorePrefixIterator(store, []byte{})

	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		var val types.PoolPosition
		k.cdc.MustUnmarshal(iterator.Value(), &val)
		list = append(list, val)
	}

	return
}
