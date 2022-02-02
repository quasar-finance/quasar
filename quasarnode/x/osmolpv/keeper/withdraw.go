package keeper

import (
	// qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// @desc Function to do depositors withdrawal operations
// Logic - Try to withdraw from lowest lockup period, then try from the higher lockup periods.
// Iteratively collect all the withdrawable amounts from all lockup periods and transfer the
// amount to users account from the orion module.
// @return Return error if sufficient withdrawable amount is not yet ready.
// func (k Keeper) RequestWithdraw(ctx sdk.Context, withdraw qbanktypes.Withdraw) error {

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
