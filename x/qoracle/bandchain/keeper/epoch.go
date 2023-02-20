package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	switch epochIdentifier {
	case k.GetCoinRatesParams(ctx).EpochIdentifier:
		k.TryUpdateCoinRates(ctx)
	}
}
