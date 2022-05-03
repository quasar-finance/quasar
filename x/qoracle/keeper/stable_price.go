package keeper

import (
	"errors"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

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

// GetStablePrice get the stable denom for the input denom
func (k Keeper) GetStablePrice(ctx sdk.Context, denom string) (price sdk.Dec, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.StablePriceKBP)
	key := types.CreateStablePriceKey(denom)
	b := store.Get(key)
	if b == nil {
		return price, false
	}
	err := (&price).Unmarshal(b)
	if err != nil {
		return price, false
	}
	return price, true
}

func (k Keeper) GetRelativeStablePrice(ctx sdk.Context, denomIn, denomOut string) (sdk.Dec, error) {
	priceIn, found := k.GetStablePrice(ctx, denomIn)
	if !found {
		return sdk.Dec{}, errors.New("error: stable price not found for " + denomIn)
	}
	priceOut, found := k.GetStablePrice(ctx, denomOut)
	if !found {
		return sdk.Dec{}, errors.New("error: stable price not found for " + denomOut)
	}
	if priceOut.IsZero() {
		return sdk.Dec{}, errors.New("error: stable price for denomOut may not be zero")
	}
	return priceIn.Quo(priceOut), nil
}
