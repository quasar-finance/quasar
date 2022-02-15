package keeper

/*
	"fmt"
	"github.com/abag/quasarnode/x/osmolpv/types"

	"github.com/cosmos/cosmos-sdk/store/prefix"
*/
import (
	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Get
func (k Keeper) getPoolAssets(id uint64) (ps gammtypes.PoolAsset) {

	return
}

// Get the available amount from the KV store of meissa strategy.
func (k Keeper) getAssetBalance(ps gammtypes.PoolAsset) (c sdk.Coin) {
	return c
}

// Get APY ranked pool list
func (k Keeper) getAPYRankedPools() (poolIDs []uint64) {
	return
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

func (k Keeper) MeissaCoinDistribution() {
	/*
		poolIDs := k.getAPYRankedPools()

		for idx, poolID := range poolIDs {
			ps := k.getPoolAssets(poolID)
			r1, r2 := k.getPoolRatio(poolID)

		}
	*/
}
