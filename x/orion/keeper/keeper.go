package keeper

import (
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/abag/quasarnode/x/orion/types"
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

		bankKeeper      types.BankKeeper
		accountKeeper   types.AccountKeeper
		qbankKeeper     types.QbankKeeper
		qoracleKeeper   types.QoracleKeeper
		intergammKeeper types.IntergammKeeper
	}
)

func NewKeeper(
	cdc codec.BinaryCodec,
	storeKey,
	memKey sdk.StoreKey,
	ps paramtypes.Subspace,
	accountkeeper types.AccountKeeper,
	bankkeeper types.BankKeeper,
	qbankkeeper types.QbankKeeper,
	qoraclekeeper types.QoracleKeeper,
	intergammkeeper types.IntergammKeeper,

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
		bankKeeper:      bankkeeper,
		qbankKeeper:     qbankkeeper,
		qoracleKeeper:   qoraclekeeper,
		intergammKeeper: intergammkeeper,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}
