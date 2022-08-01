package keeper

import (
	epochstypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Hooks wrapper struct for incentives keeper.
type EpochHooks struct {
	k Keeper
}

// Hooks wrapper struct for  orion keeper.
type Hooks struct {
	k Keeper
}

var _ epochstypes.EpochHooks = EpochHooks{}
var _ qbanktypes.DepositHooks = Hooks{}

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

// qbank deposit hooks
func (h Hooks) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	h.k.OnDeposit(ctx, vaultID, coin)
}
