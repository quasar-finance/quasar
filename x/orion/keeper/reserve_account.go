package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetReserveAccAddress retrieve the account address of orion reserve.
// This is also the orion module treasury address
func (k Keeper) GetReserveAccAddress() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.OrionReserveMaccName)
}

// GetBech32ReserveAccAddress returns bech32 address of orion reserve account
func (k Keeper) GetBech32ReserveAccAddress() string {
	accStr, err := sdk.Bech32ifyAddressBytes("quasar", k.GetReserveAccAddress())
	if err != nil {
		panic(err)
	}
	return accStr
}

// GetAllReserveBalances retrieve the balance of orion vault reserve as a slice of
// sdk.Coin as sdk.Coins
func (k Keeper) GetAllReserveBalances(ctx sdk.Context) sdk.Coins {
	balances := k.BankKeeper.GetAllBalances(ctx, k.GetReserveAccAddress())
	return balances
}

// GetReserveBalance retrieve the orion vault denom balance.
func (k Keeper) GetReserveBalance(ctx sdk.Context, denom string) sdk.Coin {
	balance := k.BankKeeper.GetBalance(ctx, k.GetReserveAccAddress(), denom)
	return balance
}

// SendCoinFromAccountToReserve transfer balance from account to orion vault reserve
func (k Keeper) SendCoinFromAccountToReserve(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	return k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, types.OrionReserveMaccName, sdk.NewCoins(amt))
}

// SendCoinFromAccountToReserve transfer balance from account to orion vault reserve
func (k Keeper) SendCoinsFromAccountToReserve(ctx sdk.Context, senderAddr sdk.AccAddress, amts sdk.Coins) error {
	return k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, types.OrionReserveMaccName, amts)
}

// SendCoinFromModuleToReserve transfer balance from module to orion vault reserve
func (k Keeper) SendCoinFromModuleToReserve(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	return k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, types.OrionReserveMaccName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToReserve transfer balance from module to orion vault reserve
func (k Keeper) SendCoinsFromModuleToReserve(ctx sdk.Context, senderModule string, amts sdk.Coins) error {
	return k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, types.OrionReserveMaccName, amts)
}

///////////////// Module Account ////////////////

// getOwnerAccStr returns the module account bech32 with which ICA transactions to be done
func (k Keeper) GetOrionAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.ModuleName)
}

// getOwnerAccStr returns the module account bech32 with which ICA transactions to be done
func (k Keeper) GetOrionAccStr() string {
	// For initial testing use alice address -
	// TODO AUDIT here (which return?)
	//return "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec" // alice on quasar

	accAddr := k.accountKeeper.GetModuleAddress(types.ModuleName)
	accStr, err := sdk.Bech32ifyAddressBytes("quasar", accAddr)
	if err != nil {
		panic(err)
	}
	return accStr
}

// GetAllReserveBalances retrieve the balance of orion vault reserve as a slice of
// sdk.Coin as sdk.Coins
func (k Keeper) GetOrionAccBalances(ctx sdk.Context) sdk.Coins {
	balances := k.BankKeeper.GetAllBalances(ctx, k.GetOrionAcc())
	return balances
}
