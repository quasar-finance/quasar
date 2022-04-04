package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolRanking set poolRanking in the store
func (k Keeper) SetPoolRanking(ctx sdk.Context, poolRanking types.PoolRanking) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolRankingKey))
	b := k.cdc.MustMarshal(&poolRanking)
	store.Set([]byte{0}, b)
}

// GetPoolRanking returns poolRanking
func (k Keeper) GetPoolRanking(ctx sdk.Context) (val types.PoolRanking, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolRankingKey))

	b := store.Get([]byte{0})
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemovePoolRanking removes poolRanking from the store
func (k Keeper) RemovePoolRanking(ctx sdk.Context) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolRankingKey))
	store.Delete([]byte{0})
}
