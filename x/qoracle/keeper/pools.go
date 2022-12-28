package keeper

import (
	"sort"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetPoolsRankedByAPY returns a list of all pools with ordered by APY in descending order with an optional denom filter.
// If denom is empty the function will return all pools otherwise it only returns pools that have denom as their deposited asset.
func (k Keeper) GetPoolsRankedByAPY(ctx sdk.Context, source, denom string) []types.Pool {
	memStore := ctx.KVStore(k.memKey)

	iter := sdk.KVStorePrefixIterator(memStore, types.KeyMemPoolPrefix)
	defer iter.Close()
	var pools types.PoolsOrderedByAPY
	for ; iter.Valid(); iter.Next() {
		var pool types.Pool
		k.cdc.MustUnmarshal(iter.Value(), &pool)

		if filterPool(pool, source, denom) {
			continue
		}

		pools = append(pools, pool)
	}

	// Order by APY in descending order
	sort.Stable(sort.Reverse(pools))
	return pools
}

func filterPool(pool types.Pool, source, denom string) bool {
	if source != "" && source != pool.Source {
		return true
	}

	// Filter out pools with the desired denom as asset
	if denom != "" {
		if found, _ := pool.Assets.Find(denom); found {
			return false
		}
	}

	return true
}

// GetPool returns a pool for the given source and id if exists.
func (k Keeper) GetPool(ctx sdk.Context, source, id string) (types.Pool, bool) {
	memStore := ctx.KVStore(k.memKey)

	key := types.GetPoolKey(source, id)
	if !memStore.Has(key) {
		return types.Pool{}, false
	}

	var pool types.Pool
	k.cdc.MustUnmarshal(memStore.Get(key), &pool)
	return pool, true
}
