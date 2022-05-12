package keeper

import (
	"errors"
	"fmt"
	"math"
	"strconv"
	"time"

	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"

	qoracletypes "github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

// TODO - Need to optimize all these getters to reduce the KV store calls

// Get pool info
//lint:ignore U1000 Ignore unused function temporarily
func (k Keeper) getPoolInfo(ctx sdk.Context, poolID uint64) qoracletypes.PoolInfo {
	poolIDStr := strconv.FormatUint(poolID, 10)
	poolInfo, _ := k.qoracleKeeper.GetPoolInfo(ctx, poolIDStr)
	return poolInfo
}

// Get pool assets from pool ID
func (k Keeper) getPoolAssets(ctx sdk.Context, poolID uint64) (ps []gammtypes.PoolAsset) {
	poolIDStr := strconv.FormatUint(poolID, 10)
	poolInfo, _ := k.qoracleKeeper.GetPoolInfo(ctx, poolIDStr)
	return poolInfo.Info.PoolAssets
}

// Get APY ranked pool list
// func (k Keeper) getAPYRankedPools(ctx sdk.Context) (poolIDs []uint64) {
// TODO : Store the uint64 values inside the KV store
func (k Keeper) getAPYRankedPools(ctx sdk.Context) (poolIDs []string) {
	pr, _ := k.qoracleKeeper.GetPoolRanking(ctx)
	return pr.PoolIdsSortedByAPY
}

// Get APY ranked pool list
func (k Keeper) getTotalShare(ctx sdk.Context, poolID uint64) (totalShare sdk.Coin) {
	poolIDStr := strconv.FormatUint(poolID, 10)
	poolInfo, _ := k.qoracleKeeper.GetPoolInfo(ctx, poolIDStr)
	return poolInfo.Info.TotalShares
}

// Get the maximum available amount in the orion staking
func (k Keeper) getMaxAvailableAmount(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdk.Int {
	return k.GetStakingBalance(ctx, lockupPeriod, denom).Amount
}

// ExecuteMeissa iterate over all the meissa strategy registered with the orion vault
func (k Keeper) ExecuteMeissa(ctx sdk.Context, epochday uint64, lockupPeriod qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug("Entered ExecuteMeissa", "epochday", epochday, "lockupType", qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	var err error

	err = k.MeissaCoinDistribution(ctx, epochday, lockupPeriod)
	if err != nil {
		return err
	}

	err = k.MeissaExit(ctx, epochday, lockupPeriod)
	if err != nil {
		return err
	}

	err = k.MeissaWithdraw(ctx, epochday, lockupPeriod)
	if err != nil {
		return err
	}

	err = k.MeissaAuditorFunction(ctx, lockupPeriod)
	if err != nil {
		return err
	}

	// Claim reward.
	// TODO | AUDIT

	return nil
}

// MeissaCoinDistribution is Meissa algorithm to distribute coins among osmosis pools
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
func (k Keeper) MeissaCoinDistribution(ctx sdk.Context, epochDay uint64, lockupType qbanktypes.LockupTypes) error {
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

		err = k.SendCoinsFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coins)
		if err != nil {
			return err
		}
		// TODO | AUDIT
		//  1. Call Intergamm IBC token transfer from  OrionStakingMaccName
		//  2. New Multihop IBC token transfer to be used via token coin1, and coin2 origin chain

		if shareOutAmount.IsPositive() {
			// Call Intergamm Add Liquidity Method
			err := k.JoinPool(ctx, poolID, shareOutAmount, maxAvailableTokens)
			if err != nil {
				return err
			}

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

	return nil
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

// CalcUsersLPWeight calculate users deposited coin1 and coin2 amount for this epochday.
// Calculate percentage of users weight
// Logic -
// 1. Get the list of users and their deposited fund on the given epochday from bank module kv store.
// 2.
func (k Keeper) CalcUsersLPWeight(lp types.LpPosition) {

}

// GetLPBondingUnbondingPeriod does the Lockup period to LP bonding-unbonding logic.
// Logic
// 7 Day Lockup ->
// a. 7 day unbonding gauge with 1 day bonding and 7 days of unbonding. So for first day it will
// earn 7 day apy and for next 7 days it will earn 1 day apy.
// 14 days Lockup ->
// a. 7 days unbonding gauge with 7 day bonding and 7 day unbonding period. For the first 7 day it
// will earn 7 day apy for the first 7 day, and then earn 1 day apy for the next 7 days.
// 21 days Lockup ->
// a. 14 day bonding gauge with 7 days of bonding period and 14 days of unbonding period. So for first
// 7 days it will earn 14 day bonding and for the next 7 days it will earn 7 day apy and for next 7 days it will
// earn 1 day apy.
// Note - Initially done for only 7 days and 21 days
// Return - unbondingPeriod signifies the gauge for which to lock lp tokens for.
func (k Keeper) GetLPBondingUnbondingPeriod(lockupType qbanktypes.LockupTypes) (bondingPeriod uint64, unbondingPeriod uint64) {
	switch lockupType {
	case qbanktypes.LockupTypes_Days_7:
		bondingPeriod = 1
		unbondingPeriod = 7
	case qbanktypes.LockupTypes_Days_21:
		bondingPeriod = 7
		unbondingPeriod = 14
	default:
		// Also include invalid type
		bondingPeriod = 0
		unbondingPeriod = 0
	}
	return bondingPeriod, unbondingPeriod
}

// MeissaExit checks for exit pool conditions for the meissa strategy.
// Param - epochday is current epoch day
// Logic -
// If the strategy did deploy any position lockup period ago ( say 7 day ago) then
// Use the [ currentday - lockupPeriodDays ] as key for epoch
// Get the pool ids and sdk.coins.
// Calls exit for the pools.
func (k Keeper) MeissaExit(ctx sdk.Context, currEpochday uint64, lockupType qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug("Entered MeissaExit", "currEpochday", currEpochday, "lockupType", qbanktypes.LockupTypes_name[int32(lockupType)])
	// TODO : We can use a different KV store and cache for the list of Currently active pools.
	// Currently active pool are those in which orion has LPing positions.
	poolIDs := k.getAPYRankedPools(ctx)
	offsetEpochDay := currEpochday - uint64(lockupType)
	for _, poolIDStr := range poolIDs {
		poolID, _ := strconv.ParseUint(poolIDStr, 10, 64)
		coins := k.GetMeissaEpochLockupPoolPosition(ctx, offsetEpochDay, lockupType, poolID)
		k.Logger(ctx).Debug("MeissaExit", "currEpochday", currEpochday, "offsetEpochDay", offsetEpochDay, "poolID", poolID, "coins", coins)
		poolTotalShare := k.getTotalShare(ctx, poolID)
		assets := k.getPoolAssets(ctx, poolID)
		var shareInAmount sdk.Int
		var tokenOutMins []sdk.Coin // TODO | AUDIT | Zero value
		for _, asset := range assets {

			shareInAmount = poolTotalShare.Amount.Quo(asset.Token.Amount)
			break
		}

		if shareInAmount.IsPositive() {
			// Call intergamm exit pool method
			err := k.ExitPool(ctx, poolID, shareInAmount, tokenOutMins)
			if err != nil {
				return err
			}
		}

	}

	return nil
}

// MeissaWithdraw checks for exit pool conditions for the meissa strategy.
// Logic -
// If the strategy did exit any position lockup period ago ( say 7 day ago) then
//  call withdraw which will initial IBC transfer from escrow account to strategy account
// Note - Orion may not need this func; withdrawal can be handled in join pool
func (k Keeper) MeissaWithdraw(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes) error {
	return nil
}

// MeissaAuditorFunction audit the positions and KV stores for any unused or leaked amount.
// If any leaked or unused coin found then it should be used.
// Logic :
// 1. check the coins available in all the orion lockup accounts at today epochday.
// 2. transfer coins to the orion treasury. Orion treasury will also be used during users withdrawal.
// 3. a secondary strategy to be implemented to use the leftover coins
func (k Keeper) MeissaAuditorFunction(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug("Entered MeissaAuditorFunction", "lockupType", qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	coins := k.GetAllStakingBalances(ctx, lockupPeriod)
	k.Logger(ctx).Debug("MeissaAuditorFunction", "coins", coins)

	return k.SendCoinsFromModuleToReserve(ctx, types.CreateOrionStakingMaccName(lockupPeriod), coins)
}

//
func (k Keeper) SetMeissaEpochLockupPoolPosition(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes, poolID uint64, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.MeissaStrategyPoolPosKBP)
	key := types.CreateMeissaPoolPositionKey(epochday, lockupType, poolID)
	var qcoins qbanktypes.QCoins
	// TODO | AUDIT | Check for the slice copy/pointers
	qcoins.Coins = coins
	value := k.cdc.MustMarshal(&qcoins)
	store.Set(key, value)
}

//
func (k Keeper) GetMeissaEpochLockupPoolPosition(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes, poolID uint64) sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.MeissaStrategyPoolPosKBP)
	key := types.CreateMeissaPoolPositionKey(epochday, lockupType, poolID)
	b := store.Get(key)
	var qcoins qbanktypes.QCoins
	k.cdc.MustUnmarshal(b, &qcoins)
	// TODO | AUDIT Check for the slice/pointers
	return qcoins.Coins
}

// Intergamm module method wrappers
func (k Keeper) JoinPool(ctx sdk.Context, poolID uint64, shareOutAmount sdk.Int, tokenInMaxs []sdk.Coin) error {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|poolID=%v|shareOutAmount=%v|tokenInMaxs=%v\n",
		poolID, shareOutAmount, tokenInMaxs))

	owner := ""
	connectionId := ""
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()

	err := k.intergammKeeper.TransmitIbcJoinPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareOutAmount,
		tokenInMaxs,
	)

	return err
}

func (k Keeper) ExitPool(ctx sdk.Context, poolID uint64, shareInAmount sdk.Int, tokenOutMins []sdk.Coin) error {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|poolID=%v|shareInAmount=%v|tokenOutMins=%v\n",
		poolID, shareInAmount, tokenOutMins))

	owner := ""
	connectionId := ""
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()

	err := k.intergammKeeper.TransmitIbcExitPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareInAmount,
		tokenOutMins,
	)

	return err
}

func (k Keeper) TokenWithdrawFromOsmosis(ctx sdk.Context, receiverAddr string, coins []sdk.Coin) error {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|receiverAddr=%v|coins=%v\n",
		receiverAddr, coins))

	owner := ""
	connectionId := ""
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()
	transferPort := "transfer"
	transferChannel := "channel-1"
	token := sdk.NewCoin("uatom", sdk.NewInt(10))

	err := k.intergammKeeper.TransmitIbcTransfer(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		transferPort,
		transferChannel,
		token,
		receiverAddr,
		ibcclienttypes.ZeroHeight(),
		uint64(timeoutTimestamp),
	)

	return err

}
