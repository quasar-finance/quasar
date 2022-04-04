package keeper

import (
	"testing"

	"github.com/abag/quasarnode/app"
	"github.com/cosmos/cosmos-sdk/store"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/stretchr/testify/require"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmdb "github.com/tendermint/tm-db"
)

type TestKeeperState struct {
	T              *testing.T
	Logger         log.Logger
	EncodingConfig cosmoscmd.EncodingConfig

	TestDb     *tmdb.MemDB
	StateStore storetypes.CommitMultiStore

	Ctx sdk.Context

	ParamsKeeper  *paramskeeper.Keeper
	AccountKeeper *authkeeper.AccountKeeper
	BankKeeper    bankkeeper.Keeper
}

func NewTestKeeperState(t *testing.T) *TestKeeperState {
	logger := log.TestingLogger()
	logger.Info("creating TestKeeperState")

	db := tmdb.NewMemDB()
	stateStore := store.NewCommitMultiStore(db)

	ctx := sdk.NewContext(stateStore, tmproto.Header{}, false, logger)
	encodingConfig := cosmoscmd.MakeEncodingConfig(app.ModuleBasics)

	return &TestKeeperState{
		T:              t,
		Logger:         logger,
		EncodingConfig: encodingConfig,
		TestDb:         db,
		StateStore:     stateStore,
		Ctx:            ctx,
	}
}

func (tks *TestKeeperState) LoadKVStores() {
	require.NoError(tks.T, tks.StateStore.LoadLatestVersion())
}

func (tks *TestKeeperState) GetParamsKeeper() paramskeeper.Keeper {
	if tks.ParamsKeeper == nil {
		panic("ParamsKeeper cannot be nil")
	}
	return *tks.ParamsKeeper
}

func (tks *TestKeeperState) GetAccountKeeper() authkeeper.AccountKeeper {
	if tks.AccountKeeper == nil {
		panic("AccountKeeper cannot be nil")
	}
	return *tks.AccountKeeper
}

func (tks *TestKeeperState) GetBankKeeper() bankkeeper.Keeper {
	if tks.BankKeeper == nil {
		panic("BankKeeper cannot be nil")
	}
	return tks.BankKeeper
}
