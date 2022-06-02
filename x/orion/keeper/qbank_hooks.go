package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// EpochHooks wrapper struct
type QBankHooks struct {
	k Keeper
}

var _ qbanktypes.DepositHooks = QBankHooks{}

// Return the wrapper struct.
func (k Keeper) QBankHooks() QBankHooks {
	return QBankHooks{k}
}

func (h QBankHooks) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	h.k.OnDeposit(ctx, vaultID, coin)
}

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
