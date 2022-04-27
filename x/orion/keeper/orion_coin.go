package keeper

import (
	"errors"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetTotalOrions calculates the total amount of orions for the input sdk.Coins
func (k Keeper) GetTotalOrions(ctx sdk.Context, coins sdk.Coins) (sdk.Coin, error) {
	var orions sdk.Coin
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
	denomPrice, found := k.GetStablePrice(ctx, coin.Denom)
	if !found {
		return sdk.Coin{}, errors.New("error: stable price not found")
	}
	orionPrice, found := k.GetStablePrice(ctx, types.ModuleName)
	if !found {
		return sdk.Coin{}, errors.New("error: stable price not found")
	}
	if orionPrice.IsZero() {
		return sdk.Coin{}, errors.New("error: orion price is zero")
	}
	spotPrice := denomPrice.Quo(orionPrice)
	OrionAmt := coin.Amount.ToDec().Mul(spotPrice).TruncateInt()
	return sdk.NewCoin(types.ModuleName, OrionAmt), nil
}

// GetStablePrice gets the amount of UST equivalent to the input one denom from the qoracle module
func (k Keeper) GetStablePrice(ctx sdk.Context, denom string) (price sdk.Dec, found bool) {
	return k.qoracleKeeper.GetStablePrice(ctx, denom)
}

// MintOrion mint orions tokens from the OrionReserveMaccName
func (k Keeper) MintOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.BankKeeper.MintCoins(ctx, types.OrionReserveMaccName, sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// BurnOrion will mint orions from the OrionReserveMaccName
func (k Keeper) BurnOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.BankKeeper.BurnCoins(ctx, types.OrionReserveMaccName,
		sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// GetEpochUsersOrionShare calculates the percentage of users Orion share based on a given epoch day
// and total users deposit in the orion vault on the same epoch day.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions.
func (k Keeper) GetEpochUsersOrionShare(ctx sdk.Context, epochDay uint64, userAcc string) (sdk.Dec, error) {
	coins := k.qbankKeeper.GetEpochUserDepositAmt(ctx, epochDay, userAcc)
	usersOrion := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range coins {
		orion, err := k.CalcReceipts(ctx, c)
		if err != nil {
			// TODO recheck error handling
			return sdk.Dec{}, err
		}
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetTotalEpochDeposits(ctx, epochDay)
	totalOrions := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range allCoins {
		orion, err := k.CalcReceipts(ctx, c)
		if err != nil {
			// TODO recheck error handling
			return sdk.Dec{}, err
		}
		totalOrions = totalOrions.Add(orion)
	}
	userShare := usersOrion.Amount.ToDec().QuoInt(totalOrions.Amount)

	return userShare, nil
}
