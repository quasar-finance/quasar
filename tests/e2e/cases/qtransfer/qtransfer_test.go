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
	lpStrategyContractPath         = "../../../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath = "../../../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath       = "../../../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path               = "../_utils/sample_pool1.json"
	osmosisPool2Path               = "../_utils/sample_pool2.json"
	osmosisPool3Path               = "../_utils/sample_pool3.json"

	QTUserFundAmount int64 = 1_003_500
	QTTransferAmount int64 = 1_000_000

	QSLDstartingTokenAmount int64 = 100_000_000_000
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

	s := &Qtransfer{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type Qtransfer struct {
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

func (s *Qtransfer) SetupSuite() {
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

// TestQtransfer_Timeout
func (s *Qtransfer) TestQtransfer_Timeout() {
	t := s.T()
	ctx := context.Background()

	t.Log("Create an user with fund on Quasar chain")
	user := s.CreateUserAndFund(ctx, s.Osmosis(), QTUserFundAmount)

	t.Log("Check user balance before executing IBC transfer expecting to be the funded amount")
	userBalanceBefore, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QTUserFundAmount, userBalanceBefore)

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
	// build tx to transfer uosmo to quasar chain to related quasar1 account of userKey previously generated
	amount := ibc.WalletAmount{
		Address: user.Bech32Address(s.Quasar().Config().Bech32Prefix),
		Denom:   s.Osmosis().Config().Denom,
		Amount:  QTTransferAmount,
	}
	// set timeout
	ibcTimeout := ibc.IBCTimeout{NanoSeconds: 1, Height: 1} // setting lowest timeoutTimestamp and height
	// execute ibc transfer tx
	tx, err := s.Osmosis().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, user.KeyName, amount, ibc.TransferOptions{Timeout: &ibcTimeout, Memo: string(memoBytes)})
	s.Require().NoError(err)
	s.Require().NoError(tx.Validate())

	t.Log("Check user balance after executing IBC transfer expecting to be 0")
	userBalanceAfterTransfer, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QTUserFundAmount-QTTransferAmount-3500, userBalanceAfterTransfer)

	t.Log("Wait for transfer packet to timeout")
	err = testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check user balance after packet timeout expecting to be 0 on Quasar side") // TODO check if this pass due to timeout or wrong Memo
	userBalanceOsmoAfterTimeout, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(0), userBalanceOsmoAfterTimeout)

	t.Log("Check user balance after packet timeout expecting to be the transfer amount")
	userBalanceAfterTimeout, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QTTransferAmount, userBalanceAfterTimeout)
}

// TestQtransferStrategyLpDepositOK tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *Qtransfer) TestQtransferStrategyLpDepositOK() {
	t := s.T()
	ctx := context.Background()

	// Variables
	bondAmount := sdk.NewInt64Coin(s.OsmosisDenomInQuasar, 10000000)
	expectedShares := 9999999
	expectedDeviation := 0.01

	t.Log("Create an user with fund on Osmosis chain")
	user := s.CreateUserAndFund(ctx, s.Osmosis(), QSLDstartingTokenAmount)
	t.Log("Check user balance before executing IBC transfer expecting to be the funded amount")
	userBalanceOsmo, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount, userBalanceOsmo)

	t.Log("Execute IBC transfer to Quasar with Memo to deposit on LP-Strategy vault")
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
	amountOsmo := ibc.WalletAmount{
		Address: user.Bech32Address(s.Quasar().Config().Bech32Prefix), // recipient in quasar chain, TODO the basic vault or the user??? try -> s.BasicVaultContractAddress
		Denom:   s.Osmosis().Config().Denom,
		Amount:  bondAmount.Amount.Int64(),
	}
	optionsOsmo := ibc.TransferOptions{Memo: string(memoBytes)}
	txOsmo, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, user.KeyName, amountOsmo, optionsOsmo)
	s.Require().NoError(err)
	s.Require().NoError(txOsmo.Validate())

	t.Log("Check user balance before executing IBC transfer expecting to be less than the funded amount")
	// check uosmo balance
	userBalanceAfterOsmo, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount-bondAmount.Amount.Int64()-3500, userBalanceAfterOsmo) // funded amount, less bond amount, less fee

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.ContractsDeploymentWallet.KeyName,
		s.BasicVaultContractAddress,
		sdk.Coins{},
		map[string]any{"clear_cache": map[string]any{}},
		nil,
	)

	t.Log("Wait for quasar and osmosis to clear cache and settle up ICA packet transfer and the ibc transfer")
	err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	var data testsuite.ContractBalanceData
	balanceBytes := s.ExecuteContractQuery(
		ctx,
		s.Quasar(),
		s.BasicVaultContractAddress,
		map[string]any{
			"balance": map[string]any{
				"address": user.Bech32Address(s.Quasar().Config().Bech32Prefix),
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

func (s *Qtransfer) TestQtransferStrategyLpDepositKO() {
	// TODO just duplicate the above test
	// but pass an amount that should cause an insufficient balance error
	// and validate with s.Require().Error(err) instead of NoError(err)
}
