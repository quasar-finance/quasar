package apptesting

import (
	"encoding/json"
	"fmt"
	"os"
	"testing"
	"time"

	coreheader "cosmossdk.io/core/header"
	"cosmossdk.io/log"
	"cosmossdk.io/math"
	storemetrics "cosmossdk.io/store/metrics"
	"cosmossdk.io/store/rootmulti"
	storetypes "cosmossdk.io/store/types"
	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cometbft/cometbft/crypto/ed25519"
	cmtproto "github.com/cometbft/cometbft/proto/tendermint/types"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/client"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	moduletestutil "github.com/cosmos/cosmos-sdk/types/module/testutil"
	"github.com/cosmos/cosmos-sdk/types/tx/signing"
	authsigning "github.com/cosmos/cosmos-sdk/x/auth/signing"
	"github.com/cosmos/cosmos-sdk/x/authz"
	authzmod "github.com/cosmos/cosmos-sdk/x/authz/module"
	"github.com/cosmos/cosmos-sdk/x/bank/testutil"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	"github.com/gogo/protobuf/proto"
	"github.com/quasar-finance/quasar/app"
	"github.com/quasar-finance/quasar/app/helpers"
	"github.com/stretchr/testify/require"
	"github.com/stretchr/testify/suite"
)

type KeeperTestHelper struct {
	suite.Suite

	// defaults to false,
	// set to true if any method that potentially alters baseapp/abci is used.
	// this controls whether or not we can reuse the app instance, or have to set a new one.
	hasUsedAbci bool
	// defaults to false, set to true if we want to use a new app instance with caching enabled.
	// then on new setup test call, we just drop the current cache.
	// this is not always enabled, because some tests may take a painful performance hit due to CacheKv.
	withCaching bool

	App         *app.QuasarApp
	Ctx         sdk.Context
	QueryHelper *baseapp.QueryServiceTestHelper
	TestAccs    []sdk.AccAddress
}

var (
	TestDenom            = "ustake"
	TestFundAmount       = math.NewInt(10000000000000000)
	SecondaryAmount      = math.NewInt(10000000000000000)
	baseTestAccts        = []sdk.AccAddress{}
	defaultTestStartTime = time.Now().UTC()
	testDescription      = stakingtypes.NewDescription("test_moniker", "test_identity", "test_website", "test_security_contact", "test_details")
)

func init() {
	baseTestAccts = CreateRandomAccounts(3)
}

// Setup sets up basic environment for suite (App, Ctx, and test accounts)
// preserves the caching enabled/disabled state.
func (s *KeeperTestHelper) Setup() {
	dir, err := os.MkdirTemp("", "osmosisd-test-home")
	if err != nil {
		panic(fmt.Sprintf("failed creating temporary directory: %v", err))
	}
	s.T().Cleanup(func() { os.RemoveAll(dir); s.withCaching = false })
	s.App = helpers.SetupWithCustomHome(false, dir)
	s.setupGeneral()

	// Manually set validator signing info, otherwise we panic
	vals, err := s.App.StakingKeeper.GetAllValidators(s.Ctx)
	if err != nil {
		panic(err)
	}
	for _, val := range vals {
		consAddr, _ := val.GetConsAddr()
		signingInfo := slashingtypes.NewValidatorSigningInfo(
			consAddr,
			s.Ctx.BlockHeight(),
			0,
			time.Unix(0, 0),
			false,
			0,
		)
		err := s.App.SlashingKeeper.SetValidatorSigningInfo(s.Ctx, consAddr, signingInfo)
		if err != nil {
			panic(err)
		}
	}
}

func (s *KeeperTestHelper) SetupWithCustomChainId(chainId string) {
	dir, err := os.MkdirTemp("", "quasard-test-home")
	if err != nil {
		panic(fmt.Sprintf("failed creating temporary directory: %v", err))
	}
	s.T().Cleanup(func() { os.RemoveAll(dir); s.withCaching = false })
	s.App = helpers.SetupWithCustomHomeAndChainId(false, dir, chainId)
	s.setupGeneralCustomChainId(chainId)

	// Manually set validator signing info, otherwise we panic
	vals, err := s.App.StakingKeeper.GetAllValidators(s.Ctx)
	if err != nil {
		panic(err)
	}
	for _, val := range vals {
		consAddr, _ := val.GetConsAddr()
		signingInfo := slashingtypes.NewValidatorSigningInfo(
			consAddr,
			s.Ctx.BlockHeight(),
			0,
			time.Unix(0, 0),
			false,
			0,
		)
		err := s.App.SlashingKeeper.SetValidatorSigningInfo(s.Ctx, consAddr, signingInfo)
		if err != nil {
			panic(err)
		}
	}
}

// resets the test environment
// requires that all commits go through helpers in s.
// On first reset, will instantiate a new app, with caching enabled.
// NOTE: If you are using ABCI methods, usage of Reset vs Setup has not been well tested.
// It is believed to work, but if you get an odd error, try changing the call to this for setup to sanity check.
// what's supposed to happen is a new setup call, and reset just does that in such a case.
func (s *KeeperTestHelper) Reset() {
	if s.hasUsedAbci || !s.withCaching {
		s.withCaching = true
		s.Setup()
	} else {
		s.setupGeneral()
	}
}

func (s *KeeperTestHelper) SetupWithLevelDb() func() {
	app, cleanup := helpers.SetupTestingAppWithLevelDb(false)
	s.App = app
	s.setupGeneral()
	return cleanup
}

func (s *KeeperTestHelper) setupGeneral() {
	s.setupGeneralCustomChainId("quasar-1")
}

func (s *KeeperTestHelper) setupGeneralCustomChainId(chainId string) {
	s.Ctx = s.App.BaseApp.NewContextLegacy(false, cmtproto.Header{Height: 1, ChainID: chainId, Time: defaultTestStartTime})
	if s.withCaching {
		s.Ctx, _ = s.Ctx.CacheContext()
	}
	s.QueryHelper = &baseapp.QueryServiceTestHelper{
		GRPCQueryRouter: s.App.GRPCQueryRouter(),
		Ctx:             s.Ctx,
	}

	s.SetEpochStartTime()
	s.TestAccs = []sdk.AccAddress{}
	s.TestAccs = append(s.TestAccs, baseTestAccts...)
	s.hasUsedAbci = false
}

func (s *KeeperTestHelper) SetupTestForInitGenesis() {
	// Setting to True, leads to init genesis not running
	s.App = helpers.Setup(true)
	s.Ctx = s.App.BaseApp.NewContextLegacy(true, cmtproto.Header{})
	s.hasUsedAbci = true
}

// RunTestCaseWithoutStateUpdates runs the testcase as a callback with the given name.
// Does not persist any state changes. This is useful when test suite uses common state setup
// but desures each test case to be run in isolation.
func (s *KeeperTestHelper) RunTestCaseWithoutStateUpdates(name string, cb func(t *testing.T)) {
	originalCtx := s.Ctx
	s.Ctx, _ = s.Ctx.CacheContext()

	s.T().Run(name, cb)

	s.Ctx = originalCtx
}

func (s *KeeperTestHelper) SetEpochStartTime() {
	epochsKeeper := s.App.EpochsKeeper

	for _, epoch := range epochsKeeper.AllEpochInfos(s.Ctx) {
		epoch.StartTime = s.Ctx.BlockTime()
		epochsKeeper.DeleteEpochInfo(s.Ctx, epoch.Identifier)
		err := epochsKeeper.AddEpochInfo(s.Ctx, epoch)
		if err != nil {
			panic(err)
		}
	}
}

// CreateTestContext creates a test context.
func (s *KeeperTestHelper) CreateTestContext() sdk.Context {
	ctx, _ := s.CreateTestContextWithMultiStore()
	return ctx
}

// CreateTestContextWithMultiStore creates a test context and returns it together with multi store.
func (s *KeeperTestHelper) CreateTestContextWithMultiStore() (sdk.Context, storetypes.CommitMultiStore) {
	db := dbm.NewMemDB()
	logger := log.NewNopLogger()

	ms := rootmulti.NewStore(db, logger, storemetrics.NewNoOpMetrics())

	return sdk.NewContext(ms, cmtproto.Header{}, false, logger), ms
}

// CreateTestContext creates a test context.
func (s *KeeperTestHelper) Commit() {
	_, err := s.App.FinalizeBlock(&abci.RequestFinalizeBlock{Height: s.Ctx.BlockHeight(), Time: s.Ctx.BlockTime()})
	if err != nil {
		panic(err)
	}
	_, err = s.App.Commit()
	if err != nil {
		panic(err)
	}

	newBlockTime := s.Ctx.BlockTime().Add(time.Second)

	header := s.Ctx.BlockHeader()
	header.Time = newBlockTime
	header.Height++

	s.Ctx = s.App.BaseApp.NewUncachedContext(false, header).WithHeaderInfo(coreheader.Info{
		Height: header.Height,
		Time:   header.Time,
	})

	s.hasUsedAbci = true
}

// FundAcc funds target address with specified amount.
func (s *KeeperTestHelper) FundAcc(acc sdk.AccAddress, amounts sdk.Coins) {
	err := testutil.FundAccount(s.Ctx, s.App.BankKeeper, acc, amounts)
	s.Require().NoError(err)
}

// FundModuleAcc funds target modules with specified amount.
func (s *KeeperTestHelper) FundModuleAcc(moduleName string, amounts sdk.Coins) {
	err := testutil.FundModuleAccount(s.Ctx, s.App.BankKeeper, moduleName, amounts)
	s.Require().NoError(err)
}

func (s *KeeperTestHelper) MintCoins(coins sdk.Coins) {
	err := s.App.BankKeeper.MintCoins(s.Ctx, minttypes.ModuleName, coins)
	s.Require().NoError(err)
}

// SetupValidator sets up a validator and returns the ValAddress.
func (s *KeeperTestHelper) SetupValidator(bondStatus stakingtypes.BondStatus) sdk.ValAddress {
	valPub := secp256k1.GenPrivKey().PubKey()
	valAddr := sdk.ValAddress(valPub.Address())
	stakingParams, err := s.App.StakingKeeper.GetParams(s.Ctx)
	s.Require().NoError(err)
	bondDenom := stakingParams.BondDenom
	bondAmt := sdk.DefaultPowerReduction
	selfBond := sdk.NewCoins(sdk.Coin{Amount: bondAmt, Denom: bondDenom})

	s.FundAcc(sdk.AccAddress(valAddr), selfBond)

	stakingCoin := sdk.Coin{Denom: sdk.DefaultBondDenom, Amount: selfBond[0].Amount}
	ZeroCommission := stakingtypes.NewCommissionRates(math.LegacyZeroDec(), math.LegacyZeroDec(), math.LegacyZeroDec())
	valCreateMsg, err := stakingtypes.NewMsgCreateValidator(valAddr.String(), valPub, stakingCoin, testDescription, ZeroCommission, math.OneInt())
	s.Require().NoError(err)
	stakingMsgSvr := stakingkeeper.NewMsgServerImpl(s.App.StakingKeeper)
	res, err := stakingMsgSvr.CreateValidator(s.Ctx, valCreateMsg)
	s.Require().NoError(err)
	s.Require().NotNil(res)

	val, err := s.App.StakingKeeper.GetValidator(s.Ctx, valAddr)
	s.Require().NoError(err)

	val = val.UpdateStatus(bondStatus)
	err = s.App.StakingKeeper.SetValidator(s.Ctx, val)
	s.Require().NoError(err)

	consAddr, err := val.GetConsAddr()
	s.Suite.Require().NoError(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		s.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	err = s.App.SlashingKeeper.SetValidatorSigningInfo(s.Ctx, consAddr, signingInfo)
	s.Require().NoError(err)

	return valAddr
}

// SetupMultipleValidators setups "numValidator" validators and returns their address in string
func (s *KeeperTestHelper) SetupMultipleValidators(numValidator int) []string {
	valAddrs := []string{}
	for i := 0; i < numValidator; i++ {
		valAddr := s.SetupValidator(stakingtypes.Bonded)
		valAddrs = append(valAddrs, valAddr.String())
	}
	return valAddrs
}

// BeginNewBlock starts a new block.
func (s *KeeperTestHelper) BeginNewBlock() {
	var valAddr []byte

	validators, err := s.App.StakingKeeper.GetAllValidators(s.Ctx)
	s.Require().NoError(err)
	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		s.Require().NoError(err)
		valAddr = valAddrFancy
	} else {
		valAddrFancy := s.SetupValidator(stakingtypes.Bonded)
		validator, _ := s.App.StakingKeeper.GetValidator(s.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2
	}

	s.BeginNewBlockWithProposer(valAddr)
}

// BeginNewBlockWithProposer begins a new block with a proposer.
func (s *KeeperTestHelper) BeginNewBlockWithProposer(proposer sdk.ValAddress) {
	validator, err := s.App.StakingKeeper.GetValidator(s.Ctx, proposer)
	s.Assert().NoError(err)

	valConsAddr, err := validator.GetConsAddr()
	s.Require().NoError(err)

	valAddr := valConsAddr

	//Note : when we add any logic executed over some epochs use below logic
	//epochIdentifier := s.App.EpochsKeeper.GetEpochInfo(s.Ctx,)
	//epoch := s.App.EpochsKeeper.GetEpochInfo(s.Ctx, epochIdentifier)
	newBlockTime := s.Ctx.BlockTime().Add(5 * time.Second)
	//if executeNextEpoch {
	//	newBlockTime = s.Ctx.BlockTime().Add(epoch.Duration).Add(time.Second)
	//}

	header := cmtproto.Header{Height: s.Ctx.BlockHeight() + 1, Time: newBlockTime}
	s.Ctx = s.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(s.Ctx.BlockHeight() + 1)
	voteInfos := []abci.VoteInfo{{
		Validator:   abci.Validator{Address: valAddr, Power: 1000},
		BlockIdFlag: cmtproto.BlockIDFlagCommit,
	}}
	s.Ctx = s.Ctx.WithVoteInfos(voteInfos)

	fmt.Println("beginning block ", s.Ctx.BlockHeight())

	_, err = s.App.BeginBlocker(s.Ctx)
	s.Require().NoError(err)

	s.Ctx = s.App.NewContextLegacy(false, header)
	s.hasUsedAbci = true
}

// EndBlock ends the block, and runs commit
func (s *KeeperTestHelper) EndBlock() {
	_, err := s.App.EndBlocker(s.Ctx)
	s.Require().NoError(err)
	s.hasUsedAbci = true
}

func (s *KeeperTestHelper) RunMsg(msg sdk.Msg) (*sdk.Result, error) {
	router := s.App.GetBaseApp().MsgServiceRouter()
	if handler := router.Handler(msg); handler != nil {
		// ADR 031 request type routing
		return handler(s.Ctx, msg)
	}
	s.FailNow("msg %v could not be ran", msg)
	s.hasUsedAbci = true
	return nil, fmt.Errorf("msg %v could not be ran", msg)
}

// AllocateRewardsToValidator allocates reward tokens to a distribution module then allocates rewards to the validator address.
func (s *KeeperTestHelper) AllocateRewardsToValidator(valAddr sdk.ValAddress, rewardAmt math.Int) {
	validator, err := s.App.StakingKeeper.GetValidator(s.Ctx, valAddr)
	s.Require().NoError(err)

	// allocate reward tokens to distribution module
	coins := sdk.Coins{sdk.NewCoin(sdk.DefaultBondDenom, rewardAmt)}
	err = testutil.FundModuleAccount(s.Ctx, s.App.BankKeeper, distrtypes.ModuleName, coins)
	s.Require().NoError(err)

	// allocate rewards to validator
	s.Ctx = s.Ctx.WithBlockHeight(s.Ctx.BlockHeight() + 1)
	decTokens := sdk.DecCoins{{Denom: sdk.DefaultBondDenom, Amount: math.LegacyMustNewDecFromStr("20000")}}
	err = s.App.DistrKeeper.AllocateTokensToValidator(s.Ctx, validator, decTokens)
	s.Require().NoError(err)
}

// BuildTx builds a transaction.
func (s *KeeperTestHelper) BuildTx(
	txBuilder client.TxBuilder,
	msgs []sdk.Msg,
	sigV2 signing.SignatureV2,
	memo string, txFee sdk.Coins,
	gasLimit uint64,
) authsigning.Tx {
	err := txBuilder.SetMsgs(msgs[0])
	s.Require().NoError(err)

	err = txBuilder.SetSignatures(sigV2)
	s.Require().NoError(err)

	txBuilder.SetMemo(memo)
	txBuilder.SetFeeAmount(txFee)
	txBuilder.SetGasLimit(gasLimit)

	return txBuilder.GetTx()
}

// StateNotAltered validates that app state is not altered. Fails if it is.
func (s *KeeperTestHelper) StateNotAltered() {
	oldState := s.App.ExportState(s.Ctx)
	s.App.CommitMultiStore().Commit()
	newState := s.App.ExportState(s.Ctx)
	s.Require().Equal(oldState, newState)
	s.hasUsedAbci = true
}

func (s *KeeperTestHelper) SkipIfWSL() {
	SkipIfWSL(s.T())
}

// SkipIfWSL skips tests if running on WSL
// This is a workaround to enable quickly running full unit test suite locally
// on WSL without failures. The failures are stemming from trying to upload
// wasm code. An OS permissioning issue.
func SkipIfWSL(t *testing.T) {
	t.Helper()
	skip := os.Getenv("SKIP_WASM_WSL_TESTS")
	fmt.Println("SKIP_WASM_WSL_TESTS", skip)
	if skip == "true" {
		t.Skip("Skipping Wasm tests")
	}
}

// CreateRandomAccounts is a function return a list of randomly generated AccAddresses
func CreateRandomAccounts(numAccts int) []sdk.AccAddress {
	testAddrs := make([]sdk.AccAddress, numAccts)
	for i := 0; i < numAccts; i++ {
		pk := ed25519.GenPrivKey().PubKey()
		testAddrs[i] = sdk.AccAddress(pk.Address())
	}

	return testAddrs
}

func TestMessageAuthzSerialization(t *testing.T, msg sdk.Msg, module module.AppModuleBasic) {
	someDate := time.Date(1, 1, 1, 1, 1, 1, 1, time.UTC)
	const (
		mockGranter string = "cosmos1abc"
		mockGrantee string = "cosmos1xyz"
	)

	var (
		mockMsgGrant  authz.MsgGrant
		mockMsgRevoke authz.MsgRevoke
		mockMsgExec   authz.MsgExec
	)

	// mutates mockMsg
	testSerDeser := func(msg proto.Message, mockMsg proto.Message) {
		encCfg := moduletestutil.MakeTestEncodingConfig(authzmod.AppModuleBasic{}, module)
		msgGrantBytes := json.RawMessage(sdk.MustSortJSON(encCfg.Codec.MustMarshalJSON(msg)))
		err := encCfg.Codec.UnmarshalJSON(msgGrantBytes, mockMsg)
		require.NoError(t, err)
	}

	// Authz: Grant Msg
	typeURL := sdk.MsgTypeURL(msg)
	expiryTime := someDate.Add(time.Hour)
	grant, err := authz.NewGrant(someDate, authz.NewGenericAuthorization(typeURL), &expiryTime)
	require.NoError(t, err)

	msgGrant := authz.MsgGrant{Granter: mockGranter, Grantee: mockGrantee, Grant: grant}
	testSerDeser(&msgGrant, &mockMsgGrant)

	// Authz: Revoke Msg
	msgRevoke := authz.MsgRevoke{Granter: mockGranter, Grantee: mockGrantee, MsgTypeUrl: typeURL}
	testSerDeser(&msgRevoke, &mockMsgRevoke)

	// Authz: Exec Msg
	msgAny, err := cdctypes.NewAnyWithValue(msg)
	require.NoError(t, err)
	msgExec := authz.MsgExec{Grantee: mockGrantee, Msgs: []*cdctypes.Any{msgAny}}
	testSerDeser(&msgExec, &mockMsgExec)
	require.Equal(t, msgExec.Msgs[0].Value, mockMsgExec.Msgs[0].Value)
}

func GenerateTestAddrs() (string, string) {
	pk1 := ed25519.GenPrivKey().PubKey()
	validAddr := sdk.AccAddress(pk1.Address()).String()
	invalidAddr := sdk.AccAddress("invalid").String()
	return validAddr, invalidAddr
}

// RequireDecCoinsSlice compares two slices of DecCoins
func (s *KeeperTestHelper) RequireDecCoinsSlice(expected, actual []sdk.DecCoins) {
	s.Require().Equal(len(expected), len(actual))
	for i := range actual {
		s.Require().Equal(expected[i].String(), actual[i].String())
	}
}
