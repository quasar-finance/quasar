package qtransfer

import (
	"context"
	"encoding/json"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"strconv"
	"testing"

	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	QSLDstartingTokenAmount        int64 = 100_000_000_000
	lpStrategyContractPath               = "../../../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath       = "../../../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath             = "../../../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path                     = "../_utils/sample_pool1.json"
	osmosisPool2Path                     = "../_utils/sample_pool2.json"
	osmosisPool3Path                     = "../_utils/sample_pool3.json"
)

var (
	init1 = map[string]any{
		"lock_period": 6, "pool_id": 1, "pool_denom": "gamm/pool/1", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "stake1",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init2 = map[string]any{
		"lock_period": 6, "pool_id": 2, "pool_denom": "gamm/pool/2", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "usdc",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init3 = map[string]any{
		"lock_period": 6, "pool_id": 3, "pool_denom": "gamm/pool/3", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "fakestake",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
)

func TestQtransferStrategyLpDeposit(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &QtransferStrategyLpDeposit{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type QtransferStrategyLpDeposit struct {
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

func (s *QtransferStrategyLpDeposit) SetupSuite() {
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
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), QSLDstartingTokenAmount)

	// Send tokens "uayy" and "uqsr" from Quasar to Osmosis account
	s.SendTokensToRespectiveAccounts(ctx)

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

// TestQtransferStrategyLpDepositOK tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *QtransferStrategyLpDeposit) TestQtransferStrategyLpDepositOK() {
	t := s.T()
	ctx := context.Background()

	// Variables
	bondAmount := sdk.NewInt64Coin(s.OsmosisDenomInQuasar, 10000000)
	expectedShares := 9999999
	expectedDeviation := 0.01

	t.Log("Create an user with fund on Quasar chain")
	user := s.CreateUserAndFund(ctx, s.Quasar(), QSLDstartingTokenAmount)
	//err := s.Quasar().SendFunds(ctx, "faucet", ibc.WalletAmount{
	//	Address: user.Bech32Address(s.Quasar().Config().Bech32Prefix),
	//	Amount:  QSLDstartingTokenAmount,
	//	Denom:   s.OsmosisDenomInQuasar,
	//})
	//s.Require().NoError(err)
	faucet := s.CreateUserAndFund(ctx, s.Osmosis(), QSLDstartingTokenAmount)
	amountOsmo := ibc.WalletAmount{
		Address: faucet.Bech32Address(s.Osmosis().Config().Bech32Prefix),
		Denom:   s.Osmosis().Config().Denom,
		Amount:  bondAmount.Amount.Int64(),
	}
	ibcTimeoutOsmo := ibc.IBCTimeout{NanoSeconds: 0, Height: 0}
	optionsOsmo := ibc.TransferOptions{Timeout: &ibcTimeoutOsmo, Memo: ""}
	txOsmo, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, faucet.KeyName, amountOsmo, optionsOsmo)
	s.Require().NoError(err)
	s.Require().NoError(txOsmo.Validate())

	t.Log("Wait for uosmo ibc transfer from faucet is relayed")
	err = testutil.WaitForBlocks(ctx, 20, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check user balance before executing IBC transfer expecting to be the funded amount")
	// check uqsr balance
	userBalanceBeforeQsr, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.Quasar().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount, userBalanceBeforeQsr)
	// check uosmo balance
	userBalanceBeforeOsmo, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount, userBalanceBeforeOsmo)

	t.Log("Execute IBC Transfer from previously created user")
	// Build memo field
	msgMap := map[string]interface{}{
		"bond": map[string]interface{}{},
	}
	memoMap := map[string]interface{}{
		"wasm": map[string]interface{}{
			"contract": s.BasicVaultContractAddress,
			"msg":      msgMap,
		},
	}
	memoBytes, err := json.Marshal(memoMap)
	s.Require().NoError(err)

	//  Build ICS20 with Memo transaction to trigger contract execution
	amount := ibc.WalletAmount{
		Address: user.Bech32Address(s.Quasar().Config().Bech32Prefix),
		Denom:   s.OsmosisDenomInQuasar,
		Amount:  bondAmount.Amount.Int64(),
	}
	ibcTimeout := ibc.IBCTimeout{NanoSeconds: 0, Height: 0}
	options := ibc.TransferOptions{Timeout: &ibcTimeout, Memo: string(memoBytes)}
	tx, err := s.Quasar().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, user.KeyName, amount, options)
	s.Require().NoError(err)
	s.Require().NoError(tx.Validate())

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
	err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
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
				"address": user.Address,
			},
		},
	)

	err = json.Unmarshal(balanceBytes, &data)
	s.Require().NoError(err)

	balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
	s.Require().NoError(err)

	s.Require().True(int64(float64(expectedShares)*(1-expectedDeviation)) <= balance)
	s.Require().True(balance <= int64(float64(expectedShares)*(1+expectedDeviation)))
}

func (s *QtransferStrategyLpDeposit) TestQtransferStrategyLpDepositKO() {
	// TODO just duplicate the above test
	// but pass an amount that should cause an insufficient balance error
	// and validate with s.Require().Error(err) instead of NoError(err)
}
