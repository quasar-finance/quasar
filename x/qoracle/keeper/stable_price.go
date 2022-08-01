package keeper

import (
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// SetStablePrice set the stable price for the symbol
func (k Keeper) SetStablePrice(ctx sdk.Context, symbol string, price sdk.Dec) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyStablePricesPrefix)

	key := []byte(symbol)
	b, err := price.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(key, b)
}

// GetStablePrice get the stable denom for the symbol
func (k Keeper) GetStablePrice(ctx sdk.Context, symbol string) (price sdk.Dec, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyStablePricesPrefix)
	key := []byte(symbol)
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

// GetRelativeStablePrice calculates how many denomOut is equivalent to one denomIn.
func (k Keeper) GetRelativeStablePrice(ctx sdk.Context, denomIn, denomOut string) (sdk.Dec, error) {
	priceIn, found := k.GetStablePrice(ctx, denomIn)
	if !found {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrStablePriceNotFound, "stable price not found for denom %s", denomIn)
	}
	priceOut, found := k.GetStablePrice(ctx, denomOut)
	if !found {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrStablePriceNotFound, "stable price not found for denom %s", denomOut)
	}
	if priceOut.IsZero() {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrZeroStablePrice, "zero stable price for denom %s", denomOut)
	}
	return priceIn.Quo(priceOut), nil
}
