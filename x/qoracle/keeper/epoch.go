package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	bandchainParams := k.BandchainParams(ctx)
	osmosisParams := k.OsmosisParams(ctx)

	if epochIdentifier == bandchainParams.CoinRatesParams.EpochIdentifier {
		k.TryUpdateCoinRates(ctx)
	}
	if epochIdentifier == osmosisParams.EpochIdentifier {
		k.TryUpdateOsmosisIncentivizedPools(ctx)
	}
}
