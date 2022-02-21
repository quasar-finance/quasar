package keeper

/*
	"fmt"
	"github.com/abag/quasarnode/x/osmolpv/types"

	"github.com/cosmos/cosmos-sdk/store/prefix"
*/
import (
	"fmt"

	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
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
func (k Keeper) getTotalShare(ctx sdk.Context, poolIDs uint64) (totalShare uint64) {
	//TODO -  Call oracle module keeper function to fetch the result
	return
}

// Get the maximum available amount in the orion staking
func (k Keeper) getMaxAvailableAmount(ctx sdk.Context, denom string, lockupPeriod qbanktypes.LockupTypes) uint64 {
	return 0
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

func (k Keeper) MeissaCoinDistribution(ctx sdk.Context, lockupType qbanktypes.LockupTypes) {

	poolIDs := k.getAPYRankedPools()

	for _, poolID := range poolIDs {
		assets := k.getPoolAssets(ctx, poolID)
		if len(assets) != 2 {
			// Initially strategy want to LP only in the pool whith 2 pairs
			continue
		}
		// r1, r2 := k.getPoolRatios(poolID)
		poolTotalShare := k.getTotalShare(ctx, poolID)
		var sharePerAssetAmount []uint64
		var shareRequired []uint64
		var maxAvailableAmount []uint64

		for idx, asset := range assets {
			sharePerAssetAmount[idx] = poolTotalShare / asset.Token.Amount.Uint64()

			maxAvailableAmount[idx] = k.getMaxAvailableAmount(ctx, asset.Token.Denom, lockupType)

			shareRequired[idx] = maxAvailableAmount[idx] * sharePerAssetAmount[idx]

		}

		// Calculate required amount for second denom based on first denom.
		RequiredSecondDenom := shareRequired[0] / sharePerAssetAmount[1]
		var FirstAssetAmount uint64
		var SecondAssetAmount uint64
		if maxAvailableAmount[1] > RequiredSecondDenom {
			// Consider this amounts for LPing
			// Use shareRequired[0]
			FirstAssetAmount = shareRequired[0] * sharePerAssetAmount[0]
			SecondAssetAmount = shareRequired[0] * sharePerAssetAmount[1]
		} else {
			// Use shareRequired[1]
			FirstAssetAmount = shareRequired[1] * sharePerAssetAmount[0]
			SecondAssetAmount = shareRequired[1] * sharePerAssetAmount[1]

		}

		k.Logger(ctx).Info(fmt.Sprintf("MeissaCoinDistribution|FirstAssetAmount=%v|SecondAssetAmount=%v\n",
			FirstAssetAmount, SecondAssetAmount))

		// TODO : Call Intergamm Add Liquidity Method
		// TODO : Update orion vault staking amount
	}

}
