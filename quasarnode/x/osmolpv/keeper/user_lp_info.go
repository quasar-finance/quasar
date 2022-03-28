package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// AUDIT NOTE - This method could be redundant.
// SetUserLPInfo set userLPInfo in the store
func (k Keeper) SetUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string, userLPInfo types.UserLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	value := k.cdc.MustMarshal(&userLPInfo)
	store.Set(key, value)
}

// AUDIT NOTE - This method could be redundant.
// GetUserLPInfo returns userLPInfo
func (k Keeper) GetUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string) (val types.UserLPInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AUDIT NOTE - This method could be redundant.
// RemoveUserLPInfo removes userLPInfo from the store
func (k Keeper) RemoveUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	store.Delete(key)
}

// AUDIT NOTE - This method could be redundant.
// AddEpochLPUser add kv store with key = {epochday} + {":"} + {lpID} + {":"} + {userAccount}
// value = UserLPInfo. This method is to be used for once time only
func (k Keeper) AddEpochLPUserInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string, userLPInfo types.UserLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	value := k.cdc.MustMarshal(&userLPInfo)
	store.Set(key, value)

}

// AUDIT NOTE - This method could be redundant.
// Prepare user rewards for the given epoch day.
// Get the list of users denom deposit done on epochDepositDay for which the lockperiod is ended
// based on the epochRewardDay ( reward collection day )
// Logic -
// 1. Iterate over qbank epoch lockup denom deposit.
// 2. Filter out the entry for which lockup period is not yet ended, and collect the rest.
// 3. Build a local map epochUserCoins = <userAcc, depositCoins>
// 4. Process epochUserCoins for each coins and build <user, RewardCoins> based on weight
// TODO | AUDIT
func (k Keeper) PrepareEpochUsersRewards(ctx sdk.Context,
	epochDepositDay uint64, epochRewardDay uint64) []types.EpochUsersReward {
	return nil
}

// AUDIT NOTE - This method could be redundant.
// PrepareEpochUsersWeights prepares users denom weight on a given epoch based on
// deposited tokens.
// Return -  []types.EpochUserDenomWeight will be be used to calculate the users rewards
func (k Keeper) PrepareEpochUsersWeights(ctx sdk.Context,
	epochDepositDay uint64, epochRewardDay uint64) []types.EpochUserDenomWeight {

	// []types.EpochUsersReward {

	// Note - We are processing qbank keeper store key here. Can it be pre-processed.
	bytePrefix := qbanktypes.UserDenomDepositKBP
	prefixKey := types.EpochDayKey(epochDepositDay)
	prefixKey = append(bytePrefix, prefixKey...)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)
	// prefixKey = qbanktypes.UserDenomDepositKBP + ""
	store := ctx.KVStore(k.qbankKeeper.GetStoreKey())
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("PrepareUsersReward|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	// key = {lockupString} + ":" + {uid} + ":" + {denom}
	var totalCoins sdk.Coins
	var ucs []types.UserCoin
	var udws []types.EpochUserDenomWeight

	for ; iter.Valid(); iter.Next() {
		key, val := iter.Key(), iter.Value()
		bsplits := qbanktypes.SplitKeyBytes(key)
		uid := string(bsplits[1])
		// denom := string(bsplits[2])
		// var userReward types.EpochUsersReward
		var coin sdk.Coin
		k.cdc.MustUnmarshal(val, &coin)
		uc := types.UserCoin{UserAcc: uid, Coin: coin}
		ucs = append(ucs, uc)
		totalCoins = totalCoins.Add(coin) // Test if totalCoins will get sorted internally
	}

	for _, uc := range ucs {
		denom := uc.Coin.Denom
		totalDenomAmt := totalCoins.AmountOf(denom)
		wieght := uc.Coin.Amount.ToDec().QuoInt(totalDenomAmt)
		udw := types.EpochUserDenomWeight{UserAcc: uc.UserAcc, Denom: denom, Weight: wieght}
		udws = append(udws, udw)
	}

	return udws
}

// ProcessDepositDayLockupPair process the list of pairs <deposit epoch day, lockup period>
// Input param signifies the lockup period used on a given epoch day where users deopisted their funds.
// Note -
// 1. This method is in connection with GetDepositDayInfos.
// 2. In this method, we are iterating over the qbank module KV store.
// This method should be called after GetDepositDayInfos at each EOD.
// Return []types.EpochUserDenomWeight is used to calculate the users reward percentage for a given epoch day.
func (k Keeper) ProcessDepositDayLockupPair(ctx sdk.Context,
	dlpairs []types.DepositDayLockupPair) []types.EpochUserDenomWeight {

	totalDenomAmtMap := make(map[string]sdk.Int) // Key = denom, Value = sdk.Int
	userCoinsMap := make(map[string]sdk.Coins)   // key = userAcc, Value = sdk.Coins
	var udws []types.EpochUserDenomWeight

	for _, dl := range dlpairs {
		// Prepare prefix key with epochday and lockup period
		bytePrefix := qbanktypes.UserDenomDepositKBP
		prefixKey := qbanktypes.CreateEpochLockupUserKey(dl.Epochday, dl.LockupPeriod, qbanktypes.Sep)
		prefixKey = append(bytePrefix, prefixKey...)

		// prefixKey = qbanktypes.UserDenomDepositKBP + {epochday} + ":" + "lockupString" + "/"
		store := ctx.KVStore(k.qbankKeeper.GetStoreKey())
		iter := sdk.KVStorePrefixIterator(store, prefixKey)
		defer iter.Close()

		logger := k.Logger(ctx)
		logger.Info(fmt.Sprintf("ProcessDepositDayLockupPair|modulename=%s|blockheight=%d|prefixKey=%s",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

		// Key = {userAcc} + {":"} + {Denom} , Value = sdk.Coin
		for ; iter.Valid(); iter.Next() {
			key, val := iter.Key(), iter.Value()
			bsplits := qbanktypes.SplitKeyBytes(key)
			uid := string(bsplits[1])
			denom := string(bsplits[2])

			var coin sdk.Coin
			k.cdc.MustUnmarshal(val, &coin)

			if amt, found := totalDenomAmtMap[denom]; found {
				totalDenomAmtMap[denom] = amt.Add(coin.Amount)
			} else {
				totalDenomAmtMap[denom] = coin.Amount
			}

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
			weight := coin.Amount.ToDec().QuoInt(totalDenomAmtMap[coin.Denom])
			udw := types.EpochUserDenomWeight{UserAcc: user, Denom: coin.Denom, Weight: weight, Amt: coin.Amount}
			udws = append(udws, udw)
		}
	}

	return udws
}
