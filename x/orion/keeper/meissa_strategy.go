package keeper

import (
	"errors"
	"fmt"
	"math"
	"strconv"

	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"

	sdkmath "cosmossdk.io/math"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Get the maximum available amount in the orion staking.
// Input denom is osmosis equivalent denom,
func (k Keeper) getMaxAvailableAmount(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, denom string) sdkmath.Int {
	wdenoms := k.qbankKeeper.WhiteListedDenomsInOrion(ctx)
	for _, v := range wdenoms {
		if v.OnehopOsmo == denom {
			return k.GetStakingBalance(ctx, lockupPeriod, v.OnehopQuasar).Amount
		}
	}
	return sdk.ZeroInt()
}

// ExecuteMeissa iterate over all the meissa strategy registered with the orion vault
func (k Keeper) ExecuteMeissa(ctx sdk.Context, epochday uint64, lockupPeriod qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug("Entered ExecuteMeissa",
		"epochday", epochday,
		"lockupType", qbanktypes.LockupTypes_name[int32(lockupPeriod)])
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
// // Logic -
// // 1. Get the list of pools with APY ranks from the oracle module.
// // 2. Iterate apy_ranked_pools with highest apy pool picked first.
// // 3. Get the list of pool assets.
// // 4. Collect the max available tokens (corresponding to the pool assets) from the Orion module staking pool.
// // 5. Calculate the max share (shareOutAmount) that can be obtained in all-asset deposit mode.
// // 6. Calculate the coins needed to obtain the shareOutAmount
// // 7. Send the coins using IBC call to osmosis from the quasar custom sender module account (intergamm module.)
// // 8. Provide liquidity to osmosis via IBC for this pool.
// // 9. TODO [1] Calculate user lp share amount for this new lp position.
// // 10. TODO [2] Create an lp position object for this LP activity.
// // 11. Update chain state to reduce staking pool amount for the coins.
// // 12. Update the amount deployed on osmosis in the appropriate KV store.
// // Go to the next pool and repeat [3 - 12]
// NOTE - At the end of the iterations; the quasar Orion staking account may still have a sufficient amount of
// denoms for which we don't have pool pairs.
func (k Keeper) MeissaCoinDistribution(ctx sdk.Context, epochDay uint64, lockupType qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug(fmt.Sprintf("Entered MeissaCoinDistribution|epochDay=%v|lockupType=%v\n",
		epochDay, qbanktypes.LockupTypes_name[int32(lockupType)]))

	pools := k.qoracleKeeper.GetPoolsRankedByAPY(ctx, "")
	k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|epochDay=%v|lockupType=%v|poolsCount=%v\n",
		epochDay, qbanktypes.LockupTypes_name[int32(lockupType)], len(pools)))
	for _, pool := range pools {
		if len(pool.Assets) != 2 {
			// Initially strategy want to LP only in the pool with 2 assets
			continue
		}

		osmosisPool := pool.Raw.GetCachedValue().(gammbalancer.Pool)

		k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|epochDay=%v|lockupType=%v|poolId=%v|share=%v|poolAssets=%v\n",
			epochDay, qbanktypes.LockupTypes_name[int32(lockupType)], osmosisPool.Id, osmosisPool.TotalShares, osmosisPool.PoolAssets))

		maxAvailableTokens := k.GetMaxAvailableTokensCorrespondingToPoolAssets(ctx, lockupType, osmosisPool.PoolAssets)
		shareOutAmount, err := ComputeShareOutAmount(osmosisPool.TotalShares.Amount, osmosisPool.PoolAssets, maxAvailableTokens)
		if err != nil {
			continue
		}

		k.Logger(ctx).Debug(fmt.Sprintf("MeissaCoinDistribution|shareOutAmount=%v\n",
			shareOutAmount))

		coins, err := ComputeNeededCoins(osmosisPool.TotalShares.Amount, shareOutAmount, osmosisPool.PoolAssets)
		if err != nil {
			continue
		}

		// AUDIT TODO - Cross validation if funds are available in AvailableInterchainFundKBP

		if shareOutAmount.IsPositive() {
			packetSeq, err := k.JoinPool(ctx, osmosisPool.Id, shareOutAmount, maxAvailableTokens)
			if err != nil {
				return err
			}
			k.OnJoinSend(ctx, packetSeq, epochDay, osmosisPool.Id, osmosisPool.TotalShares.Denom, coins, shareOutAmount, lockupType)
			k.SubAvailableInterchainFund(ctx, coins)
		}
	}
	return nil
}

func (k Keeper) OnJoinSend(ctx sdk.Context,
	packetSeq uint64,
	epochDay uint64,
	poolID uint64,
	poolShareDenom string,
	coins sdk.Coins,
	shareOutAmount sdkmath.Int,
	lockupType qbanktypes.LockupTypes) {
	k.SetMeissaEpochLockupPoolPosition(ctx, epochDay, lockupType, poolID, coins)

	bonding, unbonding := k.GetLPBondingUnbondingPeriod(lockupType)
	bondingStartEpochDay := epochDay
	unbondingStartEpochDay := bondingStartEpochDay + bonding
	var lockupID uint64
	lpTokens := sdk.NewCoin(poolShareDenom, shareOutAmount)
	lp := k.NewLP(lockupID, bondingStartEpochDay, bonding,
		unbondingStartEpochDay, unbonding, poolID,
		types.LpState_JOINING, lpTokens, coins)
	lp.SeqNo = packetSeq
	lpID := k.AddNewLPPosition(ctx, lp)
	k.SetSeqNumber(ctx, lp.SeqNo, lpID)
}

// GetMaxAvailableTokensCorrespondingToPoolAssets gets the max available amount (in Orion staking account) of all denoms
// that are in the poolAssets as a sdk.Coins object
func (k Keeper) GetMaxAvailableTokensCorrespondingToPoolAssets(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, poolAssets []gammbalancer.PoolAsset) (res sdk.Coins) {
	for _, asset := range poolAssets {
		denom := asset.Token.GetDenom()
		res = res.Add(sdk.NewCoin(denom, k.getMaxAvailableAmount(ctx, lockupPeriod, denom)))
	}
	return res
}

// ComputeShareOutAmount computes the max number of shares that can be obtained given the maxAvailableTokens (in all-asset deposit mode)
func ComputeShareOutAmount(totalSharesAmt sdkmath.Int, poolAssets []gammbalancer.PoolAsset, maxAvailableTokens sdk.Coins) (sdkmath.Int, error) {
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
func ComputeNeededCoins(totalSharesAmount, shareOutAmount sdkmath.Int, poolAssets []gammbalancer.PoolAsset) (sdk.Coins, error) {
	res := sdk.NewCoins()
	if totalSharesAmount.IsZero() && len(poolAssets) > 0 {
		return res, errors.New("error: zero totalSharesAmount and non-empty poolAssets are illogical")
	}
	for _, asset := range poolAssets {
		res = res.Add(sdk.NewCoin(asset.Token.GetDenom(), shareOutAmount.Mul(asset.Token.Amount).Quo(totalSharesAmount)))
	}
	return res, nil
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
// Logic -
// 1. Iterate over all active pool positions
// 2. if current lp exit condition matched then calculate the tokenOutMins based on shareInAmount
// 3. Call Intergamm exit.
// 4. TODO - Future. To be more precise do ibc query to determine current balance of interchain fund.
// 5. One idea is to use new interchain account for each new lp position. That will help proper accounting of lp positions and pnl.

func (k Keeper) MeissaExit(ctx sdk.Context, epochDay uint64, lockupType qbanktypes.LockupTypes) error {
	k.Logger(ctx).Debug("Entered MeissaExit",
		"currEpochday", epochDay, "lockupType", qbanktypes.LockupTypes_name[int32(lockupType)])
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	// key - {epochday} + {":"} + {LPID}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		lpEpochDayStr := string(splits[0])
		lpEpochDay, _ := strconv.ParseUint(lpEpochDayStr, 10, 64)
		lpIDStr := string(splits[1])
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)
		lp, _ := k.GetLpPosition(ctx, lpEpochDay, lpID)
		lpEndDay := lp.BondingStartEpochDay + lp.BondDuration + lp.UnbondingDuration
		shareInAmount := lp.Lptoken.Amount
		var tokenOutMins []sdk.Coin // TODO AUDIT | Can be empty
		if epochDay > lpEndDay {
			// Need to exit today
			seq, err := k.ExitPool(ctx, lp.PoolID, shareInAmount, tokenOutMins)
			if err != nil {
				return err
			}
			k.SetSeqNumber(ctx, seq, lpID)
		}
	}

	return nil
}

// MeissaWithdraw checks for exit pool conditions for the meissa strategy.
// Logic -
// If the strategy did exit any position lockup period ago ( say 7 day ago) then
//
//	call withdraw which will initial IBC transfer from escrow account to strategy account
//
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

func (k Keeper) SetMeissaEpochLockupPoolPosition(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes, poolID uint64, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.MeissaStrategyPoolPosKBP)
	key := types.CreateMeissaPoolPositionKey(epochday, lockupType, poolID)
	var qcoins qbanktypes.QCoins
	qcoins.Coins = coins
	value := k.cdc.MustMarshal(&qcoins)
	store.Set(key, value)
}

func (k Keeper) GetMeissaEpochLockupPoolPosition(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes, poolID uint64) sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.MeissaStrategyPoolPosKBP)
	key := types.CreateMeissaPoolPositionKey(epochday, lockupType, poolID)
	b := store.Get(key)
	var qcoins qbanktypes.QCoins
	k.cdc.MustUnmarshal(b, &qcoins)
	return qcoins.Coins
}

// computeTokenOutAmount calculate the token out amount from the recent values from the pool total share.
func (k Keeper) computeTokenOutAmount(ctx sdk.Context, shareInAmount sdkmath.Int, poolID uint64) sdk.Coins {
	pool, _ := k.qoracleKeeper.GetPool(ctx, strconv.FormatUint(poolID, 10))
	osmosisPool := pool.Raw.GetCachedValue().(gammbalancer.Pool)
	totalShare := osmosisPool.TotalShares
	assets := osmosisPool.PoolAssets
	if len(assets) != 2 {
		return sdk.NewCoins()
	}
	coin1 := sdk.NewCoin(assets[0].Token.Denom,
		sdk.NewDecFromInt(shareInAmount).Quo(sdk.NewDecFromInt(totalShare.Amount)).TruncateInt())
	coin2 := sdk.NewCoin(assets[1].Token.Denom,
		sdk.NewDecFromInt(shareInAmount).Quo(sdk.NewDecFromInt(totalShare.Amount)).TruncateInt())

	return sdk.NewCoins(coin1, coin2)
}

func (k Keeper) LockLPTokensIfAny(ctx sdk.Context) {
	// This method probably shouldn't be needed, but just an idea if can be implemented as a
	// validation mechanism.
	// Iterate and check if there is any lp positions which is not yet locked.
	// If found any such lp positions then call lock lp tokens.
}
