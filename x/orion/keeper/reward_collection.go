package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetRewardCollection set the osmosis RewardCollection in kv store and is expected to be called
// on each epoch day.
func (k Keeper) SetRewardCollection(ctx sdk.Context, epochday uint64, rewardCollection types.RewardCollection) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochday)
	b := k.cdc.MustMarshal(&rewardCollection)
	store.Set(key, b)
}

// GetRewardCollection returns RewardCollection info for a given epoch day.
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

// RemoveRewardCollection removes RewardCollection from the store
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

// LPExpectedReward calculate per day (todays) expected reward based on current values of gauge and apy.
// Valid for the active LP position. This will be usefull for loss analytics from the expected reward.
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
				// Today's expected reward will be based on the current apy of the active gauge of the position.
				// But, it should not be confused with the real rewards which will be provided in osmo by osmosis.
				reward[i].Amount = coin.Amount.ToDec().Mul(RewardMuliplier).TruncateInt()
			}
		}
	}
	return reward
}

// GetUserRewardDistribution implements the reward distribution to users
func (k Keeper) RewardDistribution(ctx sdk.Context,
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
