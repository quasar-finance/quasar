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

	priceOracles []types.PriceOracle
	poolOracles  map[string]types.PoolOracle
	sealed       bool
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
		cdc:          cdc,
		storeKey:     storeKey,
		memKey:       memKey,
		tkey:         tkey,
		paramSpace:   paramSpace,
		authority:    authority,
		priceOracles: []types.PriceOracle{},
		poolOracles:  map[string]types.PoolOracle{},
	}
}

// RegisterPriceOracle registers a price oracle to the keeper.
func (k *Keeper) RegisterPriceOracle(oracle types.PriceOracle) {
	if k.sealed {
		panic("cannot register a price oracle to a sealed qoracle keeper")
	}

	k.priceOracles = append(k.priceOracles, oracle)
}

// RegisterPoolOracle registers a pool oracle to the keeper.
func (k *Keeper) RegisterPoolOracle(oracle types.PoolOracle) {
	if k.sealed {
		panic("cannot register a pool oracle to a sealed qoracle keeper")
	}

	k.poolOracles[oracle.Source()] = oracle
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
		k.UpdateDenomPrices(noGasCtx)
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
	case k.IsSymbolPriceListUpdateAvailable(ctx):
		k.UpdateDenomPrices(ctx)
		// When we update prices, we also need to recalculate TVL/APY and thus update pools.
		fallthrough
	case k.IsPoolsUpdateAvailable(ctx):
		// TODO: we should only update pools from the notifier source to help with gas consumption.
		k.UpdatePools(ctx)
	}
}

// NotifySymbolPricesUpdate notifies the qoracle keeper that new symbol prices are available.
func (k Keeper) NotifySymbolPricesUpdate(ctx sdk.Context) {
	store := ctx.TransientStore(k.tkey)
	store.Set(types.KeySymbolPriceListUpdateFlag, []byte{})
}

// IsSymbolPriceListUpdateAvailable returns if there's new symbol prices available.
func (k Keeper) IsSymbolPriceListUpdateAvailable(ctx sdk.Context) bool {
	store := ctx.TransientStore(k.tkey)
	return store.Has(types.KeySymbolPriceListUpdateFlag)
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

// UpdateDenomPrices fetches the latest symbol prices from price oracles if any available,
// convert symbol prices to denom price via denom price mapping and stores them in memory store.
func (k Keeper) UpdateDenomPrices(ctx sdk.Context) {
	// NOTE: For now we only support a single price oracle.
	if len(k.priceOracles) != 1 {
		panic("only a single price oracle is supported")
	}

	k.removeAllDenomPrices(ctx)

	spl, err := k.priceOracles[0].GetSymbolPriceList(ctx)
	if err != nil {
		k.Logger(ctx).Error("failed to fetch symbol prices from price oracle",
			"oracle_source", k.priceOracles[0].Source(),
			"error", err,
		)
		return
	}

	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, types.KeyDenomSymbolMappingPrefix)
	defer iter.Close()
	memStore := ctx.KVStore(k.memKey)
	for ; iter.Valid(); iter.Next() {
		var mapping types.DenomSymbolMapping
		k.cdc.MustUnmarshal(iter.Value(), &mapping)

		price := spl.Prices.AmountOf(mapping.OracleSymbol).Mul(mapping.Multiplier)
		if price.IsZero() {
			k.Logger(ctx).Error("failed to find symbol price for denom",
				"denom", mapping.Denom,
				"oracle_symbol", mapping.OracleSymbol,
			)
			continue
		}

		priceBz, err := price.Marshal()
		if err != nil {
			panic(err)
		}
		memStore.Set(types.GetDenomPriceKey(mapping.Denom), priceBz)
	}

	memStore.Set(types.KeyMemDenomPricesUpdatedAt, sdk.FormatTimeBytes(spl.UpdatedAt))
}

func (k Keeper) removeAllDenomPrices(ctx sdk.Context) {
	memStore := ctx.KVStore(k.memKey)

	iter := sdk.KVStorePrefixIterator(memStore, types.KeyMemDenomPricePrefix)
	defer iter.Close()
	for ; iter.Valid(); iter.Next() {
		memStore.Delete(iter.Key())
	}
}

// StoreDenomPriceMapping stores a denom symbol mapping in the store.
func (k Keeper) SetDenomSymbolMapping(ctx sdk.Context, mapping types.DenomSymbolMapping) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.GetDenomSymbolMappingKey(mapping.Denom), k.cdc.MustMarshal(&mapping))
}

// DeleteDenomSymbolMapping deletes a denom symbol mapping from the store.
func (k Keeper) DeleteDenomSymbolMapping(ctx sdk.Context, denom string) {
	store := ctx.KVStore(k.storeKey)
	store.Delete(types.GetDenomSymbolMappingKey(denom))
}

// UpdatePools fetches the latest pools from pool oracles if any available
// and stores them in memory store.
func (k Keeper) UpdatePools(ctx sdk.Context) {
	k.removeAllPools(ctx)

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
			osmosisPoolStore.Set([]byte(pool.Id), k.cdc.MustMarshal(&pool))
		}
	}
}

func (k Keeper) removeAllPools(ctx sdk.Context) {
	memStore := ctx.KVStore(k.memKey)

	iter := sdk.KVStorePrefixIterator(memStore, types.KeyMemPoolPrefix)
	defer iter.Close()
	for ; iter.Valid(); iter.Next() {
		memStore.Delete(iter.Key())
	}
}

// Logger returns a module-specific logger.
func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}
