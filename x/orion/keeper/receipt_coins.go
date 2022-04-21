package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetTotalOrions calculates the total amount of orions for the input sdk.Coins
// This method will be used for end user queries by the front end in two steps.
// Step #1 - Front end calls the current total users deposits as sdk.Coins to qbank module
// Step #2 - Front end calls the current Orions amount equivalent to the input sdk.Coins
// GetTotalOrions method will be used for step #2
// AUDIT TODO - grpc query to be added
func (k Keeper) GetTotalOrions(ctx sdk.Context, coins sdk.Coins) sdk.Coin {
	var orions sdk.Coin
	for _, coin := range coins {
		orion := k.CalcReceipts(ctx, coin)
		orions = orions.Add(orion)
	}
	return orions
}

// CalcReceipts calculates the amount of orion coin equivalent to the input sdk.Coin
func (k Keeper) CalcReceipts(ctx sdk.Context, coin sdk.Coin) sdk.Coin {
	spotPrice := k.GetSpotPrice(ctx, coin.Denom)
	OrionAmt := coin.Amount.ToDec().Mul(spotPrice).TruncateInt()
	return sdk.NewCoin(types.ModuleName, OrionAmt)
}

// GetSpotPrice gets the amount of UST equivalent to the input one denom from the qoracle module
func (k Keeper) GetSpotPrice(ctx sdk.Context, denom string) sdk.Dec {
	spotPrice := k.qoracleKeeper.GetStablePrice(ctx, denom)
	return spotPrice
}

// MintOrion will mint orions from the Orion module and transfer to the orion module reserve account.
func (k Keeper) MintOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.BankKeeper.MintCoins(ctx, types.OrionReserveMaccName, sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// BurnOrion will mint orions from the Orion module and transfer to the orion module reserve account.
func (k Keeper) BurnOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.BankKeeper.BurnCoins(ctx, types.OrionReserveMaccName,
		sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// GetUsersOrionShare calculates the percentage of users Orion share based on the
// Total Orion printed so far, and total users deposit in the orion vault.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions.
// This can also be extended to calc share in ref to total orions minted so far
func (k Keeper) GetUsersOrionShare(ctx sdk.Context, userAcc string) sdk.Dec {
	qc, _ := k.qbankKeeper.GetUserDepositAmt(ctx, userAcc)
	usersOrion := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range qc.Coins {
		orion := k.CalcReceipts(ctx, c)
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetTotalDeposits(ctx)
	totalOrions := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range allCoins {
		orion := k.CalcReceipts(ctx, c)
		totalOrions = totalOrions.Add(orion)
	}
	usershare := usersOrion.Amount.ToDec().QuoInt(totalOrions.Amount)

	return usershare
}

// GetEpochUsersOrionShare calculates the percentage of users Orion share based on a given epoch day
// and total users deposit in the orion vault on the same epoch day.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions
func (k Keeper) GetEpochUsersOrionShare(ctx sdk.Context,
	epochDay uint64, userAcc string) sdk.Dec {
	coins := k.qbankKeeper.GetEpochUserDepositAmt(ctx, epochDay, userAcc)
	usersOrion := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range coins {
		orion := k.CalcReceipts(ctx, c)
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetTotalEpochDeposits(ctx, epochDay)
	totalOrions := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range allCoins {
		orion := k.CalcReceipts(ctx, c)
		totalOrions = totalOrions.Add(orion)
	}
	usershare := usersOrion.Amount.ToDec().QuoInt(totalOrions.Amount)

	return usershare
}
