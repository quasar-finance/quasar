package cmd

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
	tmconfig "github.com/tendermint/tendermint/config"
	"github.com/tendermint/tendermint/types"
	tmtime "github.com/tendermint/tendermint/types/time"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/cosmos/cosmos-sdk/crypto/hd"
	"github.com/cosmos/cosmos-sdk/crypto/keyring"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	"github.com/cosmos/cosmos-sdk/server"
	srvconfig "github.com/cosmos/cosmos-sdk/server/config"
	"github.com/cosmos/cosmos-sdk/testutil"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/cosmos/cosmos-sdk/x/genutil"
	genutiltypes "github.com/cosmos/cosmos-sdk/x/genutil/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	"github.com/google/uuid"
)

const (
	defaultDirPerm    = 0o755
	defaultFilePerm   = 0o644
	chainID           = "quasar-tesnet"
	nodeNamePrefix    = "node"
	nodeConfigDirName = "quasarnoded"
	portBaseP2p       = 26500
	portBaseRpc       = 26600
	portBaseApi       = 1300
	portBaseGrpc      = 9000
	portBaseWeb       = 9100
	flagNumNodes      = "nodes"
	flagOutputDir     = "output-dir"
	flagMinGasPrices  = "minimum-gas-prices"
)

// TestnetCmd initializes all files for tendermint testnet and application.
func TestnetCmd(mbm module.BasicManager, genBalIterator banktypes.GenesisBalancesIterator) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "testnet",
		Short: "Initialize files for a testnet",
		Long: `testnet will create "n" number of directories and populate each with
necessary files (private validator, genesis, config, etc.).

Note, strict routability for addresses is turned off in the config file.

Example:
	quasarnoded testnet -n 4 -o ./output
	`,
		RunE: func(cmd *cobra.Command, _ []string) error {
			clientCtx, err := client.GetClientQueryContext(cmd)
			if err != nil {
				return err
			}

			outputDir, _ := cmd.Flags().GetString(flagOutputDir)
			numNodes, _ := cmd.Flags().GetInt(flagNumNodes)
			minGasPrices, _ := cmd.Flags().GetString(flagMinGasPrices)

			gen := NewNodeConfigGenerator(
				outputDir,
				clientCtx,
				mbm,
				genBalIterator,
				minGasPrices,
			)

			for i := 0; i < numNodes; i++ {
				gen.AddNode()
			}

			gen.Persist()

			cmd.PrintErrf("Successfully initialized %d node directories\n", numNodes)

			return nil
		},
	}

	cmd.Flags().IntP(flagNumNodes, "n", 4, "Number of validator nodes to initialize the testnet with")
	cmd.Flags().StringP(flagOutputDir, "o", "./localnet", "Directory to store initialization data for the testnet")
	cmd.Flags().String(server.FlagMinGasPrices, fmt.Sprintf("0.000006%s", TestnetGenesisParams().NativeCoinMetadatas[0].Base), "Minimum gas prices to accept for transactions; All fees in a tx must meet this minimum (e.g. 0.01photino,0.001stake)")

	return cmd
}

type NodeConfigGenerator struct {
	nodeCount int

	outputDir string
	tmpDir    string
	gentxsDir string

	clientCtx      client.Context
	mbm            module.BasicManager
	genBalIterator banktypes.GenesisBalancesIterator
	genesisParams  GenesisParams
	minGasPrices   string

	nodeConfigs      []*tmconfig.Config
	nodeIDs          []string
	validatorPubKeys []cryptotypes.PubKey
	genesisAccounts  []authtypes.GenesisAccount
	genesisBalances  []banktypes.Balance
	genesisFiles     []string
}

func NewNodeConfigGenerator(
	outputDir string,
	clientTx client.Context,
	mbm module.BasicManager,
	genBalIterator banktypes.GenesisBalancesIterator,
	minGasPrices string,
) *NodeConfigGenerator {
	outputParentDir := filepath.Dir(outputDir)
	tmpDirName := fmt.Sprintf(".%s.tmp", uuid.New())
	tmpDir := filepath.Join(outputParentDir, tmpDirName)
	mustMkdirAll(tmpDir)

	gentxsDir := filepath.Join(tmpDir, "gentxs")
	mustMkdirAll(gentxsDir)

	return &NodeConfigGenerator{
		nodeCount: 0,

		outputDir: outputDir,
		tmpDir:    tmpDir,
		gentxsDir: gentxsDir,

		clientCtx:      clientTx,
		mbm:            mbm,
		genBalIterator: genBalIterator,
		minGasPrices:   minGasPrices,

		genesisParams: TestnetGenesisParams(),
	}
}

func (g *NodeConfigGenerator) newSimappConfig(minGasPrices string) *srvconfig.Config {
	conf := srvconfig.DefaultConfig()
	conf.MinGasPrices = minGasPrices
	conf.API.Enable = true
	conf.Telemetry.Enabled = true
	conf.Telemetry.PrometheusRetentionTime = 60
	conf.Telemetry.EnableHostnameLabel = false
	conf.Telemetry.GlobalLabels = [][]string{{"chain_id", chainID}}
	conf.API.Address = fmt.Sprintf("tcp://localhost:%d", portBaseApi+g.nodeCount)
	conf.GRPC.Address = fmt.Sprintf("tcp://localhost:%d", portBaseGrpc+g.nodeCount)
	conf.GRPCWeb.Address = fmt.Sprintf("tcp://localhost:%d", portBaseWeb+g.nodeCount)

	return conf
}

func (g *NodeConfigGenerator) AddNode() error {
	var err error

	nodeName := fmt.Sprintf("%s%d", nodeNamePrefix, g.nodeCount)
	nodeDir := filepath.Join(g.tmpDir, nodeName, nodeConfigDirName)
	mustMkdirAll(filepath.Join(nodeDir, "config"))

	nodeConfig := tmconfig.DefaultConfig()
	g.nodeConfigs = append(g.nodeConfigs, nodeConfig)

	nodeConfig.SetRoot(nodeDir)
	nodeConfig.Moniker = nodeName
	nodeConfig.RPC.ListenAddress = fmt.Sprintf("tcp://localhost:%d", portBaseRpc+g.nodeCount)
	nodeConfig.P2P.ListenAddress = fmt.Sprintf("tcp://localhost:%d", portBaseP2p+g.nodeCount)

	nodeID, validatorPubKey, err := genutil.InitializeNodeValidatorFiles(nodeConfig)
	if err != nil {
		return err
	}
	g.nodeIDs = append(g.nodeIDs, nodeID)
	g.validatorPubKeys = append(g.validatorPubKeys, validatorPubKey)
	g.genesisFiles = append(g.genesisFiles, nodeConfig.GenesisFile())

	addr, err := genTestKeyring(nodeDir, nodeName)
	if err != nil {
		return err
	}

	accTokens := sdk.TokensFromConsensusPower(1_000_000_000, sdk.NewInt(1))
	accStakingTokens := sdk.TokensFromConsensusPower(500_000_000, sdk.NewInt(1))
	coins := sdk.Coins{
		sdk.NewCoin(fmt.Sprintf("%stoken", nodeName), accTokens),
		sdk.NewCoin(g.genesisParams.NativeCoinMetadatas[0].Base, accStakingTokens),
	}

	g.genesisBalances = append(g.genesisBalances, banktypes.Balance{Address: addr.String(), Coins: coins.Sort()})
	g.genesisAccounts = append(g.genesisAccounts, authtypes.NewBaseAccount(addr, nil, 0, 0))

	valTokens := sdk.TokensFromConsensusPower(100_000_000, sdk.NewInt(1))
	createValMsg, err := stakingtypes.NewMsgCreateValidator(
		sdk.ValAddress(addr),
		g.validatorPubKeys[g.nodeCount],
		sdk.NewCoin(g.genesisParams.NativeCoinMetadatas[0].Base, valTokens),
		stakingtypes.NewDescription(nodeName, "", "", "", ""),
		stakingtypes.NewCommissionRates(
			sdk.MustNewDecFromStr("0.1"),
			sdk.MustNewDecFromStr("0.2"),
			sdk.MustNewDecFromStr("0.01"),
		),
		sdk.OneInt(),
	)
	if err != nil {
		return err
	}

	txBuilder := g.clientCtx.TxConfig.NewTxBuilder()
	if err := txBuilder.SetMsgs(createValMsg); err != nil {
		return err
	}

	nodeMemo := fmt.Sprintf("%s@localhost:%d", g.nodeIDs[g.nodeCount], portBaseP2p+g.nodeCount)
	txBuilder.SetMemo(nodeMemo)

	kr, err := loadTestKeyring(nodeDir)
	if err != nil {
		return err
	}

	txFactory := tx.Factory{}
	txFactory = txFactory.
		WithChainID(chainID).
		WithMemo(nodeMemo).
		WithKeybase(kr).
		WithTxConfig(g.clientCtx.TxConfig)

	if err := tx.Sign(txFactory, nodeName, txBuilder, true); err != nil {
		return err
	}

	txBz, err := g.clientCtx.TxConfig.TxJSONEncoder()(txBuilder.GetTx())
	if err != nil {
		return err
	}

	mustWriteFile(filepath.Join(g.gentxsDir, fmt.Sprintf("%s.json", nodeName)), txBz)
	srvconfig.WriteConfigFile(filepath.Join(nodeDir, "config/app.toml"), g.newSimappConfig(g.minGasPrices))

	g.nodeCount++

	return nil
}

func (g *NodeConfigGenerator) Persist() error {
	var err error

	// Make sure tmp dir is deleted in any case
	defer g.cleanup()

	err = g.initGenesisFiles()
	if err != nil {
		return err
	}

	err = g.updateNodeConfigs()
	if err != nil {
		return err
	}

	mustRename(g.tmpDir, g.outputDir)

	return nil
}

func (g *NodeConfigGenerator) initGenesisFiles() error {
	appGenState := g.mbm.DefaultGenesis(g.clientCtx.Codec)

	// set the accounts in the genesis state
	var authGenState authtypes.GenesisState
	g.clientCtx.Codec.MustUnmarshalJSON(appGenState[authtypes.ModuleName], &authGenState)

	accounts, err := authtypes.PackAccounts(g.genesisAccounts)
	if err != nil {
		return err
	}

	authGenState.Accounts = accounts
	appGenState[authtypes.ModuleName] = g.clientCtx.Codec.MustMarshalJSON(&authGenState)

	// set the balances in the genesis state
	var bankGenState banktypes.GenesisState
	g.clientCtx.Codec.MustUnmarshalJSON(appGenState[banktypes.ModuleName], &bankGenState)

	bankGenState.Balances = g.genesisBalances
	appGenState[banktypes.ModuleName] = g.clientCtx.Codec.MustMarshalJSON(&bankGenState)

	appGenState, _, err = PrepareGenesis(g.clientCtx, appGenState, &types.GenesisDoc{}, g.genesisParams, chainID)
	if err != nil {
		return err
	}

	appGenStateJSON, err := json.MarshalIndent(appGenState, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal application genesis state: %w", err)
	}

	genDoc := types.GenesisDoc{
		ChainID:    chainID,
		AppState:   appGenStateJSON,
		Validators: nil,
	}

	// generate empty genesis files for each validator and save
	for _, genFile := range g.genesisFiles {
		err := genDoc.SaveAs(genFile)
		if err != nil {
			return err
		}
	}

	return nil
}

func (g *NodeConfigGenerator) updateNodeConfigs() error {
	genesisTime := tmtime.Now()

	for i := 0; i < g.nodeCount; i++ {
		initCfg := genutiltypes.NewInitConfig(chainID, g.gentxsDir, g.nodeIDs[i], g.validatorPubKeys[i])

		genDoc, err := types.GenesisDocFromFile(g.genesisFiles[i])
		if err != nil {
			return err
		}

		nodeAppState, err := genutil.GenAppStateFromConfig(g.clientCtx.Codec, g.clientCtx.TxConfig, g.nodeConfigs[i], initCfg, *genDoc, g.genBalIterator)
		if err != nil {
			return err
		}

		// overwrite each validator's genesis file to have a canonical genesis time
		err = genutil.ExportGenesisFileWithTime(g.genesisFiles[i], chainID, nil, nodeAppState, genesisTime)
		if err != nil {
			return err
		}
	}

	return nil
}

func (g *NodeConfigGenerator) cleanup() {
	mustRemoveAll(g.tmpDir)
}

func genTestKeyring(outputDir string, keyname string) (sdk.AccAddress, error) {
	kb, err := keyring.New("quasar", "test", outputDir, nil)
	if err != nil {
		return nil, err
	}

	keyringAlgos, _ := kb.SupportedAlgorithms()
	algo, err := keyring.NewSigningAlgoFromString(string(hd.Secp256k1Type), keyringAlgos)
	if err != nil {
		return nil, err
	}

	addr, secret, err := testutil.GenerateSaveCoinKey(kb, keyname, "", true, algo)
	if err != nil {
		return nil, err
	}

	// Save seed to file
	seedJson, err := json.Marshal(map[string]string{"secret": secret})
	if err != nil {
		return nil, err
	}
	mustWriteFile(filepath.Join(outputDir, "key_seed.json"), seedJson)

	return addr, nil
}

func loadTestKeyring(dir string) (keyring.Keyring, error) {
	kb, err := keyring.New("quasar", "test", dir, nil)
	if err != nil {
		return nil, err
	}

	return kb, nil
}

func mustMkdirAll(dir string) {
	err := os.MkdirAll(dir, defaultDirPerm)
	if err != nil {
		panic(err)
	}
}

func mustRemoveAll(path string) {
	err := os.RemoveAll(path)
	if err != nil {
		panic(err)
	}
}

func mustRename(oldpath string, newpath string) {
	err := os.Rename(oldpath, newpath)
	if err != nil {
		panic(err)
	}
}

func mustWriteFile(path string, contents []byte) {
	var err error

	parentDir := filepath.Dir(path)
	mustMkdirAll(parentDir)

	tmpFile := filepath.Join(parentDir, fmt.Sprintf(".%s.tmp", uuid.New()))
	err = ioutil.WriteFile(tmpFile, contents, defaultFilePerm)
	if err != nil {
		panic(err)
	}

	mustRename(tmpFile, path)
}
