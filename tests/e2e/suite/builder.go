package suite

import (
	"context"
	"testing"

	dockerclient "github.com/docker/docker/client"
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

// E2ETestSuiteBuilder is a factory to simplify works behind running chains, configuring Relayer and logging.
type E2ETestSuiteBuilder struct {
	t *testing.T

	quasar         *cosmos.CosmosChain
	QuasarAccounts Accounts

	cosmos         *cosmos.CosmosChain
	CosmosAccounts Accounts

	osmosis         *cosmos.CosmosChain
	OsmosisAccounts Accounts

	Relayer        *rly.CosmosRelayer
	paths          map[string]path
	Rep            *testreporter.Reporter
	Erep           *testreporter.RelayerExecReporter
	automatedRelay bool

	dockerClient *dockerclient.Client
	networkID    string

	ic     *ibctest.Interchain
	Logger *zap.Logger
	built  bool
}

// NewE2ETestSuiteBuilder returns a new E2ETestSuiteBuilder.
func NewE2ETestSuiteBuilder(t *testing.T) *E2ETestSuiteBuilder {
	E2EBuilder := &E2ETestSuiteBuilder{}
	logger := zaptest.NewLogger(t)

	ctx := context.Background()

	chainConfig := QuasarChain
	chainConfig.PreGenesis = func(cc ibc.ChainConfig) (err error) {
		val := E2EBuilder.quasar.Validators[0]
		E2EBuilder.QuasarAccounts, err = quasarPreGenesis(ctx, val)
		return err
	}

	E2EBuilder.quasar = cosmos.NewCosmosChain(
		t.Name(),
		chainConfig,
		DefaultNumValidators,
		DefaultNumNodes,
		logger)

	dockerClient, networkID := ibctest.DockerSetup(t)
	relayer := rly.NewCosmosRelayer(logger, t.Name(), dockerClient, networkID)
	rep := testreporter.NewNopReporter()
	erep := rep.RelayerExecReporter(t)

	E2EBuilder.t = t
	E2EBuilder.Relayer = relayer
	E2EBuilder.paths = map[string]path{}
	E2EBuilder.Rep = rep
	E2EBuilder.Erep = erep
	E2EBuilder.dockerClient = dockerClient
	E2EBuilder.networkID = networkID
	E2EBuilder.ic = ibctest.NewInterchain().AddChain(E2EBuilder.quasar).AddRelayer(relayer, "Relayer").WithLog(logger)
	E2EBuilder.Logger = logger

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
		CosmosChain,
		DefaultNumValidators,
		DefaultNumNodes,
		b.Logger)

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

	osmosisConfig := OsmosisChain
	osmosisConfig.PreGenesis = func(cc ibc.ChainConfig) (err error) {
		val := b.osmosis.Validators[0]
		b.OsmosisAccounts, err = osmosisPreGenesis(ctx, val)
		return err
	}

	b.osmosis = cosmos.NewCosmosChain(
		b.t.Name(),
		osmosisConfig,
		DefaultNumValidators,
		DefaultNumNodes,
		b.Logger,
	)

	b.ic.AddChain(b.osmosis)
}

// Osmosis returns the cosmos chain instance. It will fail the test if used before calling UseOsmosis.
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
		Relayer:           b.Relayer,
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

// AutomatedRelay notifies the builder to spawn a Relayer to automatically Relayer packets.
func (b *E2ETestSuiteBuilder) AutomatedRelay() {
	b.checkBuilt()

	b.automatedRelay = true
}

// Build starts all chains and configures the Relayer, returns the E2ETestSuite and seals the builder
// so it can not be used or changed after this.
func (b *E2ETestSuiteBuilder) Build() *E2ETestSuite {
	b.checkBuilt()

	ctx := context.Background()

	require.NoError(b.t, b.ic.Build(ctx, b.Erep, ibctest.InterchainBuildOptions{
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
		require.NoError(b.t, b.Relayer.StartRelayer(ctx, b.Erep, pathNames...))
		b.t.Cleanup(func() {
			if err := b.Relayer.StopRelayer(ctx, b.Erep); err != nil {
				b.t.Logf("an error occurred while stopping the Relayer: %s", err)
			}
		})
	}

	b.built = true
	return &E2ETestSuite{
		quasar:       b.quasar,
		cosmos:       b.cosmos,
		osmosis:      b.osmosis,
		grpcClients:  b.prepareGRPCClients(),
		relayer:      b.Relayer,
		paths:        b.paths,
		rep:          b.Rep,
		erep:         b.Erep,
		dockerClient: b.dockerClient,
		networkID:    b.networkID,
		logger:       b.Logger,
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
