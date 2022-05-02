package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	// TODO get epoch identifier from params
	if epochIdentifier == "minute" {
		k.Logger(ctx).Info("epoch ended",
			"identifier", epochIdentifier,
			"number", epochNumber,
			"epochinfo", k.EpochsKeeper.GetEpochInfo(ctx, epochIdentifier))
	}
}
