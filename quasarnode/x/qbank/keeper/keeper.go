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
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	ps paramtypes.Subspace,

	bankKeeper types.BankKeeper,
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

// ProcessWithdrable implemtns the logic for the current expected withdrable amount based on
// the users deposit done; Expected withdraw amount = actual deposited amount.
// In the MVP phase - We are maintianing the same assumption to give back same equivalent deposited amount
// based on the current market value. And implementing an assurance for the users by giving them vault
// based tokens backed by quasar tokens.
// This value can also be useful to calculate denom level PnL.
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
