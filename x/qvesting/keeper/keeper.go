package keeper

import (
	"fmt"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting/exported"
	"github.com/tendermint/tendermint/libs/log"
	"google.golang.org/grpc/grpclog"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
)

type (
	Keeper struct {
		cdc        codec.BinaryCodec
		storeKey   sdk.StoreKey
		memKey     sdk.StoreKey
		paramstore paramtypes.Subspace

		accountKeeper types.AccountKeeper
		bankKeeper    types.BankKeeper
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	ps paramtypes.Subspace,

	accountKeeper types.AccountKeeper,
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

		accountKeeper: accountKeeper,
		bankKeeper:    bankKeeper,
	}
}

func (k Keeper) AddVestingAccount(ctx sdk.Context, addr sdk.AccAddress) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.VestingAccountStoreKey(addr), []byte{})
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

// iterateVestingAccounts iterates over all vesting accounts and invokes a callback function on each of them.
func (k Keeper) iterateVestingAccounts(sdkCtx sdk.Context, callback func(addr sdk.AccAddress) error) error {
	store := sdkCtx.KVStore(k.storeKey)
	accountsStore := prefix.NewStore(store, types.VestingAccountStoreKeyPrefix)
	iterator := accountsStore.Iterator(nil, nil)

	// empty allocated resources after execution
	defer func(iterator storetypes.Iterator) {
		err := iterator.Close()
		if err != nil {
			grpclog.Infof("Failed to close iterator for %s", err)
		}
	}(iterator)

	for ; iterator.Valid(); iterator.Next() {
		key := iterator.Key()
		addr := sdk.AccAddress(key)
		acct := k.accountKeeper.GetAccount(sdkCtx, addr)
		_, ok := acct.(exported.VestingAccount)
		if !ok {
			return fmt.Errorf("account is not vesting account: %s", addr.String())
		}

		// invoke the callback function for the iterated vesting account
		if err := callback(addr); err != nil {
			return err
		}
	}

	return nil
}
