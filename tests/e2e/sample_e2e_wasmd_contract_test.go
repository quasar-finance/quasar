package e2e

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

type E2EVaultTestingSuite struct {
	*testsuite.E2ETestSuite
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection
	Osmosis2QuasarConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel
	Osmosis2QuasarTransferChan *channeltypes.IdentifiedChannel

	ContractsDeploymentWallet *ibc.Wallet

	PrimitiveCodeID, RewardsStoreID, VaultStoreID uint64
}

func TestE2EVaultTestingSuite(t *testing.T) {
	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &E2EVaultTestingSuite{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}

	suite.Run(t, s)

	ctx := context.Background()

	// Wait for IBC connections to be established
	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]
	s.Osmosis2QuasarConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]
	s.Osmosis2QuasarTransferChan = s.QueryConnectionChannels(ctx, s.Osmosis(), s.Osmosis2QuasarConn.Id)[0]
}

func (s *E2EVaultTestingSuite) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	// Wait for IBC connections to be established
	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]
	s.Osmosis2QuasarConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]
	s.Osmosis2QuasarTransferChan = s.QueryConnectionChannels(ctx, s.Osmosis(), s.Osmosis2QuasarConn.Id)[0]
}

func (s *E2EVaultTestingSuite) TestDeployContracts() {
	ctx := context.Background()

	// Set up an account in quasar chain for contract deployment
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Send tokens "uayy" and "uqsr" from Quasar to Osmosis account
	s.SendTokensToRespectiveAccounts(ctx)

	// Send tokens to the respective account and create the required pools
	s.CreatePools(ctx)

	codeID, err := testsuite.StoreContractCode(ctx, s.Quasar(), lpStrategyContractPath, s.ContractsDeploymentWallet, s.E2EBuilder)
	s.Require().NoError(err)
	s.PrimitiveCodeID = codeID

	// read all the init messages provided in the input file
	contracts, err := testsuite.ReadInitMessagesFile(contractsPath)
	s.Require().NoError(err)
	fmt.Println(contracts)

	for _, c := range contracts {
		c.SetCodeID(s.PrimitiveCodeID)
		err = c.InstantiateContract(ctx, s.ContractsDeploymentWallet, s.Quasar(), sdk.NewCoins())
		s.Require().NoError(err)

		err = c.CreateICQChannel(ctx, s.E2EBuilder.Relayer, s.E2EBuilder.Erep)
		s.Require().NoError(err)

		err = c.CreateICAChannel(ctx, s.E2EBuilder.Relayer, s.E2EBuilder.Erep, s.Quasar2OsmosisConn.Id, s.Quasar2OsmosisConn.Counterparty.ConnectionId)
		s.Require().NoError(err)
	}

	codeID, err = testsuite.StoreContractCode(ctx, s.Quasar(), vaultRewardsContractPath, s.ContractsDeploymentWallet, s.E2EBuilder)
	s.Require().NoError(err)
	s.RewardsStoreID = codeID

	codeID, err = testsuite.StoreContractCode(ctx, s.Quasar(), basicVaultStrategyContractPath, s.ContractsDeploymentWallet, s.E2EBuilder)
	s.Require().NoError(err)
	s.VaultStoreID = codeID

	VaultContrct := testsuite.NewContract(map[string]any{
		"total_cap":                     "200000000000",
		"thesis":                        "e2e",
		"vault_rewards_code_id":         s.RewardsStoreID,
		"reward_token":                  map[string]any{"native": "uqsr"},
		"reward_distribution_schedules": []string{},
		"decimals":                      6,
		"symbol":                        "ORN",
		"min_withdrawal":                "1",
		"name":                          "ORION",
		"primitives": []map[string]any{
			{
				"address": contracts[0].GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init1,
				},
			},
			{
				"address": contracts[1].GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init2,
				},
			},
			{
				"address": contracts[2].GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init3,
				},
			},
		},
	}, "basic_vault", s.VaultStoreID)

	err = VaultContrct.InstantiateContract(ctx, s.ContractsDeploymentWallet, s.Quasar(), sdk.NewCoins())
	s.Require().NoError(err)

	_, err = VaultContrct.ExecuteContract(
		ctx,
		s.Quasar(),
		map[string]any{"bond": map[string]any{}},
		nil,
		sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 10000000)),
		&s.E2EBuilder.QuasarAccounts.BondTest,
	)
	s.Require().NoError(err)

	res, err := VaultContrct.QueryContract(ctx, s.Quasar(), map[string]any{
		"balance": map[string]any{
			"address": s.E2EBuilder.QuasarAccounts.BondTest.Address,
		},
	})

	var data testsuite.ContractBalanceData
	err = json.Unmarshal(res, &data)
	s.Require().NoError(err)

	balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
	s.Require().NoError(err)

	s.Require().True(balance > 0)

}

func (s *E2EVaultTestingSuite) CreatePools(ctx context.Context) {
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

func (s *E2EVaultTestingSuite) SendTokensToRespectiveAccounts(ctx context.Context) {
	// Send uqsr and uayy to Quasar authority account
	//s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Owner, s.E2EBuilder.QuasarAccounts.Authority, "10000000000000000uayy")
	//s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.MasterMinter, s.E2EBuilder.QuasarAccounts.Authority, "10000000000000000uqsr")

	// Send uqsr to all the bond test accounts
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest1, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest2, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest3, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest4, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest5, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest6, "10000000uqsr")
	s.SendTokensToOneAddress(ctx, s.Quasar(), s.E2EBuilder.QuasarAccounts.Authority, s.E2EBuilder.QuasarAccounts.BondTest7, "10000000uqsr")

	// Send stake1 and uosmo and usdc to Osmosis authority account
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.MasterMinter, s.E2EBuilder.OsmosisAccounts.Owner, "10000000uosmo")
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.MasterMinter, s.E2EBuilder.OsmosisAccounts.NewOwner, "10000000uosmo")
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Owner, s.E2EBuilder.OsmosisAccounts.Authority, "10000000000000000stake1")
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.NewOwner, s.E2EBuilder.OsmosisAccounts.Authority, "10000000000000000usdc")
	s.SendTokensToOneAddress(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.MasterMinter, s.E2EBuilder.OsmosisAccounts.Authority, "1000000000000000uosmo")

	//walletAmount := ibc.WalletAmount{
	//	Address: s.E2EBuilder.OsmosisAccounts.Authority.Address,
	//	Denom:   "uayy",
	//	Amount:  1000000000,
	//}
	//transfer, err := s.Quasar().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, s.E2EBuilder.QuasarAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	//s.Require().NoError(err)
	//s.Require().NoError(transfer.Validate())
	//
	//walletAmount.Denom = "uqsr"
	//transfer, err = s.Quasar().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, s.E2EBuilder.QuasarAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	//s.Require().NoError(err)
	//s.Require().NoError(transfer.Validate())

	walletAmount := ibc.WalletAmount{
		Address: s.E2EBuilder.QuasarAccounts.Authority.Address,
		Denom:   "uosmo",
		Amount:  1000000000,
	}
	transfer, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Authority.KeyName, walletAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())
}
