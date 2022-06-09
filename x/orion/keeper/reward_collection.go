package keeper

import (
	"errors"
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
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

	totalLPV := k.CalculateTotalLPV(ctx, epochDay-1) // Total LP in deployment

	denomWeights, err := k.CalculateDenomLPWeights(ctx, totalLPV)
	if err != nil {
		return err
	}
	denomActualReward, err := CalculateActualRewardForEachDenom(rc.Coins, denomWeights)
	if err != nil {
		return err
	}

	activeUserDepositsMap := k.qbankKeeper.GetAllActiveUserDeposits(ctx, epochDay)
	uim := CalculateUserRewards(activeUserDepositsMap, totalLPV, denomActualReward)

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

// CalculateDenomLPWeights calculates the share of the fiat value of each denom from the total fiat value of totalLPV.
// Fiat value is computed for the present time.
// The weights are later used to calculate the share of rewards for each denom.
func (k Keeper) CalculateDenomLPWeights(ctx sdk.Context, totalLPV sdk.Coins) (map[string]sdk.Dec, error) {
	weights := make(map[string]sdk.Dec)
	totalEquivalentReceipt, err := k.GetTotalOrions(ctx, totalLPV)
	if err != nil {
		return weights, err
	}
	for _, coin := range totalLPV {
		equivalentReceipt, err := k.CalcReceipts(ctx, coin)
		if err != nil {
			return weights, err
		}
		weights[coin.Denom] = equivalentReceipt.Amount.ToDec().Quo(totalEquivalentReceipt.Amount.ToDec())
	}
	return weights, nil
}

// CalculateActualRewardForEachDenom calculates the share of each denom from the rewards according to weights.
func CalculateActualRewardForEachDenom(netRewards sdk.Coins, weights map[string]sdk.Dec) (map[string]sdk.Coins, error) {
	// Validate weights
	sumWeights := sdk.ZeroDec()
	for _, w := range weights {
		sumWeights = sumWeights.Add(w)
	}
	if sumWeights.GT(sdk.OneDec()) {
		return nil, errors.New("error: sum of weights should not be greater than 1")
	}
	denomActualReward := make(map[string]sdk.Coins)
	// Fill the denomActualRewardMap
	for denom, weight := range weights {
		denomActualReward[denom], _ = sdk.NewDecCoinsFromCoins(netRewards...).MulDec(weight).TruncateDecimal()
	}
	return denomActualReward, nil
}

// CalculateUserRewards calculates the rewards for each user.
// It uses a map of denoms denoting the total rewards for each denom and multiplies that by the
// user's share of that denom (relative to totalLPV).
func CalculateUserRewards(activeUserDepositsMap map[string]sdk.Coins, totalLPV sdk.Coins, denomActualReward map[string]sdk.Coins) types.UserInfoMap {
	//////////////////////////////
	// Get users denom reward
	uim := make(types.UserInfoMap)
	for depositorAccAddress, totalActiveDeposits := range activeUserDepositsMap {
		totalReward := sdk.NewCoins()
		userDenomInfoMap := make(map[string]types.UserDenomInfo)
		for _, coin := range totalActiveDeposits {
			denom := coin.Denom
			weight := coin.Amount.ToDec().QuoInt(totalLPV.AmountOf(denom))
			reward, _ := sdk.NewDecCoinsFromCoins(denomActualReward[denom]...).MulDec(weight).TruncateDecimal()
			totalReward = totalReward.Add(reward...)
			userDenomInfoMap[denom] = types.UserDenomInfo{
				Denom:  denom,
				Weight: weight,
				Amt:    coin.Amount,
				Reward: reward,
			}
		}
		uim[depositorAccAddress] = types.UserInfo{
			UserAcc:     depositorAccAddress,
			DenomMap:    userDenomInfoMap,
			TotalReward: totalReward,
		}
	}
	return uim
}
