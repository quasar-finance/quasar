package keeper

/*
	"fmt"
	"github.com/abag/quasarnode/x/osmolpv/types"

	"github.com/cosmos/cosmos-sdk/store/prefix"
*/
import (
	"fmt"

	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Get
func (k Keeper) getPoolAssets(ctx sdk.Context, id uint64) (ps []gammtypes.PoolAsset) {
	//TODO -  Call oracle module keeper function to fetch the result
	return
}

// Get APY ranked pool list
func (k Keeper) getAPYRankedPools() (poolIDs []uint64) {
	//TODO -  Call oracle module keeper function to fetch the result
	return
}

// Get APY ranked pool list
func (k Keeper) getTotalShare(ctx sdk.Context, poolIDs uint64) (totalShare sdk.Coin) {
	//TODO -  Call oracle module keeper function to fetch the result
	return
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
	}

	// Withdraw from osmosis chain
	for _, sn := range strategies.Names {
		k.MeissaWithdraw(ctx, epochday, types.MeissaStrategiesLockup[sn])
	}

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

	poolIDs := k.getAPYRankedPools()

	for _, poolID := range poolIDs {
		assets := k.getPoolAssets(ctx, poolID)
		if len(assets) != 2 {
			// Initially strategy want to LP only in the pool with 2 assets
			continue
		}

		poolTotalShare := k.getTotalShare(ctx, poolID)

		var sharePerAssetAmount []sdk.Int
		var shareRequired []sdk.Int
		var maxAvailableAmount []sdk.Int

		for idx, asset := range assets {

			sharePerAssetAmount[idx] = poolTotalShare.Amount.Quo(asset.Token.Amount)
			maxAvailableAmount[idx] = k.getMaxAvailableAmount(ctx, lockupType, asset.Token.Denom)
			shareRequired[idx] = maxAvailableAmount[idx].Mul(sharePerAssetAmount[idx])

		}

		// TODO | AUDIT | Code optimization
		// Calculate required amount for second denom based on first denom.
		RequiredSecondDenom := shareRequired[0].Quo(sharePerAssetAmount[1])

		var FirstAssetAmount sdk.Int
		var SecondAssetAmount sdk.Int
		if maxAvailableAmount[1].GT(RequiredSecondDenom) {
			// Consider this amounts for LPing
			// Use shareRequired[0]
			FirstAssetAmount = shareRequired[0].Mul(sharePerAssetAmount[0])
			SecondAssetAmount = shareRequired[0].Mul(sharePerAssetAmount[1])
		} else {
			// Use shareRequired[1]
			FirstAssetAmount = shareRequired[1].Mul(sharePerAssetAmount[0])
			SecondAssetAmount = shareRequired[1].Mul(sharePerAssetAmount[1])

		}

		k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|FirstAssetAmount=%v|SecondAssetAmount=%v\n",
			FirstAssetAmount, SecondAssetAmount))

		// Transfer fund to the strategy global account.
		// TODO - 1. Optimize it to have one call only

		coin1 := sdk.NewCoin(assets[0].Token.Denom, FirstAssetAmount)
		coin2 := sdk.NewCoin(assets[1].Token.Denom, SecondAssetAmount)
		k.SendCoinFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coin1)
		k.SendCoinFromModuleToMeissa(ctx, types.CreateOrionStakingMaccName(lockupType), coin2)

		// TODO : Call Intergamm Add Liquidity Method

		// TODO : Update orion vault staking amount.
		// Most probably not needed as balance in the orion vault is already updated.

		// TODO : If sorted coins is required.
		coins := sdk.NewCoins(coin1, coin2)
		k.SetMeissaEpochLockupPoolPosition(ctx, epochday, lockupType, poolID, coins)
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
	poolIDs := k.getAPYRankedPools()
	offsetedEpochDay := currEpochday - uint64(lockupType)
	for _, poolID := range poolIDs {
		coins := k.GetMeissaEpochLockupPoolPosition(ctx, offsetedEpochDay, lockupType, poolID)
		k.Logger(ctx).Info(fmt.Sprintf("MeissaExit|currEpochday=%v|offsetedEpochDay=%v|PoolID=%v|Coins=%v\n",
			currEpochday, offsetedEpochDay, poolID, coins))

		// TODO - Call intergamm exit pool method
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
