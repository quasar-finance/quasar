package types

import sdk "github.com/cosmos/cosmos-sdk/types"

var (
	// CoinRatesMultiplier is the default multiplier used for coin rates oracle requests
	CoinRatesMultiplier uint64 = 1
)

// NewCoinRatesCallDataFromDecCoins creates a new CoinRatesCallData with coins symbols and default multiplier.
func NewCoinRatesCallDataFromDecCoins(coins sdk.DecCoins) CoinRatesCallData {
	symbols := make([]string, len(coins))
	for i, coin := range coins {
		symbols[i] = coin.GetDenom()
	}

	return CoinRatesCallData{
		Symbols:    symbols,
		Multiplier: CoinRatesMultiplier,
	}
}
