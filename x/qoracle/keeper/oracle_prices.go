package keeper

import (
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// updateOraclePrices sets the price of symbols requested from bandchain oracle based on the latest CoinRatesState.
func (k Keeper) updateOraclePrices(ctx sdk.Context) {
	state := k.GetCoinRatesState(ctx)
	var callData types.CoinRatesCallDataI
	if err := k.cdc.UnpackAny(state.CallData, &callData); err != nil {
		panic(err)
	}
	var result types.CoinRatesResultI
	if err := k.cdc.UnpackAny(state.Result, &result); err != nil {
		panic(err)
	}

	op := types.OraclePrices{
		Prices:          make(sdk.DecCoins, len(callData.GetSymbols())),
		UpdatedAtHeight: ctx.BlockHeight(),
	}
	mul := callData.GetMultiplier()
	for i, symbol := range callData.GetSymbols() {
		price := sdk.NewDec(int64(result.GetRates()[i])).QuoInt64(int64(mul))
		op.Prices[i] = sdk.NewDecCoinFromDec(symbol, price)
	}

	k.setOraclePrices(ctx, op)
}

// setOraclePrices set the oracle prices
func (k Keeper) setOraclePrices(ctx sdk.Context, op types.OraclePrices) {
	// Always save oracle prices sorted
	op.Prices.Sort()

	ctx.KVStore(k.storeKey).Set(types.KeyOraclePrices, k.cdc.MustMarshal(&op))
}

// GetOraclePrice get the oracle prices from store
func (k Keeper) GetOraclePrices(ctx sdk.Context) (op types.OraclePrices) {
	b := ctx.KVStore(k.storeKey).Get(types.KeyOraclePrices)
	if b == nil {
		return
	}

	k.cdc.MustUnmarshal(b, &op)
	return op
}
