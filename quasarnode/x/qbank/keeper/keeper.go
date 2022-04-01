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
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func (k Keeper) GetCdc() codec.BinaryCodec {
	return k.cdc
}
func (k Keeper) GetStoreKey() sdk.StoreKey {
	return k.storeKey
}

// Iterate will iterate through all keys with a given prefix using a provided callback function.
// If the provided callback function returns an error, iteration stops and the error
// is returned.
// Here, prefix param consists of complete key byte slice with prefix byte at 0th index

// func (k Keeper) Iterate(ctx sdk.Context, prefix []byte, cb func(key, val []byte) error) error {

func (k Keeper) Iterate(ctx sdk.Context, prefix []byte, cb func(key []byte, val sdk.Coin) error) error {
	store := ctx.KVStore(k.storeKey)

	iter := sdk.KVStorePrefixIterator(store, prefix)

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("Qbank Keeper Iterate|modulename=%s|blockheight=%d|prefix=%s",
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

// ProcessWithdrable Current implementation is based on the assumption that same withdrable amount
// will be available to withdraw which the user was submitted initially.
// However this logic can be used to calculate the expected withdrable with the assumption that market does
// not change.
// In the MVP phase - We are maintianing the same assumption, and implementing an assurance for the users
// to give back same deposited amount.
// This value can also be useful to calculate denom level PnL.
// AUDIT |  This logic can be changed in future based on the orion vault design.
func (k Keeper) ProcessWithdrable(ctx sdk.Context, keyPrefix []byte) {
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, keyPrefix)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("Qbank ProcessWithdrable|modulename=%s|blockheight=%d|keyPrefix=%s",
		types.ModuleName, ctx.BlockHeight(), string(keyPrefix)))

	// Key Example =499:Months_3:quasar1axasfth8yuuk50jqc37044nves9lht38yj5zrk:uqsar, Value = sdk.Coin
	for ; iter.Valid(); iter.Next() {

		key, val := iter.Key(), iter.Value()
		_, lockupStr, uid, _, _ := types.ParseEpochLockupUserDenomDepositKey(key)

		var coin sdk.Coin
		k.cdc.MustUnmarshal(val, &coin)
		logger.Info(fmt.Sprintf("Qbank BeginBlocker ProcessWithdrable|modulename=%s|blockheight=%d|Key=%v|Value=%v",
			types.ModuleName, ctx.BlockHeight(), string(key), coin))
		k.AddWithdrableAmt(ctx, uid, coin)
		lockupPeriod := types.LockupTypes(types.LockupTypes_value[lockupStr])
		k.AddLockupWithdrableAmt(ctx, uid, coin, lockupPeriod)
	}
}
