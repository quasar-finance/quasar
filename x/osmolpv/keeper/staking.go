package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

/////////////// Vault Level staking accounts ///////////////

// CreateOrionStakingMacc create acc for orion staking based on lockup period.
func (k Keeper) CreateOrionStakingMacc(lockupPeriod qbanktypes.LockupTypes) sdk.AccAddress {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	acc := k.accountKeeper.GetModuleAddress(accName)

	// accStr, err := sdk.Bech32ifyAddressBytes("quasar", acc)
	// if err != nil {
	// 	panic(err)
	// }
	// fmt.Printf("CreateOrionStakingMacc|accountName=%v \n", accName)
	// fmt.Printf("CreateOrionStakingMacc|accountAddress=%v\n", acc)
	// fmt.Printf("CreateOrionStakingMacc|accountAddress=%v|ERROR=%v\n", accStr, err)

	return acc
}

// GetAllStakingBalances retrieve the amount of stake as a slice of sdk.Coin as sdk.Coins
// held by an Orion vault lockup based staking accounts
func (k Keeper) GetAllStakingBalances(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) sdk.Coins {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balances := k.bankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetStakingBalance retrive the denom balance held by osmoLPV vault lockup account.
func (k Keeper) GetStakingBalance(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Coin {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balance := k.bankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// SendCoinFromAccountToStaking transfer balance from user account to vault lockup staking account
func (k Keeper) SendCoinFromAccountToStaking(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

/////////////// Vault Strategy Level staking accounts ///////////////
///////////////         Meissa strategy accounts      ///////////////

// CreateMeissaGlobalMacc create account meissa global account
func (k Keeper) CreateMeissaGlobalMacc() sdk.AccAddress {
	accName := types.CreateMeissaMaccName()
	return k.accountKeeper.GetModuleAddress(accName)
}

// GetAllMeissaBalances retrieve the amount of stake as a slice of sdk.Coin as sdk.Coins
// held by meissa strategy global accounts
func (k Keeper) GetAllMeissaBalances(ctx sdk.Context) sdk.Coins {
	accAddr := k.CreateMeissaGlobalMacc()
	balances := k.bankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetMeissaBalance retrive the denom amount held by meissa global account.
func (k Keeper) GetMeissaBalance(ctx sdk.Context, denom string) sdk.Coin {
	accAddr := k.CreateMeissaGlobalMacc()
	balance := k.bankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// SendCoinFromAccountToMeissa transefer amount from user account to meissa global account
func (k Keeper) SendCoinFromAccountToMeissa(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	accName := types.CreateMeissaMaccName()
	return k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToMeissa transefer amount from senderModulet to meissa global account
func (k Keeper) SendCoinFromModuleToMeissa(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	accName := types.CreateMeissaMaccName()
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToMeissa transefer amount from senderModulet to meissa global account
func (k Keeper) SendCoinsFromModuleToMeissa(ctx sdk.Context, senderModule string, amts sdk.Coins) error {
	accName := types.CreateMeissaMaccName()
	return k.bankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, amts)
}
