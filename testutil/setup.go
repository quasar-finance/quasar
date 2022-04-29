package testutil

import (
	"testing"

	"github.com/abag/quasarnode/app"
	"github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/mock"
	epochskeeper "github.com/abag/quasarnode/x/epochs/keeper"
	intergammkeeper "github.com/abag/quasarnode/x/intergamm/keeper"
	orionkeeper "github.com/abag/quasarnode/x/orion/keeper"
	qbankkeeper "github.com/abag/quasarnode/x/qbank/keeper"
	qoraclekeeper "github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/controller/types"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/require"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmdb "github.com/tendermint/tm-db"
)

func init() {
	cosmoscmd.SetPrefixes(app.AccountAddressPrefix)
}

func NewTestSetup(t testing.TB, controller ...*gomock.Controller) *TestSetup {
	// Test setup params

	logger := log.TestingLogger()
	// Use nop logger if logging becomes too verbose for test output
	// logger := log.NewNopLogger()
	logger.Debug("creating test setup")

	db := tmdb.NewMemDB()
	stateStore := store.NewCommitMultiStore(db)

	ctx := sdk.NewContext(stateStore, tmproto.Header{}, false, logger)
	encodingConfig := cosmoscmd.MakeEncodingConfig(app.ModuleBasics)

	// Mocks

	// If no controller is given, no mock is needed so we don't need to check that mocks were called
	var ctl *gomock.Controller
	switch len(controller) {
	case 0:
		ctl = gomock.NewController(t)
	default:
		ctl = controller[0]
	}
	icaControllerKeeperMock := mock.NewMockICAControllerKeeper(ctl)

	// Keepers

	// Create a factory first to easily create keepers
	factory := keeper.NewKeeperFactory(db, stateStore, ctx, encodingConfig)

	maccPerms := factory.TestModuleAccountPerms()
	blockedMaccAddresses := factory.BlockedModuleAccountAddrs(maccPerms)

	paramsKeeper := factory.ParamsKeeper()
	epochsKeeper := factory.EpochsKeeper(paramsKeeper)
	accountKeeper := factory.AccountKeeper(paramsKeeper, maccPerms)
	bankKeeper := factory.BankKeeper(paramsKeeper, accountKeeper, blockedMaccAddresses)
	capabilityKeeper := factory.CapabilityKeeper()
	capabilityKeeper.ScopeToModule(icacontrollertypes.SubModuleName)
	qbankKeeper := factory.QbankKeeper(paramsKeeper, bankKeeper)
	qoracleKeeper := factory.QoracleKeeper(paramsKeeper)
	intergammKeeper := factory.IntergammKeeper(paramsKeeper, capabilityKeeper, icaControllerKeeperMock)
	orionKeeper := factory.OrionKeeper(paramsKeeper, accountKeeper, bankKeeper, qbankKeeper, qoracleKeeper, intergammKeeper)

	// Note: the relative order of LoadLatestVersion and Set*DefaultParams is important.
	// Setting params before loading stores causes store does not exist error.
	// LoadLatestVersion must not be called again after setting params, as reloading stores clears set params.

	require.NoError(t, factory.StateStore.LoadLatestVersion())

	factory.SetQbankDefaultParams(qbankKeeper)
	factory.SetQoracleDefaultParams(qoracleKeeper)
	factory.SetIntergammDefaultParams(intergammKeeper)
	factory.SetOrionDefaultParams(orionKeeper)

	return &TestSetup{
		Ctx: ctx,
		Cdc: encodingConfig.Marshaler,

		Mocks: &testMocks{
			ICAControllerKeeperMock: icaControllerKeeperMock,
		},

		Keepers: &testKeepers{
			ParamsKeeper:     paramsKeeper,
			EpochsKeeper:     epochsKeeper,
			AccountKeeper:    accountKeeper,
			BankKeeper:       bankKeeper,
			CapabilityKeeper: capabilityKeeper,
			QbankKeeper:      qbankKeeper,
			QoracleKeeper:    qoracleKeeper,
			InterGammKeeper:  intergammKeeper,
			OrionKeeper:      orionKeeper,
		},
	}
}

type TestSetup struct {
	Ctx sdk.Context
	Cdc codec.Codec

	Keepers *testKeepers
	Mocks   *testMocks
}

type testMocks struct {
	ICAControllerKeeperMock *mock.MockICAControllerKeeper
}

type testKeepers struct {
	ParamsKeeper     paramskeeper.Keeper
	EpochsKeeper     *epochskeeper.Keeper
	AccountKeeper    authkeeper.AccountKeeper
	BankKeeper       bankkeeper.Keeper
	CapabilityKeeper capabilitykeeper.Keeper
	QbankKeeper      qbankkeeper.Keeper
	QoracleKeeper    qoraclekeeper.Keeper
	InterGammKeeper  intergammkeeper.Keeper
	OrionKeeper      orionkeeper.Keeper
}
