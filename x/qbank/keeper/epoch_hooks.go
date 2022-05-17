package keeper

import (
	epochstypes "github.com/abag/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Hooks wrapper struct for incentives keeper.
type EpochHooks struct {
	k Keeper
}

var _ epochstypes.EpochHooks = EpochHooks{}

// Return the wrapper struct.
func (k Keeper) EpochHooks() EpochHooks {
	return EpochHooks{k}
}

// epochs hooks
// Don't do anything pre epoch start.
func (h EpochHooks) BeforeEpochStart(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
}

func (h EpochHooks) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	h.k.AfterEpochEnd(ctx, epochIdentifier, epochNumber)
}
