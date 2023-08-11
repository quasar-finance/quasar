package wasmd

import (
	"context"
	"encoding/json"
	"github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
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

const (
	StartingTokenAmount            int64 = 100_000_000_000
	lpStrategyContractPath               = "../../../../smart-contracts/artifacts/lp_strategy.wasm"
	basicVaultStrategyContractPath       = "../../../../smart-contracts/artifacts/basic_vault.wasm"
	vaultRewardsContractPath             = "../../../../smart-contracts/artifacts/vault_rewards.wasm"
	osmosisPool1Path                     = "../_utils/sample_pool1.json"
	osmosisPool2Path                     = "../_utils/sample_pool2.json"
	osmosisPool3Path                     = "../_utils/sample_pool3.json"
)

var (
	init1 = map[string]any{
		"lock_period": 1, "pool_id": 1, "pool_denom": "gamm/pool/1", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "stake1",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init2 = map[string]any{
		"lock_period": 1, "pool_id": 2, "pool_denom": "gamm/pool/2", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "usdc",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init3 = map[string]any{
		"lock_period": 1, "pool_id": 3, "pool_denom": "gamm/pool/3", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "fakestake",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
)

func TestWasmdTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
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

	ContractsDeploymentWallet *ibc.Wallet

	RewardsStoreID            uint64
	PrimitiveStoreID          uint64
	VaultStoreID              uint64
	BasicVaultContractAddress string
}

func (s *WasmdTestSuite) SetupSuite() {
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

	// Generate the ibc denom of native tokens in other chains
	s.OsmosisDenomInQuasar = helpers.IbcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.QuasarDenomInOsmosis = helpers.IbcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// Setup an account in quasar chain for contract deployment
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Send tokens to the respective account and create the required pools
	s.CreatePools(ctx)

	// Deploy the lp strategy contract
	s.deployPrimitives(ctx, s.ContractsDeploymentWallet, lpStrategyContractPath, "lp_strategy_test", init1, init2, init3)

	// Deploy reward contract
	s.deployRewardsContract(ctx, s.ContractsDeploymentWallet, vaultRewardsContractPath)

	// deploy basic_vault contract
	s.BasicVaultContractAddress = s.deployVault(ctx, s.ContractsDeploymentWallet, basicVaultStrategyContractPath, "basic_vault",
		map[string]any{
			"total_cap":                     "200000000000",
			"thesis":                        "e2e",
			"vault_rewards_code_id":         s.RewardsStoreID,
			"reward_token":                  map[string]any{"native": "uqsr"},
			"reward_distribution_schedules": []string{},
			"decimals":                      6,
			"symbol":                        "ORN",
			"min_withdrawal":                "1",
			"name":                          "ORION",
			"deposit_denom":                 "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			"primitives": []map[string]any{
				{
					"address": s.LpStrategyContractAddress1,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init1,
					},
				},
				{
					"address": s.LpStrategyContractAddress2,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init2,
					},
				},
				{
					"address": s.LpStrategyContractAddress3,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init3,
					},
				},
			},
		})

	// set depositors for all the primitives
	s.setDepositorForContracts(ctx, s.ContractsDeploymentWallet,
		map[string]any{
			"set_depositor": map[string]any{
				"depositor": s.BasicVaultContractAddress,
			},
		},
	)
}

// TestLpStrategyContract_SuccessfulDeposit tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *WasmdTestSuite) TestLpStrategyContract_SuccessfulDeposit() {
	t := s.T()
	ctx := context.Background()

	t.Log("Create testing accounts on Quasar chain")
	accBondTest0 := s.CreateUserAndFund(ctx, s.Quasar(), 1_000_000) // unused qsr, just for tx fees
	accBondTest1 := s.CreateUserAndFund(ctx, s.Quasar(), 1_000_000) // unused qsr, just for tx fees
	accBondTest2 := s.CreateUserAndFund(ctx, s.Quasar(), 1_000_000) // unused qsr, just for tx fees

	t.Log("Fund testing accounts with uosmo via IBC transfer from Osmosis chain Treasury account")
	walletAmount0 := ibc.WalletAmount{Address: accBondTest0.Bech32Address(s.Quasar().Config().Bech32Prefix), Denom: s.Osmosis().Config().Denom, Amount: 10_000_000}
	transfer, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, walletAmount0, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())
	// Transfer "uosmo" denom to Quasar accounts via IBC Transfer - accBondTest1
	walletAmount1 := ibc.WalletAmount{Address: accBondTest1.Bech32Address(s.Quasar().Config().Bech32Prefix), Denom: s.Osmosis().Config().Denom, Amount: 1_000_000}
	transfer, err = s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, walletAmount1, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())
	// Transfer "uosmo" denom to Quasar accounts via IBC Transfer - accBondTest2
	walletAmount2 := ibc.WalletAmount{Address: accBondTest2.Bech32Address(s.Quasar().Config().Bech32Prefix), Denom: s.Osmosis().Config().Denom, Amount: 1_000_000}
	transfer, err = s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, walletAmount2, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	t.Log("Wait for packet transfer and the ibc transfer to occur to all three accounts")
	err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check tester accounts uosmo balance after executing IBC transfer")
	balanceTester0, err := s.Quasar().GetBalance(ctx, accBondTest0.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(10_000_000), balanceTester0)
	balanceTester1, err := s.Quasar().GetBalance(ctx, accBondTest1.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(1_000_000), balanceTester1)
	balanceTester2, err := s.Quasar().GetBalance(ctx, accBondTest2.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(1_000_000), balanceTester2)

	testCases := []struct {
		Account                  ibc.Wallet // necessary field
		BondAmount               sdk.Coins  // necessary in case of bonds
		UnbondAmount             string     // only in case of Action is "unbond"
		Action                   string     // necessary to provide action, 3 possibilities "bond", "unbond" or "claim"
		expectedShares           int64      // only needed in case of "bond"
		expectedDeviation        float64    // only needed in case of "bond"
		expectedNumberOfUnbonds  int64      // only needed in case of "unbond"
		expectedBalanceChange    uint64     // only needed in case of "claim"
		expectedBalanceDeviation float64    // only needed in case of "claim"
	}{
		{
			Account:           *accBondTest0,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 10000000)),
			expectedShares:    9999999,
			expectedDeviation: 0.01,
		},
		{
			Account:           *accBondTest1,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 1000000)),
			expectedShares:    1015176,
			expectedDeviation: 0.01,
		},
		{
			Account:                 *accBondTest0,
			Action:                  "unbond",
			BondAmount:              sdk.NewCoins(),
			UnbondAmount:            "1000",
			expectedNumberOfUnbonds: 1,
		},
		{
			Account:                 *accBondTest0,
			Action:                  "unbond",
			BondAmount:              sdk.NewCoins(),
			UnbondAmount:            "2000",
			expectedNumberOfUnbonds: 2,
		},
		{
			Account:                  *accBondTest0,
			Action:                   "claim",
			expectedBalanceChange:    1000,
			expectedBalanceDeviation: 0.1,
		},
		{
			Account:           *accBondTest2,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 1000000)),
			expectedShares:    1015176,
			expectedDeviation: 0.01,
		},
	}

	for _, tc := range testCases {
		switch tc.Action {
		case "bond":
			// execute bond transaction
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				tc.BondAmount,
				map[string]any{"bond": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
			err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			var data testsuite.ContractBalanceData
			balanceBytes := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.BasicVaultContractAddress,
				map[string]any{
					"balance": map[string]any{
						"address": tc.Account.Bech32Address(s.Quasar().Config().Bech32Prefix),
					},
				},
			)

			err = json.Unmarshal(balanceBytes, &data)
			s.Require().NoError(err)

			balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
			s.Require().NoError(err)

			s.Require().True(int64(float64(tc.expectedShares)*(1-tc.expectedDeviation)) <= balance)
			s.Require().True(balance <= int64(float64(tc.expectedShares)*(1+tc.expectedDeviation)))
		case "unbond":
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": tc.UnbondAmount}},
				nil,
			)

			t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
			err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			var pendingUnbondsData testsuite.PendingUnbondsData
			pendingUnbondsBytes := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.BasicVaultContractAddress,
				map[string]any{
					"pending_unbonds": map[string]any{
						"address": tc.Account.Bech32Address(s.Quasar().Config().Bech32Prefix),
					},
				},
			)

			err = json.Unmarshal(pendingUnbondsBytes, &pendingUnbondsData)
			s.Require().NoError(err)

			// verify if the unbonded amount and expected number of unbonds matches their respective conditions or not
			s.Require().Equal(tc.expectedNumberOfUnbonds, int64(len(pendingUnbondsData.Data.PendingUnbonds)))
			s.Require().Equal(tc.UnbondAmount, pendingUnbondsData.Data.PendingUnbonds[tc.expectedNumberOfUnbonds-1].Shares)
		case "claim":
			tn := testsuite.GetFullNode(s.Quasar())
			cmds := []string{"bank", "balances", tc.Account.Bech32Address(s.Quasar().Config().Bech32Prefix),
				"--output", "json",
			}

			res, _, err := tn.ExecQuery(ctx, cmds...)
			s.Require().NoError(err)

			var balanceBefore testsuite.QueryAllBalancesResponse
			err = json.Unmarshal(res, &balanceBefore)
			s.Require().NoError(err)

			pending := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.BasicVaultContractAddress,
				map[string]any{
					"pending_unbonds": map[string]any{
						"address": tc.Account.Bech32Address(s.Quasar().Config().Bech32Prefix),
					},
				},
			)
			t.Log(string(pending))

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 20, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 20, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			tn = testsuite.GetFullNode(s.Quasar())
			res, _, err = tn.ExecQuery(ctx, cmds...)
			s.Require().NoError(err)

			var balanceAfter testsuite.QueryAllBalancesResponse
			err = json.Unmarshal(res, &balanceAfter)
			s.Require().NoError(err)

			errorsPrim1 := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.LpStrategyContractAddress1,
				map[string]any{
					"trapped_errors": map[string]any{
					},
				},
			)
			t.Log(string(errorsPrim1))
			errorsPrim2 := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.LpStrategyContractAddress2,
				map[string]any{
					"trapped_errors": map[string]any{
					},
				},
			)
			t.Log(string(errorsPrim2))
			errorsPrim3 := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.LpStrategyContractAddress3,
				map[string]any{
					"trapped_errors": map[string]any{
					},
				},
			)
			t.Log(string(errorsPrim3))

			balanceChange := balanceAfter.Balances.AmountOf(s.OsmosisDenomInQuasar).Sub(balanceBefore.Balances.AmountOf(s.OsmosisDenomInQuasar)).Int64()
			s.Require().True(int64(float64(tc.expectedBalanceChange)*(1-tc.expectedBalanceDeviation)) <= balanceChange)
			t.Logf("%d",balanceChange)

			balance := balanceAfter.Balances.AmountOf(s.OsmosisDenomInQuasar).Int64()
			t.Logf("%d", balanceBefore.Balances.AmountOf(s.OsmosisDenomInQuasar).Int64())
			t.Logf("%d",balance)
			s.Require().True(balanceChange <= int64(float64(tc.expectedBalanceChange)*(1+tc.expectedBalanceDeviation)))
		default:
			t.Log("This testCase does not contain any transaction type")
		}
	}
}
