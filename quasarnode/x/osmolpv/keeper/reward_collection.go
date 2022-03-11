package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetRewardCollection set rewardCollection in the store
func (k Keeper) SetRewardCollection(ctx sdk.Context, epochday uint64, rewardCollection types.RewardCollection) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.RewardCollectionKBP)
	key := types.CreateEpochRewardKey(epochday)
	b := k.cdc.MustMarshal(&rewardCollection)
	store.Set(key, b)
}

// GetRewardCollection returns rewardCollection
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

// LPPositionReward calculate the reward associated with a particulat LP positon
func (k Keeper) LPPositionReward(ctx sdk.Context, epochday uint64, lpID uint64) (sdk.Coins, error) {
	totalReward, found := k.GetRewardCollection(ctx, epochday)
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
	}
	return result, nil
}

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

// LPExpectedReward calculate per day expected reward based on current values of gauge and apy
func (k Keeper) LPExpectedReward(ctx sdk.Context, lpID uint64) sdk.Coins {
	var reward sdk.Coins
	g := k.GetCurrentActiveGauge(lpID)
	RewardMuliplier := g.ExpectedApy.QuoInt64(types.NumDaysPerYear)
	epochday, efound := k.GetLPEpochDay(ctx, lpID)
	if efound {
		lp, lpfound := k.GetLpPosition(ctx, epochday, lpID)
		if lpfound {
			reward = lp.Coins
			for i, coin := range lp.Coins {
				reward[i].Amount = coin.Amount.ToDec().Mul(RewardMuliplier).TruncateInt()
			}
		}
	}
	return reward
}

// Notes
// - LPExpectedReward and LPPositionReward should not differ by a large amount
// - LPExpectedReward can be used by cross validation and fulfiling users expectation in
//   case of destination chain failure/halt
// - TODO
