package keeper

import (
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/cosmos/cosmos-sdk/codec"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

type (
	Keeper struct {
		cdc        codec.BinaryCodec
		storeKey   storetypes.StoreKey
		memKey     storetypes.StoreKey
		paramstore paramtypes.Subspace

		BankKeeper      types.BankKeeper
		accountKeeper   types.AccountKeeper
		qbankKeeper     types.QbankKeeper
		qoracleKeeper   types.QoracleKeeper
		intergammKeeper types.IntergammKeeper
		epochsKeeper    types.EpochsKeeper
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey storetypes.StoreKey,
	ps paramtypes.Subspace,
	accountkeeper types.AccountKeeper,
	bankkeeper types.BankKeeper,
	qbankkeeper types.QbankKeeper,
	qoraclekeeper types.QoracleKeeper,
	intergammkeeper types.IntergammKeeper,
	epochsKeeper types.EpochsKeeper,
) *Keeper {
	// set KeyTable if it has not already been set
	if !ps.HasKeyTable() {
		ps = ps.WithKeyTable(types.ParamKeyTable())
	}

	return &Keeper{

		cdc:             cdc,
		storeKey:        storeKey,
		memKey:          memKey,
		paramstore:      ps,
		accountKeeper:   accountkeeper,
		BankKeeper:      bankkeeper,
		qbankKeeper:     qbankkeeper,
		qoracleKeeper:   qoraclekeeper,
		intergammKeeper: intergammkeeper,
		epochsKeeper:    epochsKeeper,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

//func (k Keeper) GetAccountKeeper(ctx sdk.Context) types.AccountKeeper
