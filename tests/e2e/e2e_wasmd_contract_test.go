package e2e

import (
	"context"
	"fmt"
	"os"
	"testing"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	transfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testconfig "github.com/quasarlabs/quasarnode/tests/e2e/config"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	StartingTokenAmount            int64  = 100_000_000
	IBCTransferAmount              int64  = 10_000
	ProposalTitle                  string = "title"
	ProposalDescription            string = "description"
	lpStrategyContractPath                = "../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath        = "../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	osmosisPool1Path                      = "scripts/sample_pool1.json"
	osmosisPool2Path                      = "scripts/sample_pool2.json"
	osmosisPool3Path                      = "scripts/sample_pool3.json"
)

func TestWasmdTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testconfig.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &WasmdTestSuite{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type WasmdTestSuite struct {
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	*testsuite.E2ETestSuite

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection
	Osmosis2QuasarConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel
	Osmosis2QuasarTransferChan *channeltypes.IdentifiedChannel

	OsmosisDenomInQuasar string
	QuasarDenomInOsmosis string

	LpStrategyContractAddress1 string
	LpStrategyContractAddress2 string
	LpStrategyContractAddress3 string

	BasicVaultContractAddress string
}

func (s *WasmdTestSuite) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	// Send tokens to the respective account and create the required pools
	s.SendAndCreatePools(ctx)

	// Wait for IBC connections to be established
	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testconfig.Quasar2OsmosisPath)[0]
	s.Osmosis2QuasarConn = s.GetConnectionsByPath(ctx, testconfig.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]
	s.Osmosis2QuasarTransferChan = s.QueryConnectionChannels(ctx, s.Osmosis(), s.Osmosis2QuasarConn.Id)[0]

	// Send tokens "stake1", "uosmo", "fakestake" from Osmosis to Quasar account
	s.SendTokensFromOsmosisToQuasar(ctx)

	// Generate the ibc denom of native tokens in other chains
	s.OsmosisDenomInQuasar = ibcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.QuasarDenomInOsmosis = ibcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// // Setup an account in quasar chain for contract deployment
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Deploy the lp strategy contract
	s.deployContracts(ctx, quasarAccount, lpStrategyContractPath, "lp_strategy_test",
		map[string]any{
			"lock_period":           6,
			"pool_id":               1,
			"pool_denom":            "gamm/pool/1",
			"base_denom":            "uosmo",
			"local_denom":           "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			"quote_denom":           "stake1",
			"return_source_channel": "channel-0",
			"transfer_channel":      "channel-0",
		},
		map[string]any{
			"lock_period":           6,
			"pool_id":               2,
			"pool_denom":            "gamm/pool/2",
			"base_denom":            "stake1",
			"local_denom":           "ibc/BC42BB1B7065ADF71AB8F5ECE6CDE06EF93674C343C22AEAA8AE51B7EF364F0B",
			"quote_denom":           "fakestake",
			"return_source_channel": "channel-0",
			"transfer_channel":      "channel-0",
		},
		map[string]any{
			"lock_period":           6,
			"pool_id":               3,
			"pool_denom":            "gamm/pool/3",
			"base_denom":            "fakestake",
			"local_denom":           "ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1",
			"quote_denom":           "uosmo",
			"return_source_channel": "channel-0",
			"transfer_channel":      "channel-0",
		},
	)
}

// deployAndInitContract stores the contract, initiates it and returns the contract address.
func (s *WasmdTestSuite) deployContracts(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs1, initArgs2, initArgs3 any) {
	accAddress := acc.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)

	// instantiate the contracts
	res := s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs1)
	s.Require().NotEmpty(res.Address)
	s.LpStrategyContractAddress1 = res.Address

	res = s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs1)
	s.Require().NotEmpty(res.Address)
	s.LpStrategyContractAddress2 = res.Address

	res = s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs1)
	s.Require().NotEmpty(res.Address)
	s.LpStrategyContractAddress3 = res.Address

	// create channels for all the instantiated contracts address 1
	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress1),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress1),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)

	// create channels for all the instantiated contracts address 2
	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress2),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress2),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)

	// create channels for all the instantiated contracts address 3
	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress3),
		"icqhost",
		ibc.Unordered,
		"icq-1",
	)

	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress3),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)
}

// deployAndInitContract stores the contract, initiates it and returns the contract address.
func (s *WasmdTestSuite) deployContract(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs any) string {
	accAddress := acc.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)

	res := s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs)
	s.Require().NotEmpty(res.Address)

	return res.Address
}

// TestLpStrategyContract_SuccessfulDeposit tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *WasmdTestSuite) TestLpStrategyContract_SuccessfulDeposit() {
	t := s.T()
	ctx := context.Background()

	// Set up an account in quasar chain
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// deploy basic_vault contract
	s.BasicVaultContractAddress = s.deployContract(ctx, quasarAccount, basicVaultStrategyContractPath, "basic_vault",
		map[string]any{
			"decimals":       6,
			"symbol":         "ORN",
			"min_withdrawal": "1",
			"name":           "ORION",
			"primitives": []map[string]any{
				{
					"address": s.LpStrategyContractAddress1,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": map[string]any{
							"lock_period":           6,
							"pool_id":               1,
							"pool_denom":            "gamm/pool/1",
							"base_denom":            "uosmo",
							"local_denom":           "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
							"quote_denom":           "stake1",
							"return_source_channel": "channel-0",
							"transfer_channel":      "channel-0",
						},
					},
				},
				{
					"address": s.LpStrategyContractAddress2,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": map[string]any{
							"lock_period":           6,
							"pool_id":               2,
							"pool_denom":            "gamm/pool/2",
							"base_denom":            "stake1",
							"local_denom":           "ibc/BC42BB1B7065ADF71AB8F5ECE6CDE06EF93674C343C22AEAA8AE51B7EF364F0B",
							"quote_denom":           "fakestake",
							"return_source_channel": "channel-0",
							"transfer_channel":      "channel-0",
						},
					},
				},
				{
					"address": s.LpStrategyContractAddress3,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": map[string]any{
							"lock_period":           6,
							"pool_id":               3,
							"pool_denom":            "gamm/pool/3",
							"base_denom":            "fakestake",
							"local_denom":           "ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1",
							"quote_denom":           "uosmo",
							"return_source_channel": "channel-0",
							"transfer_channel":      "channel-0",
						},
					},
				},
			},
		})

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.E2EBuilder.QuasarAccounts.Authority.KeyName,
		s.BasicVaultContractAddress,
		sdk.NewCoins(
			sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 1000),
			sdk.NewInt64Coin("ibc/BC42BB1B7065ADF71AB8F5ECE6CDE06EF93674C343C22AEAA8AE51B7EF364F0B", 1000),
			sdk.NewInt64Coin("ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1", 1000),
		),
		"'{\"bond\":{}}'",
		nil)

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	time.Sleep(100000000)

	// ICA address should now have exactly 1000uqsr
	//balance, err := s.Osmosis().GetBalance(ctx, icaAddress, s.QuasarDenomInOsmosis)
	//s.Require().NoError(err)
	//s.Require().EqualValues(1000, balance)
}

func (s *WasmdTestSuite) SendAndCreatePools(ctx context.Context) {
	// Send uqsr and uayy to Quasar authority account
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Owner, s.E2EBuilder.QuasarAccounts.Authority, "10000000000000000uayy")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.MasterMinter, s.E2EBuilder.QuasarAccounts.Authority, "10000000000000000uqsr")

	// Send stake1 and uosmo to Osmosis authority account
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Owner, s.E2EBuilder.OsmosisAccounts.Authority, "10000000000000000stake1")
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.MasterMinter, s.E2EBuilder.OsmosisAccounts.Authority, "10000000000000000uosmo")

	// Read the pool details from os file
	poolBz, err := os.ReadFile(osmosisPool1Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Authority.KeyName, poolBz)

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool2Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Authority.KeyName, poolBz)

	// Read the contract from os file
	poolBz, err = os.ReadFile(osmosisPool3Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Authority.KeyName, poolBz)
}

func (s *WasmdTestSuite) SendTokensFromOsmosisToQuasar(ctx context.Context) {
	walletAmount := ibc.WalletAmount{
		Address: s.E2EBuilder.QuasarAccounts.Authority.Address,
		Denom:   "stake1",
		Amount:  100000,
	}
	transfer, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	walletAmount.Denom = "uosmo"
	transfer, err = s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	walletAmount.Denom = "fakestake"
	transfer, err = s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())
}

type channelsResponse struct {
	Channels []struct {
		Id                   string `json:"id"`
		ConnectionId         string `json:"connection_id"`
		CounterpartyEndpoint struct {
			ChannelId string `json:"channel_id"`
			PortId    string `json:"port_id"`
		} `json:"counterparty_endpoint"`
		ChannelType struct {
			ICA struct {
				CounterpartyAddress string `json:"counter_party_address"`
			} `json:"ica"`
		} `json:"channel_type"`
	} `json:"channels"`
}

type Pool struct {
	Weights        string `json:"weights"`
	InitialDeposit string `json:"initial-deposit"`
	SwapFee        string `json:"swap-fee"`
	ExitFee        string `json:"exit-fee"`
	FutureGovernor string `json:"future-governor"`
}

// ibcDenomFromChannel returns ibc denom according to the given channel port, id and denom
// this function generates the ibc denom for the main direction as an example if there is a channel from
// chain1 <-> chain2 knowing that chain1 has a denom named denom1 this function will return the ibc denom of denom1 in chain2.
func ibcDenomFromChannel(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.PortId, ch.ChannelId, baseDenom)).IBCDenom()
}

// ibcDenomFromChannelCounterparty does same as ibcDenomFromChannel but in reverse so it generates
// the ibc denom of denom2 from chain2 (counterparty chain) in chain1
func ibcDenomFromChannelCounterparty(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.Counterparty.PortId, ch.Counterparty.ChannelId, baseDenom)).IBCDenom()
}
