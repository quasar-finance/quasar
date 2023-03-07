package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	switch epochIdentifier {
	// TODO : Also call param request from osmosis
	case k.GetEpochIdentifier(ctx):
		k.TryUpdateIncentivizedPools(ctx)
		k.TryUpdateChainParams(ctx)
	}
}
