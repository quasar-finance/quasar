package keeper

/*
	"fmt"
	"github.com/abag/quasarnode/x/osmolpv/types"

	"github.com/cosmos/cosmos-sdk/store/prefix"
*/
import (
	"fmt"
	"strconv"
	"time"

	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"

	qoracletypes "github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

// TODO - Need to optimize all these getters to reduce the KV store calls

// Get pool info
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
	//return sdk.ZeroInt()
}

// ExecuteMeissa iterate over all the meissa strategy registered with the orion vault
func (k Keeper) ExecuteMeissa(ctx sdk.Context, epochday uint64, lockupPeriod qbanktypes.LockupTypes) {

	k.Logger(ctx).Info(fmt.Sprintf("Entered ExecuteMeissa|epochday=%v|lockupType=%v\n",
		epochday, qbanktypes.LockupTypes_name[int32(lockupPeriod)]))

	strategies, _ := k.GetSubStrategyNames(ctx, types.MeissaStrategyName)

	// Join pool
	for _, sn := range strategies.Names {
		k.MeissaCoinDistribution(ctx, epochday, types.MeissaStrategiesLockup[sn])
	}

	// Exit pool
	for _, sn := range strategies.Names {
		k.MeissaExit(ctx, epochday, types.MeissaStrategiesLockup[sn])
		k.MeissaWithdraw(ctx, epochday, types.MeissaStrategiesLockup[sn])
	}

	// Withdraw from osmosis chain
	//for _, sn := range strategies.Names {
	//	k.MeissaWithdraw(ctx, epochday, types.MeissaStrategiesLockup[sn])
	// }

	// Audit
	for _, sn := range strategies.Names {
		k.MeissaAudiorFunction(ctx, types.MeissaStrategiesLockup[sn])
	}
}

// MeissaCoinDistribution is Meissa algorithm to distribute coins among osmosis pools
// Logic -
// Get the list of pools with APY ranks from the oracle module.
// Iterate apy_ranked_pools with highest apy pool picked first
// Get the list of denoms from the current pool - Denom1, Denom2, and pool denom ratio.
// Collect the max possible amount from both denom 1 and denom 2 from the Orion module staking pool.
// Send the coins using IBC call to osmosis from the quasar custom sender module account ( intergamm module.)
// Provide liquidity to osmosis via IBC for this pool.
// Update chain state to reduce staking pool amount for both the denom.
// Update the amount deployed on osmosis in the appropriate KV store.
// Go to the next pool and repeat [A - F]
// At the end of the iterations; the quasar Orion staking account may still have a sufficient amount of denoms for which we don't have pool pairs. We can put them in Orion reserve or use osmosis single denom pool staking which internally swaps half of the denom amount of the paired pool denom. It will charge a swap fee, however.

func (k Keeper) MeissaCoinDistribution(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes) {

	k.Logger(ctx).Info(fmt.Sprintf("Entered MeissaCoinDistribution|epochday=%v|lockupType=%v\n",
		epochday, qbanktypes.LockupTypes_name[int32(lockupType)]))

	poolIDs := k.getAPYRankedPools(ctx)

	k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|epochday=%v|lockupType=%v|poolIds=%v\n",
		epochday, qbanktypes.LockupTypes_name[int32(lockupType)], poolIDs))
	for _, poolIDStr := range poolIDs {
		// TODO - Change the qoracle pool ID storage to uint64
		poolID, _ := strconv.ParseUint(poolIDStr, 10, 64)
		assets := k.getPoolAssets(ctx, poolID)
		if len(assets) != 2 {
			// Initially strategy want to LP only in the pool with 2 assets
			continue
		}

		poolTotalShare := k.getTotalShare(ctx, poolID)
		k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|epochday=%v|lockupType=%v|poolId=%v|share=%v|poolAssets=%v\n",
			epochday, qbanktypes.LockupTypes_name[int32(lockupType)], poolID, poolTotalShare, assets))
		var sharePerAssetAmount []sdk.Int
		var shareRequired []sdk.Int
		var maxAvailableAmount []sdk.Int

		for idx, asset := range assets {

			//k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|sharePerAssetAmount[idx]=%v|poolTotalShare.Amount=%v|asset.Token.Amount=%v\n",
			//	sharePerAssetAmount[idx], poolTotalShare.Amount, asset.Token.Amount))

			//sharePerAssetAmount[idx] = poolTotalShare.Amount.Quo(asset.Token.Amount)
			//maxAvailableAmount[idx] = k.getMaxAvailableAmount(ctx, lockupType, asset.Token.Denom)
			// shareRequired[idx] = maxAvailableAmount[idx].Mul(sharePerAssetAmount[idx])
			// TODO | AUDIT
			sharePerAssetAmount = append(sharePerAssetAmount, poolTotalShare.Amount, asset.Token.Amount)
			maxAvailableAmount = append(maxAvailableAmount, k.getMaxAvailableAmount(ctx, lockupType, asset.Token.Denom))
			shareRequired = append(shareRequired, maxAvailableAmount[idx].Mul(sharePerAssetAmount[idx]))

			k.Logger(ctx).Info(
				fmt.Sprintf(
					"MeissaCoinDistribution|epochday=%v|lockupType=%v|poolId=%v|asset=%v|"+
						"sharePerAssetAmount=%v|maxAvailableAmount=%v|shareRequired=%v|\n",
					epochday, qbanktypes.LockupTypes_name[int32(lockupType)], poolID,
					asset, sharePerAssetAmount[idx], maxAvailableAmount[idx], shareRequired[idx]))

		}

		// TODO | AUDIT | Code optimization
		// Calculate required amount for second denom based on first denom.
		RequiredSecondDenom := shareRequired[0].Quo(sharePerAssetAmount[1])

		var FirstAssetAmount sdk.Int
		var SecondAssetAmount sdk.Int
		var shareOutAmount sdk.Int
		if maxAvailableAmount[1].GT(RequiredSecondDenom) {
			// Consider this amounts for LPing based on the share required for first asset
			// Use shareRequired[0]
			shareOutAmount = shareRequired[0]
			FirstAssetAmount = shareRequired[0].Mul(sharePerAssetAmount[0])
			SecondAssetAmount = shareRequired[0].Mul(sharePerAssetAmount[1])
		} else {
			// Consider this amounts for LPing based on the share required for second asset
			// Use shareRequired[1]
			shareOutAmount = shareRequired[1]
			FirstAssetAmount = shareRequired[1].Mul(sharePerAssetAmount[0])
			SecondAssetAmount = shareRequired[1].Mul(sharePerAssetAmount[1])

		}

		k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|shareOutAmount=%v|FirstAssetAmount=%v|SecondAssetAmount=%v\n",
			shareOutAmount, FirstAssetAmount, SecondAssetAmount))

		// Transfer fund to the strategy global account.
		// TODO - 1. Optimize it to have one call only

		coin1 := sdk.NewCoin(assets[0].Token.Denom, FirstAssetAmount)
		coin2 := sdk.NewCoin(assets[1].Token.Denom, SecondAssetAmount)
		k.SendCoinFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coin1)
		k.SendCoinFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coin2)
		tokenInMaxs := []sdk.Coin{coin1, coin2}

		// TODO : Call Intergamm IBC token transfer
		if shareOutAmount.IsPositive() {
			// Call Intergamm Add Liquidity Method
			k.JoinPool(ctx, poolID, shareOutAmount, tokenInMaxs)

			// TODO : Update orion vault staking amount.
			// Most probably not needed as balance in the orion vault is already updated.

			// TODO : If sorted coins is required.
			coins := sdk.NewCoins(coin1, coin2)
			k.SetMeissaEpochLockupPoolPosition(ctx, epochday, lockupType, poolID, coins)
		}
	}

}

// MeissaExit checks for exit pool conditions for the meissa strategy.
// Param - epochday is current epoch day
// Logic -
// If the strategy did deploy any position lockup period ago ( say 7 day ago) then
// Use the [ currentday - lockupPeriodDays ] as key for epoch
// Get the pool ids and sdk.coins.
// Call exit for the pool.
func (k Keeper) MeissaExit(ctx sdk.Context, currEpochday uint64, lockupType qbanktypes.LockupTypes) {
	k.Logger(ctx).Info(fmt.Sprintf("Entered MeissaExit|currEpochday=%v|lockupType=%v\n",
		currEpochday, qbanktypes.LockupTypes_name[int32(lockupType)]))
	// TODO : We can use a different KV store and cache for the list of Currently active pools.
	// Currently active pool are those in which orion has LPing positions.
	poolIDs := k.getAPYRankedPools(ctx)
	offsetedEpochDay := currEpochday - uint64(lockupType)
	for _, poolIDStr := range poolIDs {
		poolID, _ := strconv.ParseUint(poolIDStr, 10, 64)
		coins := k.GetMeissaEpochLockupPoolPosition(ctx, offsetedEpochDay, lockupType, poolID)
		k.Logger(ctx).Info(fmt.Sprintf("MeissaExit|currEpochday=%v|offsetedEpochDay=%v|PoolID=%v|Coins=%v\n",
			currEpochday, offsetedEpochDay, poolID, coins))
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
			k.ExitPool(ctx, poolID, shareInAmount, tokenOutMins)
		}

	}

}

// MeissaWithdraw checks for exit pool conditions for the meissa strategy.
// Logic -
// If the strategy did exited any position lockup period ago ( say 7 day ago) then
//  call withdraw which will initial IBC transfer from escrow account to strategy account
// Note - Orion may not need this func; withdrawal can be handled in join pool
func (k Keeper) MeissaWithdraw(ctx sdk.Context, epochday uint64, lockupType qbanktypes.LockupTypes) {

}

// MeissaAudiorFunction audit the positions and KV stores for any unused or leaked amount.
// If any leaked or unused coin found then it should be used.
// Logic :
// 1. check the coins available in all the orion lockup accounts at today epochday.
// 2. transfer coins to the orion treasury. Orion treasury will also be used during users withdrawal.
func (k Keeper) MeissaAudiorFunction(ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes) {
	k.Logger(ctx).Info(fmt.Sprintf("Entered MeissaAudiorFunction|lockupType=%v\n",
		qbanktypes.LockupTypes_name[int32(lockupPeriod)]))
	coins := k.GetAllStakingBalances(ctx, lockupPeriod)
	k.Logger(ctx).Info(fmt.Sprintf("MeissaAudiorFunction|Coins=%v\n", coins))
	k.SendCoinsFromModuleToReserve(ctx, types.CreateOrionStakingMaccName(lockupPeriod), coins)
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
