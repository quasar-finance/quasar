package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) CreateOrionRewardMacc(lockupPeriod qbanktypes.LockupTypes) sdk.AccAddress {
	accName := types.CreateOrionRewardMaccName(lockupPeriod)
	return k.accountKeeper.GetModuleAddress(accName)
}

// Retrieve the amount of stake as a slice of sdk.Coin as sdk.Coins
// held by Orion vault reward accounts
func (k Keeper) GetAllRewardBalances(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) sdk.Coins {
	accAddr := k.CreateOrionRewardMacc(lockupPeriod)
	balances := k.bankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// Retrive the amount of reserve per denomication held by osmoLPV vault.
func (k Keeper) GetrewardBalance(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Coin {
	accAddr := k.CreateOrionRewardMacc(lockupPeriod)
	balance := k.bankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

func (k Keeper) SendCoinFromAccountToreward(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionRewardMaccName(lockupPeriod)
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}
