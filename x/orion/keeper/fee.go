package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// There are two types of fee collectors to collect fees for each type of fee:
// management fee, and vault performance fee.
// Fee collectors are implemented as module account facility from cosmos sdk x/auth module.
// Perf Fee = A set percentage to be taken from the reward collection.
// Mgmt Fee = A set percentage to be taken from the users deposit amount.

// GetFeeCollectorAccAddress gets the fee collector account address in sdk.AccAddress type from human-readable name.
func (k Keeper) GetFeeCollectorAccAddress(feeCollectorName string) sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(feeCollectorName)
}

// GetBech32FeeCollectorAccAddress returns bech32 address of orion fee collector account
func (k Keeper) GetBech32FeeCollectorAccAddress(feeCollectorName string) string {
	accStr, err := sdk.Bech32ifyAddressBytes("quasar", k.GetFeeCollectorAccAddress(feeCollectorName))
	if err != nil {
		panic(err)
	}
	return accStr
}

// GetFeeCollectorBalances gets the account balance of the inputted fee collector name.
func (k Keeper) GetFeeCollectorBalances(ctx sdk.Context, feeCollectorName string) sdk.Coins {
	balances := k.BankKeeper.GetAllBalances(ctx, k.GetFeeCollectorAccAddress(feeCollectorName))
	return balances
}

// DeductAccFees deduce fees of type based of feeCollector name from the investor address
// who deposited tokens in orion vault. There is one to one mapping between the type
// of fee with the fee collector name.  In this method, fee deduction is done from end user account managed
// by the Orion module. If the feeCollectorName input is MgmtFeeCollectorMaccName then the fee collected is
// Management fee, and so for other types of fee.
func (k Keeper) DeductAccFees(ctx sdk.Context, senderAddr sdk.AccAddress,
	feeCollectorName string, fees sdk.Coins) error {

	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
	err := k.BankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, feeCollectorName, fees)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFunds, err.Error())
	}

	return nil
}

// DeductFeesFromModuleAccount deducts the management fee from the module account
// before the rest is distributed to the user.
func (k Keeper) DeductFeesFromModuleAccount(ctx sdk.Context, senderAccName string,
	feeCollectorName string, fees sdk.Coins) error {

	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
	err := k.BankKeeper.SendCoinsFromModuleToModule(ctx, senderAccName, feeCollectorName, fees)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFunds, err.Error())
	}

	return nil
}

// DeductVaultFees deduce performance fees of type based of feeCollector name from the investor address
// who deposited tokens in orion vault. There is one to one mapping between the type
// of fee with the fee collector name. In this method, fee deduction is done from a module account managed
// by the Orion module. If the feeCollectorName input is PerfFeeCollectorMaccName then the fee collected is
// Performance fee, and so for other types of fee.
func (k Keeper) DeductVaultFees(ctx sdk.Context, sourceMacc string,
	feeCollectorName string, fees sdk.Coins) error {

	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
	err := k.BankKeeper.SendCoinsFromModuleToModule(ctx, sourceMacc, feeCollectorName, fees)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFunds, err.Error())
	}

	return nil
}

///////////////////// Calculation of Fees /////////////////////

// CalcPerFee is called by vault at the end of every profit collection round.
// CalcPerFee calculate vault performance fee.
func (k Keeper) CalcPerFee(ctx sdk.Context, profit sdk.Coin) sdk.Coin {
	factor := k.PerfFeePer(ctx)
	feeAmt := profit.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(profit.GetDenom(), feeAmt)
}

// CalculatePerformanceFeeForCoins is called by RewardDistribution.
// It calculates vault performance fee.
func (k Keeper) CalculatePerformanceFeeForCoins(ctx sdk.Context, profit sdk.Coins) sdk.Coins {
	factor := k.PerfFeePer(ctx)
	feeAmt, _ := sdk.NewDecCoinsFromCoins(profit...).MulDec(factor).TruncateDecimal()
	return feeAmt
}

// CalcMgmtFee Calculate the management fee.
func (k Keeper) CalcMgmtFee(ctx sdk.Context, coin sdk.Coin) sdk.Coin {
	factor := k.MgmtFeePer(ctx)
	feeAmt := coin.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(coin.GetDenom(), feeAmt)

}
