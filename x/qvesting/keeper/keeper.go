package keeper

import (
	"fmt"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/x/auth/vesting/exported"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"github.com/tendermint/tendermint/libs/log"
)

type (
	Keeper struct {
		cdc        codec.BinaryCodec
		storeKey   sdk.StoreKey
		memKey     sdk.StoreKey
		paramstore paramtypes.Subspace

		accountKeeper types.AccountKeeper
		bankKeeper    types.BankKeeper
		stakingKeeper types.StakingKeeper
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	ps paramtypes.Subspace,

	accountKeeper types.AccountKeeper,
	bankKeeper types.BankKeeper,
	stakingKeeper types.StakingKeeper,
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
		stakingKeeper: stakingKeeper,
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
	allAccounts := k.accountKeeper.GetAllAccounts(sdkCtx)

	for _, acct := range allAccounts {
		vestingAcct, ok := acct.(exported.VestingAccount)
		if ok {
			addr := vestingAcct.GetAddress()
			if err := callback(addr); err != nil {
				return err
			}
		}
	}

	return nil
}
