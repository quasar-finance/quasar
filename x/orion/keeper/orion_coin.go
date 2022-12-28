package keeper

import (
	sdkmath "cosmossdk.io/math"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

// GetTotalOrions calculates the total amount of orions for the input sdk.Coins
func (k Keeper) GetTotalOrions(ctx sdk.Context, coins sdk.Coins) (sdk.Coin, error) {
	orions := sdk.NewCoin(types.OrionDenom, sdk.ZeroInt())
	for _, coin := range coins {
		orion, err := k.CalcReceipts(ctx, coin)
		if err != nil {
			return sdk.Coin{}, err
		}
		orions = orions.Add(orion)
	}
	return orions, nil
}

// CalcReceipts calculates the amount of orion coin equivalent to the input sdk.Coin
// Most updated value of US dollar value is the base for the orion token calculations.
func (k Keeper) CalcReceipts(ctx sdk.Context, coin sdk.Coin) (sdk.Coin, error) {
	spotPrice, err := k.GetRelativeStablePrice(ctx, coin.Denom, types.OrionDenom)
	if err != nil {
		return sdk.Coin{}, err
	}
	OrionAmt := sdk.NewDecFromInt(coin.Amount).Mul(spotPrice).TruncateInt()
	return sdk.NewCoin(types.OrionDenom, OrionAmt), nil
}

// GetStablePrice gets the amount of UST equivalent to the input one denom from the qoracle module
func (k Keeper) GetStablePrice(ctx sdk.Context, denom string) (price sdk.Dec, found bool) {
	price, err := k.qoracleKeeper.GetDenomPrice(ctx, denom)
	if err != nil {
		return price, false
	}
	return price, true
}

// GetRelativeStablePrice gets the amount of denomOut equivalent to one denomIn from the qoracle module
func (k Keeper) GetRelativeStablePrice(ctx sdk.Context, denomIn, denomOut string) (price sdk.Dec, err error) {
	return k.qoracleKeeper.GetRelativeDenomPrice(ctx, denomIn, denomOut)
}

// MintOrion mint orions tokens from the OrionReserveMaccName
func (k Keeper) MintOrion(ctx sdk.Context, amt sdkmath.Int) error {
	return k.BankKeeper.MintCoins(ctx, types.OrionReserveMaccName, sdk.NewCoins(sdk.NewCoin(types.OrionDenom, amt)))
}

// BurnOrion will mint orions from the OrionReserveMaccName
func (k Keeper) BurnOrion(ctx sdk.Context, amt sdkmath.Int) error {
	return k.BankKeeper.BurnCoins(ctx, types.OrionReserveMaccName,
		sdk.NewCoins(sdk.NewCoin(types.OrionDenom, amt)))
}

// GetEpochUsersOrionShare calculates the percentage of users Orion share based on a given epoch day
// and total users deposit in the orion vault on the same epoch day.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions.
func (k Keeper) GetEpochUsersOrionShare(ctx sdk.Context, epochDay uint64, userAcc string) (sdk.Dec, error) {
	coins := k.qbankKeeper.GetEpochUserDepositAmt(ctx, epochDay, userAcc)
	usersOrion := sdk.NewCoin(types.OrionDenom, sdk.ZeroInt())
	for _, c := range coins {
		orion, err := k.CalcReceipts(ctx, c)
		if err != nil {
			// TODO recheck error handling
			return sdk.Dec{}, err
		}
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetTotalEpochDeposits(ctx, epochDay)
	totalOrions := sdk.NewCoin(types.OrionDenom, sdk.ZeroInt())
	for _, c := range allCoins {
		orion, err := k.CalcReceipts(ctx, c)
		if err != nil {
			// TODO recheck error handling
			return sdk.Dec{}, err
		}
		totalOrions = totalOrions.Add(orion)
	}
	userShare := sdk.NewDecFromInt(usersOrion.Amount).QuoInt(totalOrions.Amount)

	return userShare, nil
}
