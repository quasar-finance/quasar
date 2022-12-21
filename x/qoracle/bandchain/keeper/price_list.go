package keeper

import (
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
)

// updatePriceList sets the price of symbols requested from bandchain oracle based on the latest CoinRatesState.
func (k Keeper) updatePriceList(ctx sdk.Context) {
	state := k.GetCoinRatesState(ctx)
	var callData types.CoinRatesCallDataI
	if err := k.cdc.UnpackAny(state.CallData, &callData); err != nil {
		panic(err)
	}
	var result types.CoinRatesResultI
	if err := k.cdc.UnpackAny(state.Result, &result); err != nil {
		panic(err)
	}

	prices := make([]sdk.DecCoin, len(callData.GetSymbols()))
	mul := callData.GetMultiplier()
	for i, symbol := range callData.GetSymbols() {
		p := sdk.NewDec(int64(result.GetRates()[i])).QuoInt64(int64(mul))
		prices = append(prices, sdk.NewDecCoinFromDec(symbol, p))
	}

	k.setPriceList(ctx, sdk.NewDecCoins(prices...))
}

// GetPrices implements qoracle PriceOracle interface
func (k Keeper) GetPrices(ctx sdk.Context) (sdk.DecCoins, error) {
	if !k.IsEnabled(ctx) {
		return nil, types.ErrDisabled
	}

	pl := k.GetPriceList(ctx)
	// Check if the list is outdated
	if pl.UpdatedAtTime.Add(time.Duration(k.GetPriceListExpDuration(ctx))).Before(ctx.BlockTime()) {
		return nil, types.ErrPriceListOutdated
	}

	return pl.Prices, nil
}

// GetPriceList get the price list from store
func (k Keeper) GetPriceList(ctx sdk.Context) (pl types.PriceList) {
	b := ctx.KVStore(k.storeKey).Get(types.PriceListKey)
	if b == nil {
		return
	}

	k.cdc.MustUnmarshal(b, &pl)
	return pl
}

// setPriceList stores it in key-value store.
func (k Keeper) setPriceList(ctx sdk.Context, prices sdk.DecCoins) {
	if err := prices.Validate(); err != nil {
		panic(fmt.Errorf("invalid dec coins as prices: %w", err))
	}

	pl := types.PriceList{
		Prices:          prices,
		UpdatedAtHeight: ctx.BlockHeight(),
		UpdatedAtTime:   ctx.BlockTime(),
	}
	ctx.KVStore(k.storeKey).Set(types.PriceListKey, k.cdc.MustMarshal(&pl))
}
