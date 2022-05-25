package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
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
		// Prepare prefix key with epochday and lockup period
		bytePrefix := qbanktypes.UserDenomDepositKBP
		prefixKey := qbanktypes.CreateEpochLockupUserKey(dl.EpochDay, dl.LockupPeriod, qbanktypes.Sep)
		prefixKey = append(bytePrefix, prefixKey...)

		// prefixKey = qbanktypes.UserDenomDepositKBP + {epochDay} + ":" + "lockupString" + ":"
		store := ctx.KVStore(k.qbankKeeper.GetStoreKey())
		iter := sdk.KVStorePrefixIterator(store, prefixKey)
		defer iter.Close()

		logger := k.Logger(ctx)
		logger.Debug("ProcessDepositDayLockupPair",
			"BlockHeight", ctx.BlockHeight(),
			"prefixKey", string(prefixKey))

		// Key = {userAcc} + {":"} + {Denom} , Value = sdk.Coin
		for ; iter.Valid(); iter.Next() {
			key, val := iter.Key(), iter.Value()
			bsplits := qbanktypes.SplitKeyBytes(key)
			uid := string(bsplits[1])

			var coin sdk.Coin
			k.cdc.MustUnmarshal(val, &coin)

			totalCoins = totalCoins.Add(coin)

			if coins, found := userCoinsMap[uid]; found {
				userCoinsMap[uid] = coins.Add(coin)
			} else {
				userCoinsMap[uid] = sdk.NewCoins(coin)
			}
		}

	} // dlpairs for loop

	// Process user coin map
	for user, coins := range userCoinsMap {
		for _, coin := range coins {
			weight := coin.Amount.ToDec().QuoInt(totalCoins.AmountOf(coin.Denom))
			udw := types.EpochUserDenomWeight{UserAcc: user, Weight: weight, Coin: coin}
			udws = append(udws, udw)
		}
	}

	return udws, totalCoins
}
