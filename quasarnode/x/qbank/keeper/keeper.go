package keeper

import (
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

type (
	Keeper struct {
		cdc        codec.BinaryCodec
		storeKey   sdk.StoreKey
		memKey     sdk.StoreKey
		paramstore paramtypes.Subspace

		bankKeeper types.BankKeeper
		//oionKeeper types.OrionKeeper
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	ps paramtypes.Subspace,

	bankKeeper types.BankKeeper,
	//orionKeeper types.OrionKeeper,
) *Keeper {
	// set KeyTable if it has not already been set
	if !ps.HasKeyTable() {
		ps = ps.WithKeyTable(types.ParamKeyTable())
	}

	return &Keeper{

		cdc:        cdc,
		storeKey:   storeKey,
		memKey:     memKey,
		paramstore: ps,
		bankKeeper: bankKeeper,
		//oionKeeper: orionKeeper,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func (k Keeper) Iterator2(ctx sdk.Context) error {
	return nil
}

// iterate will iterate through all keys with a given prefix using a provided function.
// If the provided callback function returns an error, iteration stops and the error
// is returned.
// func (k Keeper) Iterate(ctx sdk.Context, prefix []byte, cb func(key, val []byte) error) error {

func (k Keeper) Iterate(ctx sdk.Context, prefix []byte, cb func(key []byte, val sdk.Coin) error) error {
	store := ctx.KVStore(k.storeKey)

	iter := sdk.KVStorePrefixIterator(store, prefix)

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("Qbank Iterate %s| blockheight %d | prefix = %s",
		types.ModuleName, ctx.BlockHeight(), string(prefix)))

	defer iter.Close()

	for ; iter.Valid(); iter.Next() {
		key, val := iter.Key(), iter.Value()

		var storedCoin sdk.Coin

		k.cdc.MustUnmarshal(val, &storedCoin)

		if err := cb(key, storedCoin); err != nil {
			return err
		}
	}

	return nil
}
