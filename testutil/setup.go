package testutil

import (
	"testing"

	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/controller/types"
	"github.com/golang/mock/gomock"
	"github.com/quasarlabs/quasarnode/app"
	appParams "github.com/quasarlabs/quasarnode/app/params"
	"github.com/quasarlabs/quasarnode/testutil/keeper"
	"github.com/quasarlabs/quasarnode/testutil/mock"
	epochskeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	orionkeeper "github.com/quasarlabs/quasarnode/x/orion/keeper"
	qbankkeeper "github.com/quasarlabs/quasarnode/x/qbank/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmdb "github.com/tendermint/tm-db"
)

func init() {
	cosmoscmd.SetPrefixes(appParams.AccountAddressPrefix)
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
	ibcClientKeeperMock := mock.NewMockClientKeeper(ctl)
	ibcChannelKeeperMock := mock.NewMockChannelKeeper(ctl)
	icaControllerKeeperMock := mock.NewMockICAControllerKeeper(ctl)
	ibcTransferKeeperMock := mock.NewMockIBCTransferKeeper(ctl)
	ics4WrapperMock := mock.NewMockICS4Wrapper(ctl)
	ibcPortKeeperMock := mock.NewMockPortKeeper(ctl)
	// Set BindPort method for mock and return a mock capability
	ibcPortKeeperMock.EXPECT().BindPort(gomock.Any(), gomock.Any()).AnyTimes().Return(capabilitytypes.NewCapability(1))
	ibcConnectionKeeperMock := mock.NewMockConnectionKeeper(ctl)
	// ibcClientKeeperMock := mock.NewMockClientKeeper(ctl)

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
	qoracleScopedKeeper := capabilityKeeper.ScopeToModule(qoracletypes.ModuleName)
	qoracleKeeper := factory.QoracleKeeper(paramsKeeper, ibcClientKeeperMock, ics4WrapperMock, ibcChannelKeeperMock, ibcPortKeeperMock, qoracleScopedKeeper)
	qbankKeeper := factory.QbankKeeper(paramsKeeper, bankKeeper, *epochsKeeper, qoracleKeeper)
	intergammKeeper := factory.IntergammKeeper(paramsKeeper, capabilityKeeper, ibcChannelKeeperMock, icaControllerKeeperMock, ibcTransferKeeperMock, ibcConnectionKeeperMock, ibcClientKeeperMock)
	orionKeeper := factory.OrionKeeper(paramsKeeper, accountKeeper, bankKeeper, qbankKeeper, qoracleKeeper, intergammKeeper, *epochsKeeper)

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
	InterGammKeeper  *intergammkeeper.Keeper
	OrionKeeper      orionkeeper.Keeper
}
