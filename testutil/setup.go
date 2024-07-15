package testutil

import (
	"testing"

	tmdb "github.com/cometbft/cometbft-db"
	"github.com/cometbft/cometbft/crypto/ed25519"
	"github.com/cometbft/cometbft/libs/log"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	"github.com/cosmos/cosmos-sdk/x/bank/testutil"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	stakingKeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/controller/types"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/app"
	"github.com/quasarlabs/quasarnode/testutil/keeper"
	"github.com/quasarlabs/quasarnode/testutil/mock"
	epochskeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	qtransferkeeper "github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	qvestingkeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	tfkeeper "github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
)

func init() {
	// Set prefixes
	accountPubKeyPrefix := app.AccountAddressPrefix + "pub"
	validatorAddressPrefix := app.AccountAddressPrefix + "valoper"
	validatorPubKeyPrefix := app.AccountAddressPrefix + "valoperpub"
	consNodeAddressPrefix := app.AccountAddressPrefix + "valcons"
	consNodePubKeyPrefix := app.AccountAddressPrefix + "valconspub"

	// Set and seal config
	config := sdk.GetConfig()
	config.SetBech32PrefixForAccount(app.AccountAddressPrefix, accountPubKeyPrefix)
	config.SetBech32PrefixForValidator(validatorAddressPrefix, validatorPubKeyPrefix)
	config.SetBech32PrefixForConsensusNode(consNodeAddressPrefix, consNodePubKeyPrefix)
	config.Seal()
}
func CreateRandomAccounts(numAccts int) []sdk.AccAddress {
	testAddrs := make([]sdk.AccAddress, numAccts)
	for i := 0; i < numAccts; i++ {
		pk := ed25519.GenPrivKey().PubKey()
		testAddrs[i] = sdk.AccAddress(pk.Address())
	}

	return testAddrs
}

// FundAcc funds target address with specified amount.
func (ts *TestSetup) FundAcc(t testing.TB, acc sdk.AccAddress, amounts sdk.Coins) {
	// TODO - implement alternative solution to the simapp.FundAcc
	err := testutil.FundAccount(ts.Keepers.BankKeeper, ts.Ctx, acc, amounts)
	require.NoError(t, err)
}

// FundModuleAcc funds target modules with specified amount.
func (ts *TestSetup) FundModuleAcc(t testing.TB, moduleName string, amounts sdk.Coins) {
	// TODO - implement alternative solution to the simapp.FundAcc
	// err := simapp.FundModuleAccount(ts.Keepers.BankKeeper, ts.Ctx, moduleName, amounts)
	// require.NoError(t, err)
	require.NoError(t, nil)
}

func (ts *TestSetup) MintCoins(t testing.TB, coins sdk.Coins) {
	err := ts.Keepers.BankKeeper.MintCoins(ts.Ctx, minttypes.ModuleName, coins)
	require.NoError(t, err)
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
	encodingConfig := app.MakeEncodingConfig()

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
	ics4WrapperMock := mock.NewMockICS4Wrapper(ctl)
	ibcPortKeeperMock := mock.NewMockPortKeeper(ctl)
	// Set BindPort method for mock and return a mock capability
	ibcPortKeeperMock.EXPECT().BindPort(gomock.Any(), gomock.Any()).AnyTimes().Return(capabilitytypes.NewCapability(1))
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
	stakingKeeper := factory.StakingKeeper(accountKeeper, bankKeeper)
	distrKeeper := factory.DistributionKeeper(accountKeeper, bankKeeper, stakingKeeper, "feeCollectorName")
	qosmoScopedKeeper := capabilityKeeper.ScopeToModule(qosmotypes.SubModuleName)

	qoracleKeeper := factory.QoracleKeeper(paramsKeeper, authtypes.NewModuleAddress(govtypes.ModuleName).String())
	qosmosisKeeper := factory.QosmosisKeeper(paramsKeeper, authtypes.NewModuleAddress(govtypes.ModuleName).String(), ibcClientKeeperMock, ics4WrapperMock, ibcChannelKeeperMock, ibcPortKeeperMock, qosmoScopedKeeper, qoracleKeeper)
	qoracleKeeper.RegisterPoolOracle(qosmosisKeeper)
	qoracleKeeper.Seal()
	qtransferkeeper := factory.QTransferKeeper(paramsKeeper, accountKeeper)
	qvestingKeeper := factory.QVestingKeeper(paramsKeeper, accountKeeper, bankKeeper)
	tfKeeper := factory.TfKeeper(paramsKeeper, accountKeeper, bankKeeper, distrKeeper)

	// Note: the relative order of LoadLatestVersion and Set*DefaultParams is important.
	// Setting params before loading stores causes store does not exist error.
	// LoadLatestVersion must not be called again after setting params, as reloading stores clears set params.

	require.NoError(t, factory.StateStore.LoadLatestVersion())

	factory.SetQoracleDefaultParams(qoracleKeeper)
	factory.SetQosmosisDefaultParams(qosmosisKeeper)
	testAccts := CreateRandomAccounts(3)

	//  Init Genesis of Keepers

	distrGendata := distrtypes.GenesisState{Params: distrtypes.DefaultParams()}
	distrKeeper.InitGenesis(ctx, distrGendata)
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
			QoracleKeeper:    qoracleKeeper,
			QosmosisKeeper:   qosmosisKeeper,
			QTransfer:        qtransferkeeper,
			QVestingKeeper:   qvestingKeeper,
			TfKeeper:         tfKeeper,
		},
		TestAccs: testAccts,
	}
}

type TestSetup struct {
	Ctx sdk.Context
	Cdc codec.Codec

	Keepers  *testKeepers
	Mocks    *testMocks
	TestAccs []sdk.AccAddress
}

type testMocks struct {
	ICAControllerKeeperMock *mock.MockICAControllerKeeper
}

type testKeepers struct {
	ParamsKeeper      paramskeeper.Keeper
	EpochsKeeper      *epochskeeper.Keeper
	AccountKeeper     authkeeper.AccountKeeper
	BankKeeper        bankkeeper.Keeper
	StakingKeeper     stakingKeeper.Keeper
	DistributedKeeper distrkeeper.Keeper
	CapabilityKeeper  capabilitykeeper.Keeper
	QoracleKeeper     qoraclekeeper.Keeper
	QosmosisKeeper    qosmokeeper.Keeper
	QTransfer         qtransferkeeper.Keeper
	QVestingKeeper    qvestingkeeper.Keeper
	TfKeeper          tfkeeper.Keeper
}
