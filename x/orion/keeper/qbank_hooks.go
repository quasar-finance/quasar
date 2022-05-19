package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// OnDeposit called from the qbank module callback mechanism
func (k Keeper) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	k.Logger(ctx).Info("OnDeposit", "VaultID", vaultID, "Coin", coin)

	// Packet forwarding to osmosis to be done here.
}
