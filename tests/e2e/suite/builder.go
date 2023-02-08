package suite

import (
	"context"
	"testing"

	dockerclient "github.com/docker/docker/client"
	testconfig "github.com/quasarlabs/quasarnode/tests/e2e/config"
	ibctest "github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"github.com/strangelove-ventures/interchaintest/v6/relayer/rly"
	"github.com/strangelove-ventures/interchaintest/v6/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// E2ETestSuiteBuilder is a factory to simplify works behind running chains, configuring relayer and logging.
type E2ETestSuiteBuilder struct {
	t *testing.T

	quasar  *cosmos.CosmosChain
	cosmos  *cosmos.CosmosChain
	osmosis *cosmos.CosmosChain

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
	logger := zaptest.NewLogger(t)

	quasar := cosmos.NewCosmosChain(
		t.Name(),
		testconfig.QuasarChain,
		testconfig.DefaultNumValidators,
		testconfig.DefaultNumNodes,
		logger)

	dockerClient, networkID := ibctest.DockerSetup(t)
	relayer := rly.NewCosmosRelayer(logger, t.Name(), dockerClient, networkID)
	rep := testreporter.NewNopReporter()
	erep := rep.RelayerExecReporter(t)

	return &E2ETestSuiteBuilder{
		t:            t,
		quasar:       quasar,
		relayer:      relayer,
		paths:        map[string]path{},
		rep:          rep,
		erep:         erep,
		dockerClient: dockerClient,
		networkID:    networkID,
		ic:           ibctest.NewInterchain().AddChain(quasar).AddRelayer(relayer, "relayer").WithLog(logger),
		logger:       logger,
	}
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

	b.osmosis = cosmos.NewCosmosChain(
		b.t.Name(),
		testconfig.OsmosisChain,
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
func (b *E2ETestSuiteBuilder) Link(chain1, chain2 ibc.Chain, pathName string) {
	b.checkBuilt()

	b.ic.AddLink(ibctest.InterchainLink{
		Chain1:            chain1,
		Chain2:            chain2,
		Relayer:           b.relayer,
		Path:              pathName,
		CreateChannelOpts: ibc.DefaultChannelOpts(),
	})

	b.paths[pathName] = path{
		chain1: chain1,
		chain2: chain2,
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
