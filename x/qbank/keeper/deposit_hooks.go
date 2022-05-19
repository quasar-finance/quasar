package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) OnDeposit(ctx sdk.Context, vaultID string, coin sdk.Coin) {
	k.Logger(ctx).Info("OnDeposit", "VaultID", vaultID, "Coin", coin)

	if k.DepositHooks == nil {
		println("DepositHooks can not be nil")
	}
	k.DepositHooks.OnDeposit(ctx, vaultID, coin)
}
