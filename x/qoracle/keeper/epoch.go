package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	bandchainParams := k.BandchainParams(ctx)
	osmosisParams := k.OsmosisParams(ctx)

	switch epochIdentifier {
	case bandchainParams.CoinRatesParams.EpochIdentifier:
		k.TryUpdateCoinRates(ctx)
	case osmosisParams.EpochIdentifier:
		k.TryUpdateOsmosisIncentivizedPools(ctx)
	}
}
