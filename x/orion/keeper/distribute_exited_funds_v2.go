package keeper

import (
	"fmt"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) GetCorrespondingEpochExitCoins(
	ctx sdk.Context,
	epochDay uint64,
	coins sdk.Coins,
) (res sdk.Coins) {
	res = sdk.NewCoins()
	for _, coin := range coins {
		res = res.Add(k.GetEpochExitAmt(ctx, epochDay, coin.Denom))
	}
	return res
}

// DistributeEpochLockupFundsV2 distribute the exited funds to the depositors at the end of every epoch day.
// Logic -
// 0. Fetch the actual deposit day and lockup periods corresponding to the distributionDay.
// 1. Calculate the total deposited funds on actual deposit day (totalNeededCoins).
// 2. Calculate the amount of funds exited from the osmosis today (epochExitCoins).
// 3. Get the Balance of the reserve account (reserveCoins).
// 4. Compute the coins that should be allocated from epochExitCoins and/or reserveCoins, and total deficit.
// 5. Mint orions to cover the possible deficits.
// 6. Calculate share of each user and the fees.
// 7. Add the actual withdrawable amount in qbank kv store.
// 8. Deduct the fees.
func (k Keeper) DistributeEpochLockupFundsV2(
	ctx sdk.Context,
	distributionDay uint64,
) error {
	//  []types.DepositDayLockupPair
	ddlp := k.GetDepositDayInfos(ctx, distributionDay)
	epochUserInfo, totalNeededCoins := k.ProcessDepositDayLockupPairV2(ctx, ddlp)

	epochExitCoins := k.GetCorrespondingEpochExitCoins(ctx, distributionDay, totalNeededCoins)
	reserveCoins := k.GetAllReserveBalances(ctx)
	fromEpochExit, fromReserve, excessEpochExitCoins, totalDeficit :=
		CalculateCoinAllocations(totalNeededCoins, epochExitCoins, reserveCoins)
	availableCoins := fromEpochExit.Add(fromReserve...)

	orionsMintedForEachDenom, err := k.MintDeficit(ctx, totalDeficit)
	if err != nil {
		return err
	}

	k.Logger(ctx).Info(
		fmt.Sprintf("DistributeEpochLockupFunds|Epochday=%v|totalNeededCoins=%s|"+
			"excessCoins=%s|orionsMintedForEachDenom=%v|fromReserve=%s\n",
			distributionDay,
			totalNeededCoins.String(),
			excessEpochExitCoins.String(),
			orionsMintedForEachDenom,
			fromReserve.String()))

	mgmtFeePercentage := k.MgmtFeePer(ctx)

	for _, v := range epochUserInfo {
		denom := v.Coin.Denom
		userCoins, mgmtFees := CalculateUserCoinsAndFees(denom, v.Weight, availableCoins, orionsMintedForEachDenom, mgmtFeePercentage)

		k.AddWithdrawableCoins(ctx, v.UserAcc, userCoins)

		userAccAddr, _ := sdk.AccAddressFromBech32(v.UserAcc)
		k.DeductAccFees(ctx, userAccAddr, types.MgmtFeeCollectorMaccName, mgmtFees)
	}
	return nil
}

// MintDeficit mints (equivalent value of) orions to cover the deficits.
// Since all minted coins have the same denominations, a map is used (rather than sdk.Coins),
// to keep track of how much orions is minted for each denomination.
func (k Keeper) MintDeficit(ctx sdk.Context, totalDeficit sdk.Coins) (orionsMintedForEachDenom map[string]sdk.Coin, err error) {
	orionsMintedForEachDenom = make(map[string]sdk.Coin)
	for _, c := range totalDeficit {
		orionsMintedForEachDenom[c.Denom], err = k.MintAndAllocateOrions(ctx, c)
		if err != nil {
			return nil, err
		}
	}
	return orionsMintedForEachDenom, nil
}

// AddWithdrawableCoins iterates over coins, and call AddActualWithdrawableAmt of qbank for each coin.
func (k Keeper) AddWithdrawableCoins(ctx sdk.Context, addr string, coins sdk.Coins) {
	for _, coin := range coins {
		// TODO add similar function to QBank accepting sdk.Coins
		k.qbankKeeper.AddActualWithdrawableAmt(ctx, addr, coin)
	}
}

// CalculateCoinAllocations tries to allocate neededCoins first from epochExitCoins, and then from reserveCoins.
// It will also return the possible deficit.
// If epochExitCoins has more coins than the neededCoins, that will be returned too.
func CalculateCoinAllocations(
	neededCoins,
	epochExitCoins,
	reserveCoins sdk.Coins,
) (fromEpochExit, fromReserve, excessEpochExit, totalDeficit sdk.Coins) {
	fromEpochExit = neededCoins.Min(epochExitCoins)
	excessEpochExit = epochExitCoins.Sub(fromEpochExit)
	fromReserve = neededCoins.Sub(fromEpochExit).Min(reserveCoins)
	totalDeficit = neededCoins.Sub(fromEpochExit).Sub(fromReserve)
	return
}

// CalculateUserCoinsAndFees calculates the share of a user from the availableCoins anf minted orions,
// after deducting the management fees.
func CalculateUserCoinsAndFees(
	depositedDenom string,
	depositorWeight sdk.Dec,
	availableCoins sdk.Coins,
	orionsMintedForEachDenom map[string]sdk.Coin,
	mgmtFeePercentage sdk.Dec,
) (userCoins, mgmtFees sdk.Coins) {
	userDecCoins := sdk.NewDecCoins(sdk.NewDecCoin(depositedDenom, availableCoins.AmountOf(depositedDenom)))

	if mintedForThisDenom, exist := orionsMintedForEachDenom[depositedDenom]; exist {
		userDecCoins = userDecCoins.Add(sdk.NewDecCoinFromCoin(mintedForThisDenom))
	}

	userDecCoins = userDecCoins.MulDec(depositorWeight)

	mgmtFeesDecCoins := userDecCoins.MulDecTruncate(mgmtFeePercentage)
	userDecCoins = userDecCoins.Sub(mgmtFeesDecCoins)

	// TODO get the change into a public spending pool
	userCoins, _ /*userChangeCoins*/ = userDecCoins.TruncateDecimal()
	mgmtFees, _ /*mgmtFeeChangeCoins*/ = mgmtFeesDecCoins.TruncateDecimal()

	return
}
