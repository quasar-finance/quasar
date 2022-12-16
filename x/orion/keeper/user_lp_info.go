package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

// ProcessDepositDayLockupPair process the list of pairs <deposit epoch day, lockup period>
// Input param signifies the lockup period used on a given epoch day when users deposited their funds.
// Note -
// 1. This method is in connection with GetDepositDayInfos.
// 2. In this method, we are iterating over the qbank module KV store.
// This method should be called after GetDepositDayInfos at each EOD.
// Return []types.EpochUserDenomWeight is used to calculate the users reward percentage for a given epoch day.
// TODO refactor and define more function to  break the function into smaller ones.
func (k Keeper) ProcessDepositDayLockupPair(
	ctx sdk.Context,
	dlpairs []types.DepositDayLockupPair,
) ([]types.EpochUserDenomWeight, sdk.Coins) {

	totalCoins := sdk.NewCoins()
	userCoinsMap := make(map[string]sdk.Coins)
	var udws []types.EpochUserDenomWeight

	for _, dl := range dlpairs {
		userDeposits := k.qbankKeeper.GetEpochLockupDepositAllUsersAllDenoms(ctx, dl.EpochDay, dl.LockupPeriod)

		for uid, deposit := range userDeposits {
			totalCoins = totalCoins.Add(deposit...)
			if totalUserCoins, exist := userCoinsMap[uid]; exist {
				userCoinsMap[uid] = totalUserCoins.Add(deposit...)
			} else {
				userCoinsMap[uid] = deposit
			}
		}
	} // dlpairs for loop

	// Process user coin map
	for user, coins := range userCoinsMap {
		for _, coin := range coins {
			weight := sdk.NewDecFromInt(coin.Amount).QuoInt(totalCoins.AmountOf(coin.Denom))
			udw := types.EpochUserDenomWeight{UserAcc: user, Weight: weight, Coin: coin}
			udws = append(udws, udw)
		}
	}

	return udws, totalCoins
}
