package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// updateStablePrices sets the price of unit of coins request from bandchain oracle based on the latest CoinRatesState.
func (k Keeper) updateStablePrices(ctx sdk.Context) {
	state := k.GetCoinRatesState(ctx)
	var callData types.CoinRatesCallData
	k.cdc.MustUnmarshal(state.CallData.Value, &callData)
	var result types.CoinRatesResult
	k.cdc.MustUnmarshal(state.Result.Value, &result)

	symbolsWithMul := k.BandchainParams(ctx).CoinRatesParams.SymbolsWithMul.Sort()
	if len(symbolsWithMul) != len(callData.Symbols) {
		k.Logger(ctx).Error("Failed to update stable prices because params symbols length is not equal to call data symbols length")
		return
	}
	for i, symbol := range callData.Symbols {
		mul := symbolsWithMul.AmountOf(symbol)
		if mul.IsZero() {
			k.Logger(ctx).Error("Failed to update stable prices because couldn't find multiplier for symbol %s in params", symbol)
			return
		}

		price := sdk.NewDec(int64(result.Rates[i])).QuoInt64(int64(callData.Multiplier)).Mul(mul)
		k.SetStablePrice(ctx, symbol, price)
	}
}

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
