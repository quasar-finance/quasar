package keeper

import (
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// OnDeposit called from the qbank module callback mechanism
func (k Keeper) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	k.Logger(ctx).Info("OnDeposit", "VaultID", vaultID, "Coin", coin)

	// Packet forwarding to osmosis to be done here.
	if k.Enabled(ctx) && vaultID == types.ModuleName {
		k.IBCTokenTransfer(ctx, coin)
	} else {
		k.Logger(ctx).Info("OnDeposit not calling IBC token transfer", "Enabled", k.Enabled(ctx), "VaultID", vaultID, "Coin", coin)
		// Shall we refund the deposited fund
	}
}
