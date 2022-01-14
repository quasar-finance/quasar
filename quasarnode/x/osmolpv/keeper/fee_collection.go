package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// There are four types of fee collectors to collect fees for each type of fee
// aka, vault management fee, vault performance fee, entry fee and exit fee.
// Fee collectors are implemented as module account facility from cosmos sdk x/auth module.

// Get the fee collector account address in sdk.AccAddress type from human readable name.
func (k Keeper) GetFeeCollectorAccAddress(feeCollectorName string) sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(feeCollectorName)
}

// Get the account balance of the inputed fee collector name.
func (k Keeper) GetFeeCollectorBalances(ctx sdk.Context, feeCollectorName string) sdk.Coins {
	balances := k.bankKeeper.GetAllBalances(ctx, k.GetFeeCollectorAccAddress(feeCollectorName))
	return balances
}

// Deduce fees of type based of feeCollector name from the investor address
// who deposited tokens in orion vault. There is one to one mapping between the type
// of fee with the fee collector name.
// If the feeCollectorName input is MgmtFeeCollectorMaccName then the fee collected is
// Management fee, and so for other types of fee.
func (k Keeper) DeductFees(ctx sdk.Context, senderAddr sdk.AccAddress,
	feeCollectorName string, fees sdk.Coins) error {

	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
	err := k.bankKeeper.SendCoinsFromAccountToModule(ctx, senderAddr, feeCollectorName, fees)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFunds, err.Error())
	}

	return nil
}

///////////////////// Calculation of Fees /////////////////////
// Calculate the management fee
func (k Keeper) CalcMgmtFee() sdk.Coin {
	// TODO -
	// To be calculated on pro rata basis at every epoch
	return sdk.NewCoin("test", sdk.ZeroInt())
}

// Calculate vault performance fee.
// This function is called by vault at the end of every profit collection
// round. This could be the end of 1-Week Gauge, 3-Week Gauge etc.
// return value will be deduced from the profit and allocated to the
// vault reserve.
func (k Keeper) CalcPerFee(profit sdk.Coin) sdk.Coin {
	// TODO - To be added in vault parameter
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.2")
	feeAmt := profit.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(profit.GetDenom(), feeAmt)
}

// Calculate the entry fee every time when a user deposit coins
// into vault. Return value will be deduced from the depositor account.
func (k Keeper) CalcEntryFee(depositAmt sdk.Coin) sdk.Coin {
	// TODO - Factor value to be added in parameter.
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.01")
	feeAmt := depositAmt.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(depositAmt.GetDenom(), feeAmt)
}

// Calculate the exit fee every time when a user withdwar coins
// into vault. Return value will be deduced from the depositor
// account, who is exiting his positions.
func (k Keeper) CalcExitFee(exitAmt sdk.Coin) sdk.Coin {
	// TODO - Factor value to be added in parameter.
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.01")
	feeAmt := exitAmt.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(exitAmt.GetDenom(), feeAmt)
}

/*
func (k Keeper) GetMgmtFeeCollectorAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.MgmtFeeCollectorMaccName)
}

func (k Keeper) GetPerfFeeCollectorAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.PerfFeeCollectorMaccName)
}

func (k Keeper) GetEntryFeeCollectorAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.EntryFeeCollectorMaccName)
}

func (k Keeper) GetExitFeeCollectorAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.ExitFeeCollectorMaccName)
}
*/

/*
// Deduce management fee from the depositor address
func (k Keeper) DeductMgmtFees(ctx sdk.Context, senderAddr sdk.AccAddress, fees sdk.Coin) error {
	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
}

// Deduce management fee from the depositor address
func (k Keeper) DeductPerFees(ctx sdk.Context, senderAddr sdk.AccAddress, fees sdk.Coin) error {
	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
}

// Deduce entry fee from the depositor address
func (k Keeper) DeductEntryFees(ctx sdk.Context, senderAddr sdk.AccAddress, fees sdk.Coin) error {
	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
}

// Deduce exit fee from the depositor address
func (k Keeper) DeductExitFees(ctx sdk.Context, senderAddr sdk.AccAddress, fees sdk.Coin) error {
	if !fees.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInsufficientFee, "invalid fee amount: %s", fees)
	}
}
*/
