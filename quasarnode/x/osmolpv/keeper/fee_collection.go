package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// There are four types of fee collector to collect fees for each type of fees
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
