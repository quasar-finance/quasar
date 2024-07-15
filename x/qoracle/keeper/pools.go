package keeper

import (
	"sort"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetPoolsRankedByAPY returns a list of all pools with ordered by APY in descending order with an optional denom filter.
// If denom is empty the function will return all pools otherwise it only returns pools that have denom as their deposited asset.
func (k Keeper) GetPoolsRankedByAPY(ctx sdk.Context, denom string) []types.Pool {
	memStore := ctx.KVStore(k.memKey)
	poolStore := prefix.NewStore(memStore, types.KeyMemPoolPrefix)
	osmosisPoolStore := prefix.NewStore(poolStore, types.KeyOsmosisPoolPrefix)

	iter := osmosisPoolStore.Iterator(nil, nil)
	defer iter.Close()
	var pools types.PoolsOrderedByAPY
	for ; iter.Valid(); iter.Next() {
		var pool types.Pool
		k.cdc.MustUnmarshal(iter.Value(), &pool)

		if denom != "" && !IsDenomInPool(pool, denom) {
			// Skip to next pool
			continue
		}
		// If denom is empty or denom is in pool assets
		pools = append(pools, pool)
	}

	// Order by APY in descending order
	sort.Stable(sort.Reverse(pools))
	return pools
}

func IsDenomInPool(pool types.Pool, denom string) bool {

	if denom != "" {
		for _, c := range pool.Assets {
			if c.Denom == denom {
				// If found ; return true
				return true
			}
		}
		/*
			    // Find exist in furture version
				if found, _ := pool.Assets.Find(denom); found {
					return false
				}

		*/
	}

	return false
}

// GetPool returns a pool for the given id if exists.
func (k Keeper) GetPool(ctx sdk.Context, id string) (types.Pool, bool) {
	memStore := ctx.KVStore(k.memKey)
	poolStore := prefix.NewStore(memStore, types.KeyMemPoolPrefix)
	osmosisPoolStore := prefix.NewStore(poolStore, types.KeyOsmosisPoolPrefix)

	key := []byte(id)
	if !osmosisPoolStore.Has(key) {
		return types.Pool{}, false
	}

	var pool types.Pool
	k.cdc.MustUnmarshal(memStore.Get(key), &pool)
	return pool, true
}
