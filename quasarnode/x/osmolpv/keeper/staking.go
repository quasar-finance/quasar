package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

/////////////// Vault Level staking accounts ///////////////

// Orion vault staking acc name creation based on lockup period.
func (k Keeper) CreateOrionStakingMacc(lockupPeriod qbanktypes.LockupTypes) sdk.AccAddress {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	return k.accountKeeper.GetModuleAddress(accName)
}

// Retrieve the amount of stake as a slice of sdk.Coin as sdk.Coins
// held by Orion vault staking accounts
func (k Keeper) GetAllStakingBalances(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) sdk.Coins {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balances := k.bankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// Retrive the amount of reserve per denomication held by osmoLPV vault.
func (k Keeper) GetStakingBalance(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Coin {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balance := k.bankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// Balance transefer function, from user account to vault staking account
func (k Keeper) SendCoinFromAccountToStaking(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

/////////////// Vault Strategy Level staking accounts ///////////////
///////////////         Meissa strategy accounts      ///////////////
