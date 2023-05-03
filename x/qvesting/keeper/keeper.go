package keeper

import (
	"fmt"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting/exported"
	"github.com/tendermint/tendermint/libs/log"

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

// IterateVestingAccounts iterates over all the stored accounts and performs a callback function.
// Stops iteration when callback returns true.
func (k Keeper) IterateVestingAccounts(ctx sdk.Context, cb func(account exported.VestingAccount) (stop bool)) {
	store := ctx.KVStore(k.storeKey)
	iterator := sdk.KVStorePrefixIterator(store, types.VestingAccountStoreKeyPrefix)

	defer iterator.Close()
	for ; iterator.Valid(); iterator.Next() {
		addr := types.AddressFromVestingAccountKey(iterator.Key())

		acct := k.accountKeeper.GetAccount(ctx, addr)
		vestingAcct, ok := acct.(exported.VestingAccount)
		if !ok {
			// not vesting account
			continue
		}
		if cb(vestingAcct) {
			break
		}
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}
