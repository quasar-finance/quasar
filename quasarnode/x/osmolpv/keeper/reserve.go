package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetReserveAccAddress retrieve the account address of osmoLPV reserve.
// This is also the orion module treasury address
func (k Keeper) GetReserveAccAddress() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.OsmoLPVReserveMaccName)
}

// GetAllReserveBalances retrieve the balance of osmoLPV vault reserve as a slice of
// sdk.Coin as sdk.Coins
func (k Keeper) GetAllReserveBalances(ctx sdk.Context, denom string) sdk.Coins {
	balances := k.bankKeeper.GetAllBalances(ctx, k.GetReserveAccAddress())
	return balances
}

// GetReserveBalance retrieve the osmoLPV vault denom balance.
func (k Keeper) GetReserveBalance(ctx sdk.Context, denom string) sdk.Coin {
	balance := k.bankKeeper.GetBalance(ctx, k.GetReserveAccAddress(), denom)
	return balance
}

// SendCoinFromAccountToReserve transfer balance from account to osmoLPV vault reserve
func (k Keeper) SendCoinFromAccountToReserve(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, types.OsmoLPVReserveMaccName, sdk.NewCoins(amt))
}

// SendCoinFromAccountToReserve transfer balance from account to osmoLPV vault reserve
func (k Keeper) SendCoinsFromAccountToReserve(ctx sdk.Context, senderAddr sdk.AccAddress, amts sdk.Coins) error {
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, types.OsmoLPVReserveMaccName, amts)
}

// SendCoinFromModuleToReserve transfer balance from module to osmoLPV vault reserve
func (k Keeper) SendCoinFromModuleToReserve(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, types.OsmoLPVReserveMaccName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToReserve transfer balance from module to osmoLPV vault reserve
func (k Keeper) SendCoinsFromModuleToReserve(ctx sdk.Context, senderModule string, amts sdk.Coins) error {
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, types.OsmoLPVReserveMaccName, amts)
}
