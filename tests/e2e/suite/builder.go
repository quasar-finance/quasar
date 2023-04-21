package suite

import (
	"context"
	"testing"

	dockerclient "github.com/docker/docker/client"
	testconfig "github.com/quasarlabs/quasarnode/tests/e2e/config"
	ibctest "github.com/strangelove-ventures/interchaintest/v4"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/relayer/rly"
	"github.com/strangelove-ventures/interchaintest/v4/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type Accounts struct {
	Authority                                                                             ibc.Wallet
	Owner                                                                                 ibc.Wallet
	NewOwner                                                                              ibc.Wallet
	MasterMinter                                                                          ibc.Wallet
	BondTest, BondTest1, BondTest2, BondTest3, BondTest4, BondTest5, BondTest6, BondTest7 ibc.Wallet
}

// E2ETestSuiteBuilder is a factory to simplify works behind running chains, configuring relayer and logging.
type E2ETestSuiteBuilder struct {
	t *testing.T

	quasar         *cosmos.CosmosChain
	QuasarAccounts Accounts

	cosmos         *cosmos.CosmosChain
	CosmosAccounts Accounts

	osmosis         *cosmos.CosmosChain
	OsmosisAccounts Accounts

	relayer        *rly.CosmosRelayer
	paths          map[string]path
	rep            *testreporter.Reporter
	erep           *testreporter.RelayerExecReporter
	automatedRelay bool

	dockerClient *dockerclient.Client
	networkID    string

	ic     *ibctest.Interchain
	logger *zap.Logger
	built  bool
}

// NewE2ETestSuiteBuilder returns a new E2ETestSuiteBuilder.
func NewE2ETestSuiteBuilder(t *testing.T) *E2ETestSuiteBuilder {
	E2EBuilder := &E2ETestSuiteBuilder{}
	logger := zaptest.NewLogger(t)

	ctx := context.Background()

	chainConfig := ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "quasar",
		ChainID: "quasar",
		Images: []ibc.DockerImage{
			{
				Repository: "quasar",
				Version:    "local",
			},
		},
		Bin:            "quasarnoded",
		Bech32Prefix:   "quasar",
		CoinType:       "118",
		Denom:          "uqsr",
		GasPrices:      "0.01uqsr",
		GasAdjustment:  1.3,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		PreGenesis: func(cc ibc.ChainConfig) (err error) {
			val := E2EBuilder.quasar.Validators[0]
			E2EBuilder.QuasarAccounts, err = quasarPreGenesis(ctx, val)
			return err
		},
		ModifyGenesis: modifyGenesis(
			modifyGenesisSetVotingPeriod(VotingPeriod),
		),
	}

	E2EBuilder.quasar = cosmos.NewCosmosChain(
		t.Name(),
		chainConfig,
		testconfig.DefaultNumValidators,
		testconfig.DefaultNumNodes,
		logger)

	dockerClient, networkID := ibctest.DockerSetup(t)
	relayer := rly.NewCosmosRelayer(logger, t.Name(), dockerClient, networkID)
	rep := testreporter.NewNopReporter()
	erep := rep.RelayerExecReporter(t)

	E2EBuilder.t = t
	E2EBuilder.relayer = relayer
	E2EBuilder.paths = map[string]path{}
	E2EBuilder.rep = rep
	E2EBuilder.erep = erep
	E2EBuilder.dockerClient = dockerClient
	E2EBuilder.networkID = networkID
	E2EBuilder.ic = ibctest.NewInterchain().AddChain(E2EBuilder.quasar).AddRelayer(relayer, "relayer").WithLog(logger)
	E2EBuilder.logger = logger

	return E2EBuilder
}

// Quasar returns the quasar chain instance.
func (b *E2ETestSuiteBuilder) Quasar() *cosmos.CosmosChain {
	require.NotNil(b.t, b.quasar)
	return b.quasar
}

// UseCosmos notifies the builder to build a test suite with an instance of cosmos chain.
func (b *E2ETestSuiteBuilder) UseCosmos() {
	b.checkBuilt()

	b.cosmos = cosmos.NewCosmosChain(
		b.t.Name(),
		testconfig.CosmosChain,
		testconfig.DefaultNumValidators,
		testconfig.DefaultNumNodes,
		b.logger)

	b.ic.AddChain(b.cosmos)
}

// Cosmos returns the cosmos chain instance. It fails the test if used before calling UseCosmos.
func (b *E2ETestSuiteBuilder) Cosmos() *cosmos.CosmosChain {
	require.NotNil(b.t, b.cosmos)
	return b.cosmos
}

// UseOsmosis notifies the builder to build a test suite with an instance of osmosis chain.
func (b *E2ETestSuiteBuilder) UseOsmosis() {
	b.checkBuilt()

	ctx := context.Background()

	osmosisConfig := ibc.ChainConfig{
		Type:    "cosmos",
		Name:    "osmosis",
		ChainID: "osmosis",
		Images: []ibc.DockerImage{
			{
				Repository: "osmosis",
				Version:    "local",
			},
		},
		Bin:            "osmosisd",
		Bech32Prefix:   "osmo",
		Denom:          "uosmo",
		CoinType:       "118",
		GasPrices:      "0.01uosmo",
		GasAdjustment:  1.5,
		TrustingPeriod: "508h",
		NoHostMount:    false,
		PreGenesis: func(cc ibc.ChainConfig) (err error) {
			val := b.osmosis.Validators[0]
			b.OsmosisAccounts, err = osmosisPreGenesis(ctx, val)
			return err
		},
		ModifyGenesis: modifyGenesis(
			modifyGenesisICAModule(
				true,
				[]string{
					"/ibc.applications.transfer.v1.MsgTransfer",
					"/cosmos.bank.v1beta1.MsgSend",
					"/cosmos.staking.v1beta1.MsgDelegate",
					"/cosmos.staking.v1beta1.MsgBeginRedelegate",
					"/cosmos.staking.v1beta1.MsgCreateValidator",
					"/cosmos.staking.v1beta1.MsgEditValidator",
					"/cosmos.staking.v1beta1.MsgUndelegate",
					"/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward",
					"/cosmos.distribution.v1beta1.MsgSetWithdrawAddress",
					"/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission",
					"/cosmos.distribution.v1beta1.MsgFundCommunityPool",
					"/cosmos.gov.v1beta1.MsgVote",
					"/osmosis.gamm.v1beta1.MsgJoinPool",
					"/osmosis.gamm.v1beta1.MsgExitPool",
					"/osmosis.gamm.v1beta1.MsgSwapExactAmountIn",
					"/osmosis.gamm.v1beta1.MsgSwapExactAmountOut",
					"/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn",
					"/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut",
					"/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut",
					"/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn",
					"/osmosis.lockup.MsgBeginUnlocking",
					"/osmosis.lockup.MsgLockTokens",
					"/osmosis.superfluid.MsgSuperfluidUnbondLock",
				},
				"icahost",
			),
			modifyGenesisICQModule(
				true,
				[]string{
					"/ibc.applications.transfer.v1.Query/DenomTrace",
					"/cosmos.auth.v1beta1.Query/Account",
					"/cosmos.auth.v1beta1.Query/Params",
					"/cosmos.bank.v1beta1.Query/Balance",
					"/cosmos.bank.v1beta1.Query/DenomMetadata",
					"/cosmos.bank.v1beta1.Query/Params",
					"/cosmos.bank.v1beta1.Query/SupplyOf",
					"/cosmos.distribution.v1beta1.Query/Params",
					"/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
					"/cosmos.distribution.v1beta1.Query/ValidatorCommission",
					"/cosmos.gov.v1beta1.Query/Deposit",
					"/cosmos.gov.v1beta1.Query/Params",
					"/cosmos.gov.v1beta1.Query/Vote",
					"/cosmos.slashing.v1beta1.Query/Params",
					"/cosmos.slashing.v1beta1.Query/SigningInfo",
					"/cosmos.staking.v1beta1.Query/Delegation",
					"/cosmos.staking.v1beta1.Query/Params",
					"/cosmos.staking.v1beta1.Query/Validator",
					"/osmosis.epochs.v1beta1.Query/EpochInfos",
					"/osmosis.epochs.v1beta1.Query/CurrentEpoch",
					"/osmosis.gamm.v1beta1.Query/NumPools",
					"/osmosis.gamm.v1beta1.Query/TotalLiquidity",
					"/osmosis.gamm.v1beta1.Query/Pool",
					"/osmosis.gamm.v1beta1.Query/PoolParams",
					"/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
					"/osmosis.gamm.v1beta1.Query/TotalShares",
					"/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares",
					"/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares",
					"/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares",
					"/osmosis.gamm.v1beta1.Query/PoolType",
					"/osmosis.gamm.v2.Query/SpotPrice",
					"/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
					"/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
					"/osmosis.incentives.Query/ModuleToDistributeCoins",
					"/osmosis.incentives.Query/LockableDurations",
					"/osmosis.lockup.Query/ModuleBalance",
					"/osmosis.lockup.Query/ModuleLockedAmount",
					"/osmosis.lockup.Query/AccountUnlockableCoins",
					"/osmosis.lockup.Query/AccountUnlockingCoins",
					"/osmosis.lockup.Query/LockedDenom",
					"/osmosis.lockup.Query/LockedByID",
					"/osmosis.lockup.Query/NextLockID",
					"/osmosis.mint.v1beta1.Query/EpochProvisions",
					"/osmosis.mint.v1beta1.Query/Params",
					"/osmosis.poolincentives.v1beta1.Query/GaugeIds",
					"/osmosis.superfluid.Query/Params",
					"/osmosis.superfluid.Query/AssetType",
					"/osmosis.superfluid.Query/AllAssets",
					"/osmosis.superfluid.Query/AssetMultiplier",
					"/osmosis.poolmanager.v1beta1.Query/NumPools",
					"/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
					"/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
					"/osmosis.txfees.v1beta1.Query/FeeTokens",
					"/osmosis.txfees.v1beta1.Query/DenomSpotPrice",
					"/osmosis.txfees.v1beta1.Query/DenomPoolId",
					"/osmosis.txfees.v1beta1.Query/BaseDenom",
					"/osmosis.tokenfactory.v1beta1.Query/Params",
					"/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata",
					"/osmosis.twap.v1beta1.Query/ArithmeticTwap",
					"/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow",
					"/osmosis.twap.v1beta1.Query/GeometricTwap",
					"/osmosis.twap.v1beta1.Query/GeometricTwapToNow",
					"/osmosis.twap.v1beta1.Query/Params",
					"/osmosis.downtimedetector.v1beta1.Query/RecoveredSinceDowntimeOfLength",
				},
				"icqhost",
			),
			modifyMintModule(),
			modifyIncentivesModule(),
			modifyPoolIncentivesModule(),
		),
	}

	b.osmosis = cosmos.NewCosmosChain(
		b.t.Name(),
		osmosisConfig,
		testconfig.DefaultNumValidators,
		testconfig.DefaultNumNodes,
		b.logger,
	)

	b.ic.AddChain(b.osmosis)
}

// Cosmos returns the cosmos chain instance. It will fail the test if used before calling UseOsmosis.
func (b *E2ETestSuiteBuilder) Osmosis() *cosmos.CosmosChain {
	require.NotNil(b.t, b.osmosis)
	return b.osmosis
}

// Link creates a pair of ibc clients, connection and a default transfer channel between chain1 and chain2.
func (b *E2ETestSuiteBuilder) Link(pathName string) {
	b.checkBuilt()

	b.ic.AddLink(ibctest.InterchainLink{
		Chain1:            b.Quasar(),
		Chain2:            b.Osmosis(),
		Relayer:           b.relayer,
		Path:              pathName,
		CreateChannelOpts: ibc.DefaultChannelOpts(),
		CreateClientOpts: ibc.CreateClientOptions{
			TrustingPeriod: "24h",
		},
	})

	b.paths[pathName] = path{
		chain1: b.Quasar(),
		chain2: b.Osmosis(),
	}
}

// AutomatedRelay notifies the builder to spawn a relayer to automatically relayer packets.
func (b *E2ETestSuiteBuilder) AutomatedRelay() {
	b.checkBuilt()

	b.automatedRelay = true
}

// Build starts all chains and configures the relayer, returns the E2ETestSuite and seals the builder
// so it can not be used or changed after this.
func (b *E2ETestSuiteBuilder) Build() *E2ETestSuite {
	b.checkBuilt()

	ctx := context.Background()

	require.NoError(b.t, b.ic.Build(ctx, b.erep, ibctest.InterchainBuildOptions{
		TestName:         b.t.Name(),
		Client:           b.dockerClient,
		NetworkID:        b.networkID,
		SkipPathCreation: false,
	}))
	b.t.Cleanup(func() {
		if err := b.ic.Close(); err != nil {
			b.t.Logf("could not close interchain properly: %s", err)
		}
	})

	if b.automatedRelay {
		pathNames := b.pathNames()
		require.NoError(b.t, b.relayer.StartRelayer(ctx, b.erep, pathNames...))
		b.t.Cleanup(func() {
			if err := b.relayer.StopRelayer(ctx, b.erep); err != nil {
				b.t.Logf("an error occurred while stopping the relayer: %s", err)
			}
		})
	}

	b.built = true
	return &E2ETestSuite{
		quasar:       b.quasar,
		cosmos:       b.cosmos,
		osmosis:      b.osmosis,
		grpcClients:  b.prepareGRPCClients(),
		relayer:      b.relayer,
		paths:        b.paths,
		rep:          b.rep,
		erep:         b.erep,
		dockerClient: b.dockerClient,
		networkID:    b.networkID,
		logger:       b.logger,
	}
}

func (b *E2ETestSuiteBuilder) checkBuilt() {
	if b.built {
		b.t.Fatal("e2e test suite is already built")
	}
}

func (b *E2ETestSuiteBuilder) pathNames() []string {
	var pathNames []string
	for k := range b.paths {
		pathNames = append(pathNames, k)
	}
	return pathNames
}

func (b *E2ETestSuiteBuilder) prepareGRPCClients() map[ibc.Chain]*grpc.ClientConn {
	clients := map[ibc.Chain]*grpc.ClientConn{
		b.quasar: b.dialChainGRPC(b.quasar),
	}

	if b.cosmos != nil {
		clients[b.cosmos] = b.dialChainGRPC(b.cosmos)
	}
	if b.osmosis != nil {
		clients[b.osmosis] = b.dialChainGRPC(b.osmosis)
	}
	return clients
}

func (b *E2ETestSuiteBuilder) dialChainGRPC(chain *cosmos.CosmosChain) *grpc.ClientConn {
	grpcConn, err := grpc.Dial(
		chain.GetHostGRPCAddress(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	require.NoError(b.t, err)
	b.t.Cleanup(func() {
		if err := grpcConn.Close(); err != nil {
			b.t.Logf("failed closing GRPC connection to chain %s: %s", chain.Config().ChainID, err)
		}
	})

	return grpcConn
}
