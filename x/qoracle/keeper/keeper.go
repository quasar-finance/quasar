package keeper

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/tendermint/tendermint/libs/log"
)

type Keeper struct {
	cdc        codec.BinaryCodec
	storeKey   storetypes.StoreKey
	memKey     storetypes.StoreKey
	tkey       storetypes.StoreKey
	paramSpace paramtypes.Subspace
	authority  string // the address capable of adding or removing denom symbol mappings. Usually the gov module account

	poolOracles map[string]types.PoolOracle
	sealed      bool
}

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey storetypes.StoreKey,
	memKey storetypes.StoreKey,
	tkey storetypes.StoreKey,
	paramSpace paramtypes.Subspace,
	authority string,
) Keeper {
	// set KeyTable if it has not already been set
	if !paramSpace.HasKeyTable() {
		paramSpace = paramSpace.WithKeyTable(types.ParamKeyTable())
	}

	return Keeper{
		cdc:        cdc,
		storeKey:   storeKey,
		memKey:     memKey,
		tkey:       tkey,
		paramSpace: paramSpace,
		authority:  authority,
		// priceOracles: []types.PriceOracle{},
		poolOracles: map[string]types.PoolOracle{},
	}
}

// RegisterPoolOracle registers a pool oracle to the keeper.
func (k *Keeper) RegisterPoolOracle(_ types.PoolOracle) {
	if k.sealed {
		panic("cannot register a pool oracle to a sealed qoracle keeper")
	}

	// TODO_IMPORTANT -
	// k.poolOracles[oracle.Source()] = oracle
}

// Seal seals the keeper to prevent registering further oracles.
// Seal may be called during app initialization.
func (k *Keeper) Seal() {
	if k.sealed {
		panic("cannot initialize and seal an already sealed qoracle keeper")
	}

	k.sealed = true
}

// IsSealed returns if the keeper is sealed.
func (k *Keeper) IsSealed() bool {
	return k.sealed
}

// InitMemStore will assure that the module store is a memory store (it will panic if it's not)
// and will initialize it. The function is safe to be called multiple times.
// InitMemStore must be called every time the app starts before the keeper is used (so
// `BeginBlock` or `InitChain` - whichever is first). We need access to the store so we
// can't initialize it in a constructor.
func (k Keeper) InitMemStore(ctx sdk.Context) {
	memStore := ctx.KVStore(k.memKey)
	memStoreType := memStore.GetStoreType()

	if memStoreType != storetypes.StoreTypeMemory {
		panic(fmt.Sprintf("invalid memory store type; got %s, expected: %s", memStoreType, storetypes.StoreTypeMemory))
	}

	// create context with no block gas meter to ensure we do not consume gas during local initialization logic.
	noGasCtx := ctx.WithBlockGasMeter(sdk.NewInfiniteGasMeter())

	// check if memory store has not been initialized yet by checking if initialized flag is nil.
	if !k.IsInitialized(noGasCtx) {
		// initialize memory store here
		k.UpdatePools(noGasCtx)

		// set the initialized flag so we don't rerun initialization logic
		memStore := noGasCtx.KVStore(k.memKey)
		memStore.Set(types.KeyMemInitialized, []byte{1})
	}
}

// IsInitialized returns if the memory store has been initialized.
func (k Keeper) IsInitialized(ctx sdk.Context) bool {
	memStore := ctx.KVStore(k.memKey)
	return memStore.Has(types.KeyMemInitialized)
}

// UpdateMemStore updates the memory store. it first checks if there's new updated data available
// for either the symbol prices or the pools. If there is new symbol prices available, it will
// update the symbol prices and then update the pools. If there is new pools available, it will
// only update the pools. oracle submodules can notify the qoracle keeper that new data is available
// by setting the corresponding update flag.
func (k Keeper) UpdateMemStore(ctx sdk.Context) {
	switch {
	case k.IsPoolsUpdateAvailable(ctx):
		// TODO: we should only update pools from the notifier source to help with gas consumption.
		k.UpdatePools(ctx)
	}
}

// NotifyPoolsUpdate notifies the qoracle keeper that new pools are available.
func (k Keeper) NotifyPoolsUpdate(ctx sdk.Context) {
	store := ctx.TransientStore(k.tkey)
	store.Set(types.KeyPoolsUpdateFlag, []byte{})
}

// IsPoolsUpdateAvailable returns if there's new pools available.
func (k Keeper) IsPoolsUpdateAvailable(ctx sdk.Context) bool {
	store := ctx.TransientStore(k.tkey)
	return store.Has(types.KeyPoolsUpdateFlag)
}

// UpdatePools fetches the latest pools from pool oracles if any available
// and stores them in memory store.
func (k Keeper) UpdatePools(ctx sdk.Context) {
	k.Logger(ctx).Debug("UpdatePools...")
	k.removeAllPools(ctx)

	// Check the length of pools sources
	for _, oracle := range k.poolOracles {
		pools, err := oracle.GetPools(ctx)
		if err != nil {
			k.Logger(ctx).Error("failed to fetch pools from pool oracle",
				"oracle_source", oracle.Source(),
				"error", err,
			)
			continue
		}

		memStore := ctx.KVStore(k.memKey)
		poolStore := prefix.NewStore(memStore, types.KeyMemPoolPrefix)
		osmosisPoolStore := prefix.NewStore(poolStore, types.KeyOsmosisPoolPrefix)
		for _, pool := range pools {
			pool := pool
			osmosisPoolStore.Set([]byte(pool.Id), k.cdc.MustMarshal(&pool))
		}
	}
}

// Remove all the pools iterating through the KeyMemPoolPrefix itr.
func (k Keeper) removeAllPools(ctx sdk.Context) {
	memStore := ctx.KVStore(k.memKey)

	iter := sdk.KVStorePrefixIterator(memStore, types.KeyMemPoolPrefix)
	defer iter.Close()
	for ; iter.Valid(); iter.Next() {
		k.Logger(ctx).Debug("Deleting pools ",
			"Key", iter.Key(),
			"Value", iter.Value(),
		)
		memStore.Delete(iter.Key())
	}
}

// Logger returns a module-specific logger.
func (Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}
