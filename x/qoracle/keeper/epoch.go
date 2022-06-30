package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	bandchainParams := k.BandchainParams(ctx)

	switch epochIdentifier {
	case bandchainParams.CoinRatesParams.EpochIdentifier:
		k.TryUpdateCoinRates(ctx)
	}
}
