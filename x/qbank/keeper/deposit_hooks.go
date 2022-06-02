package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	k.Logger(ctx).Info("OnDeposit", "VaultID", vaultID, "Coin", coin)

	if k.DepositHooks == nil {
		// This condition comes in the unit testing
		println("DepositHooks can not be nil")
		k.Logger(ctx).Info("OnDeposit DepositHooks is nil", "VaultID", vaultID, "Coin", coin)
		return
	}
	k.DepositHooks.OnDeposit(ctx, vaultID, coin)
}
