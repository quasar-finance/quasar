package keeper

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
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
		prices[i] = sdk.NewDecCoinFromDec(symbol, p)
	}

	k.setPriceList(ctx, sdk.NewDecCoins(prices...))

	k.qoracleKeeper.NotifySymbolPricesUpdate(ctx)
}

// GetSymbolPriceList implements qoracle PriceOracle interface
func (k Keeper) GetSymbolPriceList(ctx sdk.Context) (qoracletypes.SymbolPriceList, error) {
	if !k.IsEnabled(ctx) {
		return qoracletypes.SymbolPriceList{}, types.ErrDisabled
	}

	return k.GetPriceList(ctx), nil
}

// GetPriceList get the price list from store
func (k Keeper) GetPriceList(ctx sdk.Context) (pl qoracletypes.SymbolPriceList) {
	b := ctx.KVStore(k.storeKey).Get(types.KeyPriceList)
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

	pl := qoracletypes.SymbolPriceList{
		Prices:    prices,
		UpdatedAt: ctx.BlockTime(),
	}
	ctx.KVStore(k.storeKey).Set(types.KeyPriceList, k.cdc.MustMarshal(&pl))
}
