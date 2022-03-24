package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// CreateOrionRewardMacc create orion vault lockup based reward account
func (k Keeper) CreateOrionRewardMacc(lockupPeriod qbanktypes.LockupTypes) sdk.AccAddress {
	accName := types.CreateOrionRewardMaccName(lockupPeriod)
	return k.accountKeeper.GetModuleAddress(accName)
}

// GetAllRewardBalances retrieve the reward balance as a slice of sdk.Coin as sdk.Coins
// held by Orion vault reward accounts
func (k Keeper) GetAllRewardBalances(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) sdk.Coins {
	accAddr := k.CreateOrionRewardMacc(lockupPeriod)
	balances := k.bankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetRewardBalance retrieve the denom balance held by osmoLPV lockup reward account.
func (k Keeper) GetRewardBalance(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Coin {
	accAddr := k.CreateOrionRewardMacc(lockupPeriod)
	balance := k.bankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// SendCoinFromAccountToreward transfer balance from account to lockup reward account
func (k Keeper) SendCoinFromAccountToReward(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionRewardMaccName(lockupPeriod)
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

// SendCoinFromAccountToreward transfer balance from module to lockup reward account
func (k Keeper) SendCoinFromModuleToReward(ctx sdk.Context, senderModule string, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionRewardMaccName(lockupPeriod)
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, sdk.NewCoins(amt))
}

// SendCoinFromAccountToGlobalReward transfer balance from account to lockup reward account
func (k Keeper) SendCoinFromAccountToGlobalReward(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

// AUDIT NOTE - As of now global reward module account is used for the account.
// And below two methods will be used in the module.
// SendCoinFromModuleToGlobalReward transfer balance from module to lockup reward account
func (k Keeper) SendCoinFromModuleToGlobalReward(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, sdk.NewCoins(amt))
}

// SendCoinFromGlobalRewardToAccount transfer balance from account to lockup reward account
func (k Keeper) SendCoinFromGlobalRewardToAccount(ctx sdk.Context, userAcc sdk.AccAddress, amt sdk.Coins) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.bankKeeper.SendCoinsFromModuleToAccount(ctx, accName, userAcc, amt)
}
