package types

var (
	// CoinRatesSymbols is the list of coin rates symbols
	CoinRatesSymbols = []string{
		"BTC",
		"ETH",
		"XRP",
		"ATOM",
	}

	// CoinRatesMultiplier is the default multiplier used for coin rates oracle requests
	CoinRatesMultiplier = 1e8
)
