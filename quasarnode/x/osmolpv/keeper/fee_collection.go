package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// There are four types of fee collectors to collect fees for each type of fee
// aka, vault management fee, vault performance fee, entry fee and exit fee.
// Fee collectors are implemented as module account facility from cosmos sdk x/auth module.
// AUDOT NOTE - We still need to decide which fee collections to be activate. Code should be flexible enough
// to activate any of the fee type with parameters changes.

// GetFeeCollectorAccAddress gets the fee collector account address in sdk.AccAddress type from human readable name.
func (k Keeper) GetFeeCollectorAccAddress(feeCollectorName string) sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(feeCollectorName)
}

// GetFeeCollectorBalances gets the account balance of the inputed fee collector name.
func (k Keeper) GetFeeCollectorBalances(ctx sdk.Context, feeCollectorName string) sdk.Coins {
	balances := k.bankKeeper.GetAllBalances(ctx, k.GetFeeCollectorAccAddress(feeCollectorName))
	return balances
}

// DeductFees deduce fees of type based of feeCollector name from the investor address
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
// round. This could be the end of every epoch, 1-Week Gauge, 3-Week Gauge etc.
// return value will be deduced from the profit and allocated to the
// vault reserve.
// TODO | AUDIT | Initially taking 5% hardocded profit as performance fees.
// Should be a paramater of orion module
func (k Keeper) CalcPerFee(profit sdk.Coin) sdk.Coin {
	// TODO - To be added in vault parameter
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.05")
	feeAmt := profit.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(profit.GetDenom(), feeAmt)
}

// CalcEntryFee calculate the entry fee every time when a user deposit coins
// into vault. Return value will be deduced from the depositor account.
// Note : This function maynot be used for some type of strategies.
func (k Keeper) CalcEntryFee(depositAmt sdk.Coin) sdk.Coin {
	// TODO - Factor value to be added in parameter.
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.01")
	feeAmt := depositAmt.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(depositAmt.GetDenom(), feeAmt)
}

// CalcExitFee, calculate the exit fee every time when a user withdwar coins
// from vault. Return value will be deduced from the depositor
// account, who is exiting his positions.
func (k Keeper) CalcExitFee(exitAmt sdk.Coin) sdk.Coin {
	// TODO - Factor value to be added in parameter.
	var factor sdk.Dec = sdk.MustNewDecFromStr("0.01")
	feeAmt := exitAmt.Amount.ToDec().Mul(factor).RoundInt()
	return sdk.NewCoin(exitAmt.GetDenom(), feeAmt)
}
