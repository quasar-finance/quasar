package e2e

import (
	"context"
	"cosmossdk.io/math"
	"fmt"
	"testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/docker/docker/client"
	"github.com/strangelove-ventures/interchaintest/v8"
	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
	"github.com/strangelove-ventures/interchaintest/v8/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap/zaptest"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module/testutil"
	ibclocalhost "github.com/cosmos/ibc-go/v8/modules/light-clients/09-localhost"
)

var (
	VotingPeriod     = "15s"
	MaxDepositPeriod = "10s"
	Denom            = "stake"

	quasarE2ERepo  = "quasar:local" // using quasar so it picks up from local build
	quasarMainRepo = "quasar:local" // using quasar so it picks up from local build

	IBCRelayerImage   = "ghcr.io/cosmos/relayer"
	IBCRelayerVersion = "main"

	quasarRepo, quasarVersion = GetDockerImageInfo()

	quasarImage = ibc.DockerImage{
		Repository: quasarRepo,
		Version:    quasarVersion,
		UidGid:     "1025:1025",
	}

	// SDK v47 Genesis
	defaultGenesisKV = []cosmos.GenesisKV{
		{
			Key:   "app_state.gov.params.voting_period",
			Value: VotingPeriod,
		},
		{
			Key:   "app_state.gov.params.max_deposit_period",
			Value: MaxDepositPeriod,
		},
		{
			Key:   "app_state.gov.params.min_deposit.0.denom",
			Value: Denom,
		},
		//{
		//	Key:   "app_state.feepay.params.enable_feepay",
		//	Value: false,
		//},
	}

	quasarConfig = ibc.ChainConfig{
		Type:                "cosmos",
		Name:                "quasar",
		ChainID:             "quasar-2",
		Images:              []ibc.DockerImage{quasarImage},
		Bin:                 "quasard",
		Bech32Prefix:        "quasar",
		Denom:               Denom,
		CoinType:            "118",
		GasPrices:           fmt.Sprintf("1.0%s", Denom),
		GasAdjustment:       2.0,
		TrustingPeriod:      "112h",
		NoHostMount:         false,
		ConfigFileOverrides: nil,
		ModifyGenesis:       cosmos.ModifyGenesis(defaultGenesisKV),
	}

	quasarConfig1 = ibc.ChainConfig{
		Type:                "cosmos",
		Name:                "quasar",
		ChainID:             "quasar-3",
		Images:              []ibc.DockerImage{quasarImage},
		Bin:                 "quasard",
		Bech32Prefix:        "quasar",
		Denom:               Denom,
		CoinType:            "118",
		GasPrices:           fmt.Sprintf("1.0%s", Denom),
		GasAdjustment:       2.0,
		TrustingPeriod:      "112h",
		NoHostMount:         false,
		ConfigFileOverrides: nil,
		ModifyGenesis:       cosmos.ModifyGenesis(defaultGenesisKV),
	}

	genesisWalletAmount = math.NewInt(100_000_000_000)
)

func init() {
	sdk.GetConfig().SetBech32PrefixForAccount("quasar", "quasar")
	sdk.GetConfig().SetBech32PrefixForValidator("quasarvaloper", "quasar")
	sdk.GetConfig().SetBech32PrefixForConsensusNode("quasarvalcons", "quasar")
	sdk.GetConfig().SetCoinType(118)
}

// quasarEncoding registers the quasar specific module codecs so that the associated types and msgs
// will be supported when writing to the blocksdb sqlite database.
func quasarEncoding() *testutil.TestEncodingConfig {
	cfg := cosmos.DefaultEncoding()

	// register custom types
	ibclocalhost.RegisterInterfaces(cfg.InterfaceRegistry)
	wasmtypes.RegisterInterfaces(cfg.InterfaceRegistry)

	return &cfg
}

// CreateChain generates a new chain with a custom image (useful for upgrades)
func CreateChain(t *testing.T, numVals, numFull int, img ibc.DockerImage) []ibc.Chain {
	cfg := quasarConfig
	cfg.Images = []ibc.DockerImage{img}
	return CreateChainWithCustomConfig(t, numVals, numFull, cfg)
}

// CreateThisBranchChain generates this branch's chain (ex: from the commit)
func CreateThisBranchChain(t *testing.T, numVals, numFull int) []ibc.Chain {
	return CreateChain(t, numVals, numFull, quasarImage)
}

func CreateChainWithCustomConfig(t *testing.T, numVals, numFull int, config ibc.ChainConfig) []ibc.Chain {
	cf := interchaintest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*interchaintest.ChainSpec{
		{
			Name:          "quasar",
			ChainName:     "quasar",
			Version:       config.Images[0].Version,
			ChainConfig:   config,
			NumValidators: &numVals,
			NumFullNodes:  &numFull,
		},
	})

	// Get chains from the chain factory
	chains, err := cf.Chains(t.Name())
	require.NoError(t, err)

	// chain := chains[0].(*cosmos.CosmosChain)
	return chains
}

func BuildInitialChain(t *testing.T, chains []ibc.Chain) (*interchaintest.Interchain, context.Context, *client.Client, string) {
	// Create a new Interchain object which describes the chains, relayers, and IBC connections we want to use
	ic := interchaintest.NewInterchain()

	for _, chain := range chains {
		ic = ic.AddChain(chain)
	}

	rep := testreporter.NewNopReporter()
	eRep := rep.RelayerExecReporter(t)

	ctx := context.Background()
	dockerClient, network := interchaintest.DockerSetup(t)

	err := ic.Build(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:         t.Name(),
		Client:           dockerClient,
		NetworkID:        network,
		SkipPathCreation: true,
		// This can be used to write to the block database which will index all block data e.g. txs, msgs, events, etc.
		// BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
	})
	require.NoError(t, err)

	return ic, ctx, dockerClient, network
}
