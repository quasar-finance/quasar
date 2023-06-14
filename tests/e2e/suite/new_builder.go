package suite

import (
	"context"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
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
)

type E2eTestBuilder struct {
	//suite.Suite
	t *testing.T

	IC *ibctest.Interchain

	Logger *zap.Logger
	Built  bool

	Relayer *rly.CosmosRelayer
	Paths   map[string]path

	Rep            *testreporter.Reporter
	Erep           *testreporter.RelayerExecReporter
	automatedRelay bool

	dockerClient *dockerclient.Client
	networkID    string

	Chains Chains
}

func NewE2eTestBuilder(t *testing.T) *E2eTestBuilder {
	logger := zaptest.NewLogger(t)
	dockerClient, networkID := ibctest.DockerSetup(t)
	relayer := rly.NewCosmosRelayer(logger, t.Name(), dockerClient, networkID)
	rep := testreporter.NewNopReporter()
	erep := rep.RelayerExecReporter(t)

	return &E2eTestBuilder{
		t:              t,
		IC:             ibctest.NewInterchain().AddRelayer(relayer, "Relayer").WithLog(logger),
		Logger:         logger,
		Relayer:        relayer,
		Paths:          map[string]path{},
		Rep:            rep,
		Erep:           erep,
		automatedRelay: false,
		dockerClient:   dockerClient,
		networkID:      networkID,
		Chains:         Chains{},
	}
}

func (e *E2eTestBuilder) AddChain(chainConfig ibc.ChainConfig, genesisCoins sdk.Coins, numberOfValidators, numberOfNodes int, IsWasmEnabled bool) {
	e.checkBuilt()

	ctx := context.Background()

	// taking index that does not exist in e.Chains as this PreGenesis function will be executed later once
	// append action happens to the e.Chains slice
	chainConfig.PreGenesis = func(cc ibc.ChainConfig) (err error) {
		chain, found := e.Chains.GetChain(cc.Name)
		if !found {
			return fmt.Errorf("unbale to find chain %s", cc.Name)
		}
		val := chain.Chain.Validators[0]
		chain.ChainAccount, err = addPreGenesis(ctx, val, genesisCoins)
		e.setChain(cc.Name, chain)
		return err
	}

	if numberOfNodes == 0 {
		numberOfNodes = DefaultNumNodes
	}
	if numberOfValidators == 0 {
		numberOfValidators = DefaultNumValidators
	}

	chain := cosmos.NewCosmosChain(
		e.t.Name(),
		chainConfig,
		numberOfValidators,
		numberOfNodes,
		e.Logger)

	e.Chains = append(e.Chains, &Chain{
		Chain:         chain,
		IsWasmEnabled: IsWasmEnabled,
	})

	e.IC.AddChain(chain)
}

func (e *E2eTestBuilder) setChain(chainName string, chain *Chain) {
	for i, c := range e.Chains {
		if c.Chain.Config().Name == chainName {
			e.Chains[i] = chain
			return
		}
	}
}

func (e *E2eTestBuilder) GetChain(chainName string) (*Chain, bool) {
	for _, ch := range e.Chains {
		if ch.Chain.Config().Name == chainName {
			return ch, true
		}
	}
	return &Chain{}, false
}

func (e *E2eTestBuilder) AddRelayer(chain1, chain2 ibc.Chain, relayer ibc.Relayer, pathName string, createChannelOpts ibc.CreateChannelOptions, createClientOptions ibc.CreateClientOptions) {
	e.checkBuilt()

	e.IC.AddLink(ibctest.InterchainLink{
		Chain1:            chain1,
		Chain2:            chain2,
		Relayer:           relayer,
		Path:              pathName,
		CreateChannelOpts: createChannelOpts,
		CreateClientOpts:  createClientOptions,
	})

	e.Paths[pathName] = path{
		chain1: chain1,
		chain2: chain2,
	}
}

// AutomatedRelay notifies the builder to spawn a Relayer to automatically Relayer packets.
func (e *E2eTestBuilder) AutomatedRelay() {
	e.checkBuilt()
	e.automatedRelay = true
}

func (e *E2eTestBuilder) pathNames() []string {
	var pathNames []string
	for k := range e.Paths {
		pathNames = append(pathNames, k)
	}
	return pathNames
}

func (e *E2eTestBuilder) checkBuilt() {
	if e.Built {
		e.t.Fatal("e2e test suite is already built")
	}
}

func (e *E2eTestBuilder) Build() *E2eTestBuilder {
	e.checkBuilt()

	ctx := context.Background()

	// todo add more sanity checks if needed

	require.NoError(e.t, e.IC.Build(ctx, e.Erep, ibctest.InterchainBuildOptions{
		TestName:         e.t.Name(),
		Client:           e.dockerClient,
		NetworkID:        e.networkID,
		SkipPathCreation: false,
	}))
	e.t.Cleanup(func() {
		if err := e.IC.Close(); err != nil {
			e.t.Logf("could not close interchain properly: %s", err)
		}
	})

	if e.automatedRelay {
		pathNames := e.pathNames()
		require.NoError(e.t, e.Relayer.StartRelayer(ctx, e.Erep, pathNames...))
		e.t.Cleanup(func() {
			if err := e.Relayer.StopRelayer(ctx, e.Erep); err != nil {
				e.t.Logf("an error occurred while stopping the Relayer: %s", err)
			}
		})
	}

	e.Built = true
	return &E2eTestBuilder{
		t:              e.t,
		IC:             e.IC,
		Logger:         e.Logger,
		Built:          e.Built,
		Relayer:        e.Relayer,
		Paths:          e.Paths,
		Rep:            e.Rep,
		Erep:           e.Erep,
		automatedRelay: e.automatedRelay,
		dockerClient:   e.dockerClient,
		networkID:      e.networkID,
		Chains:         e.Chains,
	}
}
