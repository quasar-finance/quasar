package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
)

////////////////////////////////////////////////////////////////
/////////////////// REWARD ACCOUNT /////////////////////////////
////////////////////////////////////////////////////////////////

// CreateOrionGlobalRewardMacc returns orion global reward account address
func (k Keeper) CreateOrionGlobalRewardMacc() sdk.AccAddress {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.accountKeeper.GetModuleAddress(accName)
}

// SendCoinFromAccountToGlobalReward transfer balance from account to global reward account
func (k Keeper) SendCoinFromAccountToGlobalReward(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToGlobalReward transfer balance from sender module to global reward account
func (k Keeper) SendCoinFromModuleToGlobalReward(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, sdk.NewCoins(amt))
}

// SendCoinFromGlobalRewardToAccount transfer balance from account to userAcc
func (k Keeper) SendCoinFromGlobalRewardToAccount(ctx sdk.Context, userAcc sdk.AccAddress, amt sdk.Coins) error {
	accName := types.CreateOrionRewardGloablMaccName()
	return k.BankKeeper.SendCoinsFromModuleToAccount(ctx, accName, userAcc, amt)
}

// GetAllGlobalRewardBalances retrieve the total reward balance
func (k Keeper) GetAllGlobalRewardBalances(ctx sdk.Context) sdk.Coins {
	accAddr := k.CreateOrionGlobalRewardMacc()
	balances := k.BankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetGlobalRewardBalance retrieves the total denom reward balance
func (k Keeper) GetGlobalRewardBalance(ctx sdk.Context, denom string) sdk.Coin {
	accAddr := k.CreateOrionGlobalRewardMacc()
	balance := k.BankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

////////////////////////////////////////////////////////////////
/////////////////// ORION MODULE ACCOUNT ACCOUNT ///////////////
////////////////////////////////////////////////////////////////

// GetOrionAcc returns the module account bech32 with which ICA transactions to be done
func (k Keeper) GetOrionAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.ModuleName)
}

// GetOrionAccStr returns the module account bech32 with which ICA transactions to be done
func (k Keeper) GetOrionAccStr() string {
	accAddr := k.accountKeeper.GetModuleAddress(types.ModuleName)
	accStr, err := sdk.Bech32ifyAddressBytes("quasar", accAddr)
	if err != nil {
		panic(err)
	}
	return accStr
}

// GetOrionAccBalances retrieve the balance of orion module account
func (k Keeper) GetOrionAccBalances(ctx sdk.Context) sdk.Coins {
	balances := k.BankKeeper.GetAllBalances(ctx, k.GetOrionAcc())
	return balances
}

////////////////////////////////////////////////////////////////
/////////////////// RESERVE ACCOUNT ACCOUNT ////////////////////
////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////
/////////////////// Vault Level staking accounts ///////////////////
////////////////////////////////////////////////////////////////

// CreateOrionStakingMacc create acc for orion staking based on lockup period.
func (k Keeper) CreateOrionStakingMacc(lockupPeriod qbanktypes.LockupTypes) sdk.AccAddress {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	acc := k.accountKeeper.GetModuleAddress(accName)
	return acc
}

// GetAllStakingBalances retrieve the stake balance (users deposits) held by Orion vault lockup based staking accounts
func (k Keeper) GetAllStakingBalances(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) sdk.Coins {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balances := k.BankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetStakingBalance retrive the denom balance held by orion vault lockup account.
func (k Keeper) GetStakingBalance(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Coin {
	accAddr := k.CreateOrionStakingMacc(lockupPeriod)
	balance := k.BankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// SendCoinFromAccountToStaking transfer balance from user account to vault lockup staking account
func (k Keeper) SendCoinFromAccountToStaking(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin, lockupPeriod qbanktypes.LockupTypes) error {
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	return k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
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
	balances := k.BankKeeper.GetAllBalances(ctx, accAddr)
	return balances
}

// GetMeissaBalance retrive the denom amount held by meissa global account.
func (k Keeper) GetMeissaBalance(ctx sdk.Context, denom string) sdk.Coin {
	accAddr := k.CreateMeissaGlobalMacc()
	balance := k.BankKeeper.GetBalance(ctx, accAddr, denom)
	return balance
}

// SendCoinFromAccountToMeissa transefer amount from user account to meissa global account
func (k Keeper) SendCoinFromAccountToMeissa(ctx sdk.Context, senderAddr sdk.AccAddress, amt sdk.Coin) error {
	accName := types.CreateMeissaMaccName()
	return k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, accName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToMeissa transefer amount from senderModulet to meissa global account
func (k Keeper) SendCoinFromModuleToMeissa(ctx sdk.Context, senderModule string, amt sdk.Coin) error {
	accName := types.CreateMeissaMaccName()
	return k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, sdk.NewCoins(amt))
}

// SendCoinFromModuleToMeissa transefer amount from senderModulet to meissa global account
func (k Keeper) SendCoinsFromModuleToMeissa(ctx sdk.Context, senderModule string, amts sdk.Coins) error {
	accName := types.CreateMeissaMaccName()
	return k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderModule, accName, amts)
}
