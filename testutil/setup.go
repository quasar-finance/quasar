package testutil

import (
	"testing"

	"cosmossdk.io/log"
	"cosmossdk.io/store"
	storemetrics "cosmossdk.io/store/metrics"
	"github.com/cometbft/cometbft/crypto/ed25519"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	"github.com/cosmos/cosmos-sdk/x/bank/testutil"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	stakingKeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	capabilitykeeper "github.com/cosmos/ibc-go/modules/capability/keeper"
	capabilitytypes "github.com/cosmos/ibc-go/modules/capability/types"
	icacontrollertypes "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/controller/types"
	"github.com/golang/mock/gomock"
	"github.com/quasar-finance/quasar/app"
	appparams "github.com/quasar-finance/quasar/app/params"
	"github.com/quasar-finance/quasar/testutil/keeper"
	"github.com/quasar-finance/quasar/testutil/mock"
	epochskeeper "github.com/quasar-finance/quasar/x/epochs/keeper"
	tfkeeper "github.com/quasar-finance/quasar/x/tokenfactory/keeper"
	"github.com/stretchr/testify/require"
)

func init() {
	// Set prefixes
	accountPubKeyPrefix := appparams.Bech32PrefixAccAddr + "pub"
	validatorAddressPrefix := appparams.Bech32PrefixAccAddr + "valoper"
	validatorPubKeyPrefix := appparams.Bech32PrefixAccAddr + "valoperpub"
	consNodeAddressPrefix := appparams.Bech32PrefixAccAddr + "valcons"
	consNodePubKeyPrefix := appparams.Bech32PrefixAccAddr + "valconspub"

	// Set and seal config
	config := sdk.GetConfig()
	config.SetBech32PrefixForAccount(appparams.Bech32PrefixAccAddr, accountPubKeyPrefix)
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
	err := testutil.FundAccount(ts.Ctx, ts.Keepers.BankKeeper, acc, amounts)
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

	logger := log.NewTestLogger(t)
	// Use nop logger if logging becomes too verbose for test output
	// logger := log.NewNopLogger()
	logger.Debug("creating test setup")

	db := dbm.NewMemDB()
	stateStore := store.NewCommitMultiStore(db, logger, storemetrics.NewNoOpMetrics())

	ctx := sdk.NewContext(stateStore, tmproto.Header{}, false, logger)
	encodingConfig := app.MakeEncodingConfig()

	// Mocks

	// If no controller is given, no mock is needed so we don't need to check that mocks were called
	var ctl *gomock.Controller
	controllerLength := len(controller)

	switch controllerLength {
	case 0:
		ctl = gomock.NewController(t)
	default:
		ctl = controller[0]
	}
	icaControllerKeeperMock := mock.NewMockICAControllerKeeper(ctl)
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
	tfKeeper := factory.TfKeeper(paramsKeeper, accountKeeper, bankKeeper, distrKeeper)

	// Note: the relative order of LoadLatestVersion and Set*DefaultParams is important.
	// Setting params before loading stores causes store does not exist error.
	// LoadLatestVersion must not be called again after setting params, as reloading stores clears set params.

	require.NoError(t, factory.StateStore.LoadLatestVersion())

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
	TfKeeper          tfkeeper.Keeper
}
