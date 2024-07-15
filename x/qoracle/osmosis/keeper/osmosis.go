package keeper

import (
	"fmt"
	"time"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"

	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (k Keeper) SetEpochsInfo(ctx sdk.Context, epochs []epochtypes.EpochInfo) {
	store := ctx.KVStore(k.storeKey)

	store.Set(types.KeyEpochsInfo, k.cdc.MustMarshal(&types.EpochsInfo{EpochsInfo: epochs}))
}

// GetEpochsInfo returns the latest received epochs info from osmosis
func (k Keeper) GetEpochsInfo(ctx sdk.Context) []epochtypes.EpochInfo {
	store := ctx.KVStore(k.storeKey)

	var epochsInfo types.EpochsInfo
	k.cdc.MustUnmarshal(store.Get(types.KeyEpochsInfo), &epochsInfo)
	return epochsInfo.EpochsInfo
}

func (k Keeper) SetPool(ctx sdk.Context, pool balancerpool.Pool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPoolPrefix)

	key := sdk.Uint64ToBigEndian(pool.Id)
	store.Set(key, k.cdc.MustMarshal(&pool))
}

func (k Keeper) SetPoolsUpdatedAt(ctx sdk.Context, updatedAt time.Time) {
	store := ctx.KVStore(k.storeKey)

	store.Set(types.KeyPoolsUpdatedAt, sdk.FormatTimeBytes(updatedAt))
}

// GetPools implements qoracletypes.PoolOracle
func (k Keeper) GetPools(ctx sdk.Context) ([]qoracletypes.Pool, error) {
	apyc, err := k.newAPYCalculator(ctx)
	if err != nil {
		return nil, fmt.Errorf("could not create a new apyCalculator: %w", err)
	}

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPoolPrefix)
	iter := store.Iterator(nil, nil)
	defer iter.Close()
	var pools []qoracletypes.Pool
	for ; iter.Valid(); iter.Next() {
		var pool balancerpool.Pool
		k.cdc.MustUnmarshal(iter.Value(), &pool)

		tvl, err := k.CalculatePoolTVL(ctx, pool)
		if err != nil {
			k.Logger(ctx).Error("failed to calculate tvl for pool",
				"pool", pool,
				"error", err)
			continue
		}
		apy, err := apyc.Calculate(ctx, pool, tvl)
		if err != nil {
			k.Logger(ctx).Error("failed to calculate apy for pool",
				"pool", pool,
				"error", err)
			continue
		}

		raw, err := codectypes.NewAnyWithValue(&pool)
		if err != nil {
			panic(err) // There's something wrong with our proto definitions
		}

		pools = append(pools, qoracletypes.Pool{
			Id:        fmt.Sprintf("%d", pool.Id),
			Assets:    extractPoolAssets(pool),
			TVL:       tvl,
			APY:       apy,
			Raw:       raw,
			UpdatedAt: k.GetPoolsUpdatedAt(ctx),
		})
	}

	return pools, nil
}

// GetPoolsUpdatedAt returns the block time of the last time the pools were updated
func (k Keeper) GetPoolsUpdatedAt(ctx sdk.Context) time.Time {
	store := ctx.KVStore(k.storeKey)

	updatedAt, err := sdk.ParseTimeBytes(store.Get(types.KeyPoolsUpdatedAt))
	if err != nil {
		return ctx.BlockTime()
	}
	return updatedAt
}

// GetPool returns the pool with the given id if exists
func (k Keeper) GetPool(ctx sdk.Context, id uint64) (balancerpool.Pool, bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPoolPrefix)

	key := sdk.Uint64ToBigEndian(id)
	bz := store.Get(key)
	if bz == nil {
		return balancerpool.Pool{}, false
	}

	var pool balancerpool.Pool
	k.cdc.MustUnmarshal(bz, &pool)
	return pool, true
}

func (k Keeper) removeAllPools(ctx sdk.Context) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPoolPrefix)

	iterator := store.Iterator(nil, nil)
	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		store.Delete(iterator.Key())
	}
}

func (k Keeper) SetLockableDurations(ctx sdk.Context, lockableDurations poolincentivestypes.QueryLockableDurationsResponse) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyLockableDurations, k.cdc.MustMarshal(&lockableDurations))
}

// GetLockableDurations returns the latest received lockable durations from osmosis
func (k Keeper) GetLockableDurations(ctx sdk.Context) []time.Duration {
	store := ctx.KVStore(k.storeKey)
	var lockableDurations poolincentivestypes.QueryLockableDurationsResponse
	k.cdc.MustUnmarshal(store.Get(types.KeyLockableDurations), &lockableDurations)
	return lockableDurations.LockableDurations
}

func (k Keeper) SetMintParams(ctx sdk.Context, mintParams minttypes.Params) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyMintParams, k.cdc.MustMarshal(&mintParams))
}

// GetMintParams returns the latest received mint params from osmosis
func (k Keeper) GetMintParams(ctx sdk.Context) minttypes.Params {
	store := ctx.KVStore(k.storeKey)
	var mintParams minttypes.Params
	k.cdc.MustUnmarshal(store.Get(types.KeyMintParams), &mintParams)
	return mintParams
}

func (k Keeper) SetMintEpochProvisions(ctx sdk.Context, epochProvisions sdk.Dec) {
	store := ctx.KVStore(k.storeKey)

	bz, err := epochProvisions.Marshal()
	if err != nil {
		panic(err)
	}
	store.Set(types.KeyMintEpochProvisions, bz)
}

// GetMintEpochProvisions returns the latest received epoch provisions from osmosis
func (k Keeper) GetMintEpochProvisions(ctx sdk.Context) sdk.Dec {
	store := ctx.KVStore(k.storeKey)
	bz := store.Get(types.KeyMintEpochProvisions)

	var epochProvisions sdk.Dec
	err := epochProvisions.Unmarshal(bz)
	if err != nil {
		panic(err)
	}
	return epochProvisions
}

func (k Keeper) SetIncentivizedPools(ctx sdk.Context, pools []poolincentivestypes.IncentivizedPool) {
	store := ctx.KVStore(k.storeKey)

	store.Set(types.KeyIncentivizedPools, k.cdc.MustMarshal(&types.IncentivizedPools{IncentivizedPools: pools}))
}

func (k Keeper) GetIncentivizedPools(ctx sdk.Context) []poolincentivestypes.IncentivizedPool {
	store := ctx.KVStore(k.storeKey)

	var pools types.IncentivizedPools
	k.cdc.MustUnmarshal(store.Get(types.KeyIncentivizedPools), &pools)
	return pools.IncentivizedPools
}

func (k Keeper) SetDistrInfo(ctx sdk.Context, distrInfo poolincentivestypes.DistrInfo) {
	store := ctx.KVStore(k.storeKey)

	store.Set(types.KeyDistrInfo, k.cdc.MustMarshal(&distrInfo))
}

// GetDistrInfo returns the latest received distr info from osmosis
func (k Keeper) GetDistrInfo(ctx sdk.Context) poolincentivestypes.DistrInfo {
	store := ctx.KVStore(k.storeKey)

	var distrInfo poolincentivestypes.DistrInfo
	k.cdc.MustUnmarshal(store.Get(types.KeyDistrInfo), &distrInfo)
	return distrInfo
}
