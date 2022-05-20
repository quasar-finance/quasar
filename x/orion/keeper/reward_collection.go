package keeper

import (
	"errors"
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetRewardCollection set the osmosis RewardCollection in kv store and is expected to be called
// on each epoch day.
func (k Keeper) SetRewardCollection(ctx sdk.Context, epochDay uint64, rewardCollection types.RewardCollection) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochDay)
	b := k.cdc.MustMarshal(&rewardCollection)
	store.Set(key, b)
}

// GetRewardCollection returns RewardCollection info for a given epoch day.
func (k Keeper) GetRewardCollection(ctx sdk.Context, epochDay uint64) (val types.RewardCollection, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochDay)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveRewardCollection removes RewardCollection from the store
func (k Keeper) RemoveRewardCollection(ctx sdk.Context, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochDay)
	store.Delete(key)
}

// LPPositionReward calculate the reward associated with a particular LP position on a given epochDay
// Param - epochDay is the day on which users deposited fund. lpID is unique id for the LP position
func (k Keeper) LPPositionReward(ctx sdk.Context, epochDay uint64, lpID uint64) (sdk.Coins, error) {
	lp, _ := k.GetLpPosition(ctx, epochDay, lpID)
	expectedRewardCollectionDay := epochDay + lp.BondDuration + lp.UnbondingDuration + 2
	totalReward, found := k.GetRewardCollection(ctx, expectedRewardCollectionDay)
	if !found {
		return nil, fmt.Errorf("reward not found for collection epoch day %v", expectedRewardCollectionDay)
	}
	w, err := k.CalculateLPWeight(ctx, epochDay, lpID)
	if err != nil {
		return nil, err
	}
	result, _ := sdk.NewDecCoinsFromCoins(totalReward.Coins...).MulDec(w).TruncateDecimal()
	return result, nil
}

// LPExpectedReward calculate per day (today's) expected reward based on current values of gauge and apy.
// Valid for the active LP position. This will be useful for loss analytics from the expected reward.
func (k Keeper) LPExpectedReward(ctx sdk.Context, lpID uint64) (sdk.Coins, error) {
	epochDay, found := k.GetLPEpochDay(ctx, lpID)
	if !found {
		return nil, errors.New("error: LP epochDay not found")
	}
	lp, found := k.GetLpPosition(ctx, epochDay, lpID)
	if !found {
		return nil, errors.New("error: LP position not found")
	}
	g := k.GetCurrentActiveGauge(ctx, epochDay, lpID)
	RewardMultiplier := g.ExpectedApy.QuoInt64(types.NumDaysPerYear)
	reward, _ /*change*/ := sdk.NewDecCoinsFromCoins(lp.Coins...).MulDec(RewardMultiplier).TruncateDecimal()
	return reward, nil
}

// RewardDistribution implements the reward distribution to users
func (k Keeper) RewardDistribution(ctx sdk.Context, epochDay uint64) error {
	rc, found := k.GetRewardCollection(ctx, epochDay)

	if !found {
		return fmt.Errorf("rewards not yet collected for epoch day %v", epochDay)
	}

	perFees, err := k.DeductPerformanceFee(ctx, rc.Coins)
	if err != nil {
		// TODO recheck error handling
		return err
	}
	// rc.Coins to be used for further reward distribution so needs to be subtracted by perFees
	rc.Coins = rc.Coins.Sub(perFees)

	totalLPV := k.CalculateTotalLPV(ctx, epochDay) // Total LP in deployment

	denomActualReward, err := k.CalculateActualRewardForEachDenom(ctx, totalLPV, rc.Coins)
	if err != nil {
		return err
	}

	uim := k.CalculateUserRewards(ctx, totalLPV, denomActualReward)

	for userAcc, ui := range uim {
		k.qbankKeeper.AddUserClaimRewards(ctx, userAcc, types.ModuleName, ui.TotalReward)
	}

	return nil
}

// DeductPerformanceFee calculates the performance fee and deducts it from the profits.
func (k Keeper) DeductPerformanceFee(ctx sdk.Context, profits sdk.Coins) (sdk.Coins, error) {
	perFees := k.CalculatePerformanceFeeForCoins(ctx, profits)
	err := k.DeductVaultFees(ctx, types.CreateOrionRewardGloablMaccName(), types.PerfFeeCollectorMaccName, perFees)
	return perFees, err
}

// CalculateTotalLPV calculates the total liquidity provided to the vault that are active on the given epochDay.
func (k Keeper) CalculateTotalLPV(ctx sdk.Context, epochDay uint64) sdk.Coins {
	lpIds := k.GetActiveLpIDList(ctx, epochDay)
	totalLPV := sdk.NewCoins() // Total LP in deployment
	// Preparing totalLPV
	for _, lpId := range lpIds {
		lpDay, _ := k.GetLPEpochDay(ctx, lpId)
		lp, _ := k.GetLpPosition(ctx, lpDay, lpId)
		totalLPV = totalLPV.Add(lp.Coins...)
	} // lpIds loop
	return totalLPV
}

// CalculateActualRewardForEachDenom calculates the share of each denom from the rewards.
// The share of each denom from the rewards is the same as its share of fiat value from the totalLPV.
// Fiat value is computed for the present time.
// The totalLPV is the the total liquidity provided to the vault that are active on the given epochDay.
func (k Keeper) CalculateActualRewardForEachDenom(ctx sdk.Context, totalLPV, netRewards sdk.Coins) (map[string]sdk.Coins, error) {
	denomActualReward := make(map[string]sdk.Coins)
	// Fill the denomActualRewardMap
	totalEquivalentReceipt, err := k.GetTotalOrions(ctx, totalLPV)
	if err != nil {
		return denomActualReward, err
	}
	for _, coin := range totalLPV {
		equivalentReceipt, err := k.CalcReceipts(ctx, coin)
		if err != nil {
			return denomActualReward, err
		}
		weight := equivalentReceipt.Amount.ToDec().Quo(totalEquivalentReceipt.Amount.ToDec())
		denomActualReward[coin.Denom], _ = sdk.NewDecCoinsFromCoins(netRewards...).MulDec(weight).TruncateDecimal()
	}
	return denomActualReward, nil
}

// CalculateUserRewards calculates the rewards for each user.
// It uses a map of denoms denoting the total rewards for each denom and multiplies that by the
// user's share of that denom (relative to totalLPV).
func (k Keeper) CalculateUserRewards(ctx sdk.Context, totalLPV sdk.Coins, denomActualReward map[string]sdk.Coins) types.UserInfoMap {
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
		udi.Weight = udi.Amt.ToDec().QuoInt(totalLPV.AmountOf(denom))
		udi.Reward, _ = sdk.NewDecCoinsFromCoins(denomActualReward[denom]...).MulDec(udi.Weight).TruncateDecimal()

		if _, exist := uim[userAcc]; !exist {
			uim[userAcc] = types.UserInfo{
				UserAcc:     userAcc,
				DenomMap:    map[string]types.UserDenomInfo{},
				TotalReward: sdk.NewCoins(),
			}
		}
		elem := uim[userAcc]
		elem.TotalReward = elem.TotalReward.Add(udi.Reward...)
		elem.DenomMap[denom] = udi
		uim[userAcc] = elem
	}
	return uim
}
