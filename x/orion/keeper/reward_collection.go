package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetRewardCollection set rewardCollection from osmosis in the store on a given epoch day.
// This method should be called on the end blocker. Orion module should initiate reward collection
// at the end blocker via intergamm module
// TODO | AUDIT - Make sure the internal coin slice is sorted properly
func (k Keeper) SetRewardCollection(ctx sdk.Context, epochday uint64, rewardCollection types.RewardCollection) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochday)
	b := k.cdc.MustMarshal(&rewardCollection)
	store.Set(key, b)
}

// GetRewardCollection returns rewardCollection on a given epoch day.
// Assuming that the reward is collected every day successful.
// TODO | AUDIT - Edge case If reward can not be collected due to network issue, relayer issue or chain halts etc.
func (k Keeper) GetRewardCollection(ctx sdk.Context, epochday uint64) (val types.RewardCollection, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochday)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveRewardCollection removes rewardCollection from the store
func (k Keeper) RemoveRewardCollection(ctx sdk.Context, epochday uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochday)
	store.Delete(key)
}

// LPPositionReward calculate the reward associated with a particulat LP positon on a given epochday
// Param - epochday is the day on which users deposited fund. lpID is unique id for the LP position
func (k Keeper) LPPositionReward(ctx sdk.Context, epochday uint64, lpID uint64) (sdk.Coins, error) {

	lp, _ := k.GetLpPosition(ctx, epochday, lpID)
	expectedRewardCollectionDay := epochday + lp.BondDuration + lp.UnbondingDuration + 2
	totalReward, found := k.GetRewardCollection(ctx, expectedRewardCollectionDay)
	var result sdk.Coins
	if found {
		w, error := k.CalculateLPWeight(ctx, epochday, lpID)
		if error == nil {
			return result, fmt.Errorf("LPID=%v not found", lpID)
		}
		coins := totalReward.Coins
		for _, coin := range coins {
			if !coin.IsZero() {
				tmp := coin.Amount.ToDec().Mul(w).TruncateInt()
				coin.Amount = tmp
				result = append(result, coin)
			}
		}
	} else {
		return result, fmt.Errorf("reward not found for collection epoch day %v", expectedRewardCollectionDay)
	}
	return result, nil
}

// AUDIT NOTE : This method may be redundant
// CalculateEpochLPUsersReward calculate a users reward for a givep lp position in the given epoch day.
// lpReward is the reward collected for an lp position.
// lpshare is the share contribution to the lp position by a user.
// Caller of this function should determine the lp reward for the position, fetch
// the user lp shares before calling.
func (k Keeper) CalculateLPUsersReward(ctx sdk.Context, lpReward sdk.Coins, lpshare sdk.Dec) sdk.Coins {
	var userReward sdk.Coins
	for _, r := range lpReward {
		amt := r.Amount.ToDec().Mul(lpshare).TruncateInt()
		userReward = userReward.Add(sdk.NewCoin(r.Denom, amt))
	}
	return userReward
}

// LPExpectedReward calculate per day (todays) expected reward based on current values of gauge and apy.
// Valid for the active LP position. This will be usefull for loss analytics from the expected reward.
// TODO - AUDIT | An extended
func (k Keeper) LPExpectedReward(ctx sdk.Context, lpID uint64) sdk.Coins {
	var reward sdk.Coins
	epochday, efound := k.GetLPEpochDay(ctx, lpID)
	g := k.GetCurrentActiveGauge(ctx, epochday, lpID)
	RewardMuliplier := g.ExpectedApy.QuoInt64(types.NumDaysPerYear)

	if efound {
		lp, lpfound := k.GetLpPosition(ctx, epochday, lpID)
		if lpfound {
			reward = lp.Coins // initialize
			for i, coin := range lp.Coins {
				// Todays. expected reward will be based on the current apy of the active gauge of the position.
				// But, it should not be confused with the real rewards which will be provided in osmo by osmosis.
				reward[i].Amount = coin.Amount.ToDec().Mul(RewardMuliplier).TruncateInt()
			}
		}
	}
	return reward
}

// GetUserRewardDistribution2 - Gives the users reward distribution map.
// AUDIT TODO - The reward is to be available to the users view for claim in qbank module KV store.
// Based on the return value of this function. And the actual reward amount should be moved to orion
// reward collection global account. When a user will click on the claim reward it will go to the users account

func (k Keeper) GetUserRewardDistribution2(ctx sdk.Context,
	epochday uint64) error {

	lpids := k.GetActiveLpIDList(ctx, epochday)
	rc, found := k.GetRewardCollection(ctx, epochday)

	if !found {
		return fmt.Errorf("rewards not yet collected for epoch day %v", epochday)
	}

	// Deduce the performance fees for each denom
	var perFees sdk.Coins
	for _, c := range rc.Coins {
		perFees = perFees.Add(k.CalcPerFee(ctx, c))
	}
	// rc.Coins to be used for further reward distribution so needs to be substracted by perFees
	rc.Coins = rc.Coins.Sub(perFees)
	k.DeductVaultFees(ctx, types.CreateOrionRewardGloablMaccName(), types.PerfFeeCollectorMaccName, perFees)

	denomExpReward := make(map[string]sdk.Coin)
	denomActualReward := make(map[string]sdk.Coins) // AUDIT TODO
	denomAmt := make(map[string]sdk.Int)
	var totalLPV sdk.Coins // Total LP in deployment

	// Preparing denomExpReward and denomAmt
	for _, lpid := range lpids {
		lpday, _ := k.GetLPEpochDay(ctx, lpid)
		lp, _ := k.GetLpPosition(ctx, lpday, lpid)
		g := k.GetCurrentActiveGauge(ctx, lpday, lpid)
		for _, c := range lp.Coins {
			totalLPV = totalLPV.Add(c)
			if v, ok := denomAmt[c.Denom]; ok {
				denomAmt[c.Denom] = v.Add(c.Amount)
			} else {
				denomAmt[c.Denom] = c.Amount
			}
			// Calc todays expected reward based APY in terms of LP tokens
			rAmt := lp.Lptoken.Amount.ToDec().Mul(g.ExpectedApy).TruncateInt()
			rCoin := sdk.NewCoin(lp.Lptoken.Denom, rAmt)
			if v, ok := denomExpReward[c.Denom]; ok {
				denomExpReward[c.Denom] = v.Add(rCoin)
			} else {
				denomExpReward[c.Denom] = rCoin
			}
		}
	} // lpids loop

	// Fill the denomActualRewardMap
	totalEquivalentReceipt := k.GetTotalOrions(ctx, totalLPV)
	for denom, amt := range denomAmt {
		equivalentReciept := k.CalcReceipts(ctx, sdk.NewCoin(denom, amt))
		weight := equivalentReciept.Amount.ToDec().Quo(totalEquivalentReceipt.Amount.ToDec())
		for _, rewardCoin := range rc.Coins {
			reward := sdk.NewCoin(rewardCoin.Denom, rewardCoin.Amount.ToDec().Mul(weight).TruncateInt())
			if r, ok := denomActualReward[denom]; ok {
				denomActualReward[denom] = r.Add(reward)
			} else {
				denomActualReward[denom] = sdk.NewCoins(reward)
			}
		}
	}
	//////////////////////////////
	// Get users denom reward
	// Note - This iteration is happening qbank module keeper.
	uim := make(types.UserInfoMap)
	bytePrefix := qbanktypes.UserDenomDepositKBP
	store := ctx.KVStore(k.qbankKeeper.GetStoreKey())
	iter := sdk.KVStorePrefixIterator(store, bytePrefix)
	defer iter.Close()
	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetUserRewardDistribution2|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(bytePrefix)))

	// Key = "{uid}" + {":"} + "{denom}", value = sdk.Coin
	for ; iter.Valid(); iter.Next() {
		var udi types.UserDenomInfo
		key, value := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		userAcc := string(splits[0])
		denom := string(splits[1])
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		udi.Denom = denom
		udi.Amt = coin.Amount
		udi.Weight = udi.Amt.ToDec().QuoInt(denomAmt[denom])
		for _, r := range denomActualReward[denom] {
			tmp := sdk.NewCoin(r.Denom, r.Amount.ToDec().Mul(udi.Weight).TruncateInt())
			udi.Reward = udi.Reward.Add(tmp)
		}

		if tmpui, ok := uim[userAcc]; ok {
			for _, c := range udi.Reward {
				tmpui.TotalReward = tmpui.TotalReward.Add(c)
			}
			tmpui.DenomMap[denom] = udi
			uim[userAcc] = tmpui
		} else {
			var ui types.UserInfo
			ui.UserAcc = userAcc
			ui.TotalReward = ui.TotalReward.Add()
			ui.DenomMap = make(map[string]types.UserDenomInfo)
			ui.DenomMap[denom] = udi
			uim[userAcc] = ui
		}

	}

	for userAcc, ui := range uim {
		k.qbankKeeper.AddUserClaimRewards(ctx, userAcc, types.ModuleName, ui.TotalReward)
	}

	return nil
}

// AUDIT NOTE - Incorrect implementation.
// DistributeRewards is used to distribute the rewards for a given epoch day.
// Logic -
// 1. Fetch the reward from the epoch KV store GetRewardCollection.
//  Note - Assuming that the GetRewardCollection will return the reward at the end
// the users deposit lockups.
// AUDIT - TODO | So storage into this need to be implemented carefully.
// 2. Fetch the corresponding epoch deposit day and lockup periods using GetDepositDayInfos
// 3. Process the ProcessDepositDayLockupPair and get EpochUserDenomWeight
// 4. Get Each Denoms weight based on equivalent orions. Get list of denoms used on deposit day
// 5. Calculate denom contribution of reward
// 6. Calculate the users contribution for each of users denom deposit based on EpochUserDenomWeight
// Return - map[string]sdk.Coins // key = user, value = sdk.Coins
func (k Keeper) GetUserRewardDistribution(ctx sdk.Context, epochday uint64) (map[string]sdk.Coins, error) {
	rc, found := k.GetRewardCollection(ctx, epochday)
	if !found {
		return nil, fmt.Errorf("rewards not yet collected for epoch day %v", epochday)
	}

	// epochday is the reward collection day here.
	// ddlps => slice of DepositDay - Lockup Pair. []types.DepositDayLockupPair
	ddlps := k.GetDepositDayInfos(ctx, epochday)
	// denomWeights => []types.EpochUserDenomWeight
	userDenomWeights := k.ProcessDepositDayLockupPair(ctx, ddlps)
	// denomWeights => []types.EpochDenomWeight
	denomWeights := k.GetEpochDenomWeight(ctx, epochday)
	denomRewardMap := make(map[string]sdk.Coins) // key = denom, value = sdk.Coins
	userRewardMap := make(map[string]sdk.Coins)  // key = user, value = sdk.Coins

	// Process one denom at a time
	for _, dw := range denomWeights {
		var rewards sdk.Coins
		// rc => reward coin will be mostly osmo ibc coins
		for _, coin := range rc.Coins {
			reward := coin.Amount.ToDec().Mul(dw.Weight).TruncateInt() // one denom amount
			rewards = rewards.Add(sdk.NewCoin(coin.Denom, reward))
		}

		denomRewardMap[dw.Denom] = rewards
	}

	// One user - denom at a time. udr => user denom rewards
	for _, udw := range userDenomWeights {
		denomTotalRewards := denomRewardMap[udw.Denom]
		ur := userRewardMap[udw.UserAcc]
		for _, coin := range denomTotalRewards {
			rAmt := coin.Amount.ToDec().Mul(udw.Weight).TruncateInt()
			rCoin := sdk.NewCoin(udw.Denom, rAmt)
			ur = append(ur, rCoin)
		}
		userRewardMap[udw.UserAcc] = ur
	}
	return userRewardMap, nil
}

// AUDIT NOTE - Incorrect implementation.
// DistributeRewards distribute the rewards to the end users
func (k Keeper) DistributeRewards(ctx sdk.Context, epochday uint64) error {
	userRewardMap, err := k.GetUserRewardDistribution(ctx, epochday)
	if err != nil {
		// Note - Can't panic because it is not a tx message processing.
		// This process is happning in the end blocker
		return err
	}
	for user, reward := range userRewardMap {
		// Update the qbank module KV store
		for _, r := range reward {
			k.qbankKeeper.AddUserClaimReward(ctx, user, types.ModuleName, r)
		}
	}

	return nil
}

// Brainstorming Notes -
// What is the reward collection day for a given LP position ?
// Expected Day =>  LP Day + lockup period + 2 = LP Day + bond duration + unbond duration + 2
// Day 1 - LP collection in Quasar.
// Day 2 - Osmosis LPing will be done on the next day just after the osmosis EOD.
// Day N - Reward collection, Pool Exit and Distribution. N = Day 2 + bond duration + unbond duration + 1
// Notes
// - LPExpectedReward and LPPositionReward should not differ by a large amount
// - LPExpectedReward can be used by cross validation and fulfiling users expectation in
//   case of destination chain failure/halt
