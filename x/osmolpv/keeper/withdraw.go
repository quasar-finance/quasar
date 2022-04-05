package keeper

import (
	// qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestWithdraw  handles the depositors withdrawal request
// Logic - Try to withdraw from lowest lockup period, then try from the higher lockup periods.
// Iteratively collect all the withdrawable amounts from all lockup account and transfer the
// amount from the orion module to users account
// Return error if sufficient withdrawable amount is not yet ready.
// func (k Keeper) RequestWithdraw(ctx sdk.Context, withdraw qbanktypes.Withdraw) error {

// AUDIT NOTE | Redundant
//This function maybe removed. Orion module should update the withdrable amount based on the
// the strategy performance and exit position.
func (k Keeper) RequestWithdraw(ctx sdk.Context, depositorAddr string, coin sdk.Coin) error {
	/*
		depositorAddr, err := sdk.AccAddressFromBech32(withdraw.DepositorAccAddress)
		if err != nil {
			return err
		}
	*/

	// TODO - ITERATIVE LOGIC

	/*
		if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
			types.ModuleName,
			depositorAddr,
			sdk.NewCoins(withdraw.GetCoin())); err != nil {
			return err
		}
	*/
	return nil
}
