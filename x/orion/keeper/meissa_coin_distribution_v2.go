package keeper

import (
	"errors"
	"fmt"
	"math"
	"strconv"

	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// MeissaCoinDistributionV2 is Meissa algorithm to distribute coins among osmosis pools
//// Logic -
//// 1. Get the list of pools with APY ranks from the oracle module.
//// 2. Iterate apy_ranked_pools with highest apy pool picked first.
//// 3. Get the list of pool assets.
//// 4. Collect the max available tokens (corresponding to the pool assets) from the Orion module staking pool.
//// 5. Calculate the max share (shareOutAmount) that can be obtained in all-asset deposit mode.
//// 6. Calculate the coins needed to obtain the shareOutAmount
//// 7. Send the coins using IBC call to osmosis from the quasar custom sender module account (intergamm module.)
//// 8. Provide liquidity to osmosis via IBC for this pool.
//// 9. TODO [1] Calculate user lp share amount for this new lp position.
//// 10. TODO [2] Create an lp position object for this LP activity.
//// 11. Update chain state to reduce staking pool amount for the coins.
//// 12. Update the amount deployed on osmosis in the appropriate KV store.
//// Go to the next pool and repeat [3 - 12]
// NOTE - At the end of the iterations; the quasar Orion staking account may still have a sufficient amount of
// denoms for which we don't have pool pairs.
func (k Keeper) MeissaCoinDistributionV2(ctx sdk.Context, epochDay uint64, lockupType qbanktypes.LockupTypes) {
	k.Logger(ctx).Debug(fmt.Sprintf("Entered MeissaCoinDistribution|epochDay=%v|lockupType=%v\n",
		epochDay, qbanktypes.LockupTypes_name[int32(lockupType)]))

	poolIDs := k.getAPYRankedPools(ctx)

	k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|epochDay=%v|lockupType=%v|poolIds=%v\n",
		epochDay, qbanktypes.LockupTypes_name[int32(lockupType)], poolIDs))
	for _, poolIDStr := range poolIDs {
		// TODO | Refactoring | Change the qoracle pool ID storage to uint64
		poolID, _ := strconv.ParseUint(poolIDStr, 10, 64)
		poolAssets := k.getPoolAssets(ctx, poolID)
		if len(poolAssets) != 2 {
			// Initially strategy want to LP only in the pool with 2 assets
			continue
		}

		poolTotalShare := k.getTotalShare(ctx, poolID)
		k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|epochDay=%v|lockupType=%v|poolId=%v|share=%v|poolAssets=%v\n",
			epochDay, qbanktypes.LockupTypes_name[int32(lockupType)], poolID, poolTotalShare, poolAssets))

		maxAvailableTokens := k.GetMaxAvailableTokensCorrespondingToPoolAssets(ctx, lockupType, poolAssets)
		shareOutAmount, err := ComputeShareOutAmount(poolTotalShare.Amount, poolAssets, maxAvailableTokens)
		if err != nil {
			continue
		}
		k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|shareOutAmount=%v\n",
			shareOutAmount))

		// Transfer fund to the strategy global account.
		coins, err := ComputeNeededCoins(poolTotalShare.Amount, shareOutAmount, poolAssets)
		if err != nil {
			continue
		}
		k.SendCoinsFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coins)

		// TODO | AUDIT
		//  1. Call Intergamm IBC token transfer from  OrionStakingMaccName
		//  2. New Multihop IBC token transfer to be used via token coin1, and coin2 origin chain

		if shareOutAmount.IsPositive() {
			// Call Intergamm Add Liquidity Method
			k.JoinPool(ctx, poolID, shareOutAmount, maxAvailableTokens)

			// TODO : Lock the LP tokens and receive lockId.
			// TODO : Update orion vault staking amount.
			// Most probably not needed as balance in the orion vault is already updated.

			// coins := sdk.NewCoins(coin1, coin2)
			k.SetMeissaEpochLockupPoolPosition(ctx, epochDay, lockupType, poolID, coins)

			bonding, unbonding := k.GetLPBondingUnbondingPeriod(lockupType)
			bondingStartEpochDay := epochDay
			unbondingStartEpochDay := bondingStartEpochDay + bonding
			var lockupID uint64   // TODO : To be received from osmosis
			var lpTokens sdk.Coin // TODO : To be received from osmosis
			lp := NewLP(lockupID, bondingStartEpochDay, bonding,
				unbondingStartEpochDay, unbonding, poolID, lpTokens, coins)

			k.AddNewLPPosition(ctx, lp)
		}
	}
}

// GetMaxAvailableTokensCorrespondingToPoolAssets gets the max available amount (in Orion staking account) of all denoms
// that are in the poolAssets as a sdk.Coins object
func (k Keeper) GetMaxAvailableTokensCorrespondingToPoolAssets(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, poolAssets []gammtypes.PoolAsset) (res sdk.Coins) {
	for _, asset := range poolAssets {
		denom := asset.Token.GetDenom()
		res = res.Add(sdk.NewCoin(denom, k.getMaxAvailableAmount(ctx, lockupPeriod, denom)))
	}
	return res
}

// ComputeShareOutAmount computes the max number of shares that can be obtained given the maxAvailableTokens (in all-asset deposit mode)
func ComputeShareOutAmount(totalSharesAmt sdk.Int, poolAssets []gammtypes.PoolAsset, maxAvailableTokens sdk.Coins) (sdk.Int, error) {
	if len(poolAssets) == 0 {
		return sdk.ZeroInt(), errors.New("error: empty pool assets")
	}
	shareOutAmount := sdk.NewInt(math.MaxInt64)
	for _, asset := range poolAssets {
		if asset.Token.Amount.IsZero() {
			return sdk.ZeroInt(), errors.New("error: zero asset amount")
		}
		share := totalSharesAmt.Mul(maxAvailableTokens.AmountOf(asset.Token.GetDenom())).Quo(asset.Token.Amount)
		if share.LT(shareOutAmount) {
			shareOutAmount = share
		}
	}
	return shareOutAmount, nil
}

// ComputeNeededCoins computes the coins needed to obtain shareOutAmount
func ComputeNeededCoins(totalSharesAmount, shareOutAmount sdk.Int, poolAssets []gammtypes.PoolAsset) (sdk.Coins, error) {
	res := sdk.NewCoins()
	if totalSharesAmount.IsZero() && len(poolAssets) > 0 {
		return res, errors.New("error: zero totalSharesAmount and non-empty poolAssets are illogical")
	}
	for _, asset := range poolAssets {
		res = res.Add(sdk.NewCoin(asset.Token.GetDenom(), shareOutAmount.Mul(asset.Token.Amount).Quo(totalSharesAmount)))
	}
	return res, nil
}
