package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Retrieve the Account address of osmoLPV reserve
func (k Keeper) GetReserveAccAddress() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.OsmoLPVReserveMaccName)
}

// Retrieve the amount of reserve as a slice of sdk.Coin as sdk.Coins
// help by osmoLPV vault
func (k Keeper) GetAllReserveBalances(ctx sdk.Context, denom string) sdk.Coins {
	balances := k.bankKeeper.GetAllBalances(ctx, k.GetReserveAccAddress())
	return balances
}

// Retrive the amount of reserve per denomication help by osmoLPV vault.
func (k Keeper) GetReserveBalance(ctx sdk.Context, denom string) sdk.Coin {
	balance := k.bankKeeper.GetBalance(ctx, k.GetReserveAccAddress(), denom)
	return balance
}

func (k Keeper) SendCoinFromAccountToReserve(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, types.OsmoLPVReserveMaccName, sdk.NewCoins(amt))
}
