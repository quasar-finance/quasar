package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetPoolSpotPrice set a specific poolSpotPrice in the store from its index
func (k Keeper) SetPoolSpotPrice(ctx sdk.Context, poolSpotPrice types.PoolSpotPrice) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolSpotPriceKeyPrefix))
	b := k.cdc.MustMarshal(&poolSpotPrice)
	store.Set(types.PoolSpotPriceKey(
		poolSpotPrice.PoolId,
		poolSpotPrice.DenomIn,
		poolSpotPrice.DenomOut,
	), b)
}

// GetPoolSpotPrice returns a poolSpotPrice from its index
func (k Keeper) GetPoolSpotPrice(
	ctx sdk.Context,
	poolId string,
	denomIn string,
	denomOut string,

) (val types.PoolSpotPrice, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolSpotPriceKeyPrefix))

	b := store.Get(types.PoolSpotPriceKey(
		poolId,
		denomIn,
		denomOut,
	))
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemovePoolSpotPrice removes a poolSpotPrice from the store
func (k Keeper) RemovePoolSpotPrice(
	ctx sdk.Context,
	poolId string,
	denomIn string,
	denomOut string,

) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolSpotPriceKeyPrefix))
	store.Delete(types.PoolSpotPriceKey(
		poolId,
		denomIn,
		denomOut,
	))
}

// GetAllPoolSpotPrice returns all poolSpotPrice
func (k Keeper) GetAllPoolSpotPrice(ctx sdk.Context) (list []types.PoolSpotPrice) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.PoolSpotPriceKeyPrefix))
	iterator := sdk.KVStorePrefixIterator(store, []byte{})

	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		var val types.PoolSpotPrice
		k.cdc.MustUnmarshal(iterator.Value(), &val)
		list = append(list, val)
	}

	return
}

// SetStablePrice set the stable price for the input denom
func (k Keeper) SetStablePrice(ctx sdk.Context, denom string, price sdk.Dec) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.StablePriceKBP)
	key := types.CreateStablePriceKey(denom)
	b, err := price.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(key, b)
}

// GetStablePrice get the stabe denom for the input denom
func (k Keeper) GetStablePrice(ctx sdk.Context, denom string) sdk.Dec {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.StablePriceKBP)
	key := types.CreateStablePriceKey(denom)
	b := store.Get(key)
	var price sdk.Dec
	err := (&price).Unmarshal(b)
	if err != nil {
		return sdk.ZeroDec()
	}
	return price
}
