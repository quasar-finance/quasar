package keeper

import (
	"fmt"
	"sort"
	"time"

	sdkmath "cosmossdk.io/math"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

const Year = 365 * 24 * time.Hour

func (k Keeper) SetOsmosisEpochsInfo(ctx sdk.Context, epochs []epochtypes.EpochInfo) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyEpochsInfoPrefix)

	for _, epoch := range epochs {
		store.Set([]byte(epoch.Identifier), k.cdc.MustMarshal(&epoch))
	}
}

// getOsmosisStore returns the prefix store for osmosis incoming data
func (k Keeper) getOsmosisStore(ctx sdk.Context) prefix.Store {
	return prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyOsmosisPrefix)
}

// GetOsmosisEpochsInfo returns the latest received epochs info from osmosis
func (k Keeper) GetOsmosisEpochsInfo(ctx sdk.Context) []epochtypes.EpochInfo {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyEpochsInfoPrefix)

	iter := store.Iterator(nil, nil)
	defer iter.Close()
	var epochs []epochtypes.EpochInfo
	for ; iter.Valid(); iter.Next() {
		var epoch epochtypes.EpochInfo
		k.cdc.MustUnmarshal(iter.Value(), &epoch)
		epochs = append(epochs, epoch)
	}
	return epochs
}

func (k Keeper) SetOsmosisPool(ctx sdk.Context, pool types.OsmosisPool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyPoolPrefix)

	key := sdk.Uint64ToBigEndian(pool.PoolInfo.Id)
	store.Set(key, k.cdc.MustMarshal(&pool))
}

// GetOsmosisPool returns the pool with the given id if exists
func (k Keeper) GetOsmosisPool(ctx sdk.Context, id uint64) (types.OsmosisPool, bool) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyPoolPrefix)

	key := sdk.Uint64ToBigEndian(id)
	bz := store.Get(key)
	if bz == nil {
		return types.OsmosisPool{}, false
	}

	var pool types.OsmosisPool
	k.cdc.MustUnmarshal(bz, &pool)
	return pool, true
}

func (k Keeper) removeAllPools(ctx sdk.Context) {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyPoolPrefix)

	iterator := store.Iterator(nil, nil)
	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		store.Delete(iterator.Key())
	}
}

// GetOsmosisPoolsRankedByAPY returns a list of all pools with ordered by APY in descending order with an optional denom filter.
// If denom is empty the function will return all osmosis incentivized pools (for more info follow https://docs.osmosis.zone/osmosis-core/modules/spec-pool-incentives)
// otherwise it only returns pools that have denom as their deposited asset.
func (k Keeper) GetOsmosisPoolsRankedByAPY(ctx sdk.Context, denom string) []types.OsmosisPool {
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyPoolPrefix)

	iter := store.Iterator(nil, nil)
	defer iter.Close()
	var pools types.OsmosisPoolsOrderedByAPY
	for ; iter.Valid(); iter.Next() {
		var pool types.OsmosisPool
		k.cdc.MustUnmarshal(iter.Value(), &pool)

		if denom != "" {
			// Filter out pools with the desired denom as asset
			if !poolHasDenom(pool.PoolInfo, denom) {
				continue
			}
		}
		pools = append(pools, pool)
	}

	// Order by APY in descending order
	sort.Stable(sort.Reverse(pools))
	return pools
}

func poolHasDenom(pool balancerpool.Pool, denom string) bool {
	for _, asset := range pool.PoolAssets {
		if asset.Token.Denom == denom {
			return true
		}
	}
	return false
}

func (k Keeper) setOsmosisLockableDurations(ctx sdk.Context, lockableDurations poolincentivestypes.QueryLockableDurationsResponse) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyLockableDurations, k.cdc.MustMarshal(&lockableDurations))
}

// GetOsmosisLockableDurations returns the latest received lockable durations from osmosis
func (k Keeper) GetOsmosisLockableDurations(ctx sdk.Context) []time.Duration {
	store := k.getOsmosisStore(ctx)

	var lockableDurations poolincentivestypes.QueryLockableDurationsResponse
	k.cdc.MustUnmarshal(store.Get(types.KeyLockableDurations), &lockableDurations)
	return lockableDurations.LockableDurations
}

func (k Keeper) SetOsmosisMintParams(ctx sdk.Context, mintParams minttypes.Params) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyMintParams, k.cdc.MustMarshal(&mintParams))
}

// GetOsmosisMintParams returns the latest received mint params from osmosis
func (k Keeper) GetOsmosisMintParams(ctx sdk.Context) minttypes.Params {
	store := k.getOsmosisStore(ctx)

	var mintParams minttypes.Params
	k.cdc.MustUnmarshal(store.Get(types.KeyMintParams), &mintParams)
	return mintParams
}

func (k Keeper) SetOsmosisMintEpochProvisions(ctx sdk.Context, epochProvisions sdk.Dec) {
	store := k.getOsmosisStore(ctx)

	bz, err := epochProvisions.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(types.KeyMintEpochProvisions, bz)
}

// GetOsmosisMintEpochProvisions returns the latest received epoch provisions from osmosis
func (k Keeper) GetOsmosisMintEpochProvisions(ctx sdk.Context) sdk.Dec {
	store := k.getOsmosisStore(ctx)
	bz := store.Get(types.KeyMintEpochProvisions)

	var epochProvisions sdk.Dec
	err := epochProvisions.Unmarshal(bz)
	if err != nil {
		panic(err)
	}
	return epochProvisions
}

func (k Keeper) SetOsmosisIncentivizedPools(ctx sdk.Context, pools []poolincentivestypes.IncentivizedPool) {
	store := k.getOsmosisStore(ctx)
	store.Set(types.KeyIncentivizedPools, k.cdc.MustMarshal(&types.IncentivizedPools{IncentivizedPools: pools}))
}

func (k Keeper) GetOsmosisIncentivizedPools(ctx sdk.Context) []poolincentivestypes.IncentivizedPool {
	store := k.getOsmosisStore(ctx)
	var pools types.IncentivizedPools
	k.cdc.MustUnmarshal(store.Get(types.KeyIncentivizedPools), &pools)
	return pools.IncentivizedPools
}

func (k Keeper) SetOsmosisDistrInfo(ctx sdk.Context, distrInfo poolincentivestypes.DistrInfo) {
	store := k.getOsmosisStore(ctx)

	store.Set(types.KeyDistrInfo, k.cdc.MustMarshal(&distrInfo))
}

// GetOsmosisDistrInfo returns the latest received distr info from osmosis
func (k Keeper) GetOsmosisDistrInfo(ctx sdk.Context) poolincentivestypes.DistrInfo {
	store := k.getOsmosisStore(ctx)

	var distrInfo poolincentivestypes.DistrInfo
	k.cdc.MustUnmarshal(store.Get(types.KeyDistrInfo), &distrInfo)
	return distrInfo
}

func (k Keeper) CalculatePoolTVL(ctx sdk.Context, pool balancerpool.Pool) (sdk.Dec, error) {
	tvl := sdk.ZeroDec()
	for _, asset := range pool.PoolAssets {
		price, found := k.GetStablePrice(ctx, asset.Token.Denom)
		if !found {
			return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrStablePriceNotFound, fmt.Sprintf("denom: %s", asset.Token.Denom))
		}

		tvl = tvl.Add(sdk.NewDecFromInt(asset.Token.Amount).Mul(price))
	}
	return tvl, nil
}

func (k Keeper) calculateOsmosisPoolMetrics(ctx sdk.Context, pool balancerpool.Pool) (types.OsmosisPoolMetrics, error) {
	tvl, err := k.CalculatePoolTVL(ctx, pool)
	if err != nil {
		return types.OsmosisPoolMetrics{}, sdkerrors.Wrap(err, "could not calculate tvl")
	}
	apy, err := k.CalculatePoolAPY(ctx, pool, tvl)
	if err != nil {
		return types.OsmosisPoolMetrics{}, sdkerrors.Wrap(err, "could not calculate apy")
	}

	return types.OsmosisPoolMetrics{
		APY: apy,
		TVL: tvl,
	}, nil
}

func (k Keeper) CalculatePoolAPY(ctx sdk.Context, pool balancerpool.Pool, poolTVL sdk.Dec) (sdk.Dec, error) {
	distrInfo := k.GetOsmosisDistrInfo(ctx)
	epochProvisions := k.GetOsmosisMintEpochProvisions(ctx)

	mintParams := k.GetOsmosisMintParams(ctx)
	poolIncentivesProportion := mintParams.DistributionProportions.PoolIncentives
	mintDenomPrice, found := k.GetStablePrice(ctx, mintParams.MintDenom)
	if !found {
		return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrStablePriceNotFound, fmt.Sprintf("denom: %s", mintParams.MintDenom))
	}
	mintEpoch, found := k.findOsmosisEpochByIdentifier(ctx, mintParams.EpochIdentifier)
	if !found {
		return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrEpochNotFound, fmt.Sprintf("could not find osmosis mint, epoch identifier: %s", mintParams.EpochIdentifier))
	}

	poolTotalWeight := sdk.ZeroInt()
	for _, incentive := range k.GetOsmosisIncentivizedPools(ctx) {
		if incentive.PoolId == pool.Id {
			gaugeWeight, found := findGaugeWeight(ctx, incentive.GaugeId, distrInfo)
			if !found {
				return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrGaugeWeightNotFound, fmt.Sprintf("gauge id: %d", incentive.GaugeId))
			}
			poolTotalWeight = poolTotalWeight.Add(gaugeWeight)
		}
	}

	// Number of mint epochs occurrence in a year
	annualMintEpochs := Year.Nanoseconds() / mintEpoch.Duration.Nanoseconds()
	annualProvisions := epochProvisions.MulInt64(annualMintEpochs)
	// Annual provisions share to incentivize pools is equal to "annualProvisions * poolIncentivesProportion"
	annualPoolIncentives := annualProvisions.Mul(poolIncentivesProportion)
	// Total annual provision share (including all gauges) of the requested pool in $
	// is equal to "annualPoolIncentives * poolTotalWeight / distrInfo.TotalWeight * mintDenomPrice"
	poolAnnualProvisions := annualPoolIncentives.MulInt(poolTotalWeight).QuoInt(distrInfo.TotalWeight).Mul(mintDenomPrice)
	// APY of the requested pool is equal to "(poolAnnualProvisions / poolTVL) * 100"
	poolAPY := poolAnnualProvisions.Quo(poolTVL).Mul(sdk.NewDec(100))
	return poolAPY, nil
}

// findOsmosisEpochByIdentifier iterates over all osmosis epochs and returns the epoch with given identifier if exists.
func (k Keeper) findOsmosisEpochByIdentifier(ctx sdk.Context, identifier string) (epochtypes.EpochInfo, bool) {
	for _, epoch := range k.GetOsmosisEpochsInfo(ctx) {
		if epoch.Identifier == identifier {
			return epoch, true
		}
	}
	return epochtypes.EpochInfo{}, false
}

// findGaugeWeight iterates over distrInfo.Records and returns the weight of record is it finds and record with given gaugeId.
func findGaugeWeight(ctx sdk.Context, gaugeId uint64, distrInfo poolincentivestypes.DistrInfo) (sdkmath.Int, bool) {
	for _, record := range distrInfo.Records {
		if record.GaugeId == gaugeId {
			return record.Weight, true
		}
	}
	return sdk.ZeroInt(), false
}
