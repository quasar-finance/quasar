package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, _ int64) {
	if k.GetEpochIdentifier(ctx) == epochIdentifier {
		k.TryUpdateIncentivizedPools(ctx)
		k.TryUpdateChainParams(ctx)
	}
}
