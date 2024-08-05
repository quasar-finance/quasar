package qtransfer

import (
	"context"
	"encoding/json"
	"strconv"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	connectiontypes "github.com/cosmos/ibc-go/v8/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v8/modules/core/04-channel/types"
	testsuite "github.com/quasar-finance/quasar/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	lpStrategyContractPath         = "../../../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath = "../../../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath       = "../../../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path               = "../_utils/pools/high_liquidity/balancer_pool1.json"
	osmosisPool2Path               = "../_utils/pools/high_liquidity/balancer_pool2.json"
	osmosisPool3Path               = "../_utils/pools/high_liquidity/balancer_pool3.json"

	QTUserFundAmount int64 = 1_003_500
	QTTransferAmount int64 = 1_000_000

	QSLDstartingTokenAmount int64 = 100_000_000_000
	QSLDbondAmount          int64 = 10_000_000
	QSLDexpectedShares      int64 = 9_999_999
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

func TestQtransfer(t *testing.T) {
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
			"deposit_denom":                 s.OsmosisDenomInQuasar,
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
	// build tx to transfer uosmo to quasar chain to related quasar1 account of userKey previously generated
	amount := ibc.WalletAmount{
		Address: s.BasicVaultContractAddress, // recipient in quasar chain (`wasm["contract"] should be the same as the receiver of the packet`)
		Denom:   s.Osmosis().Config().Denom,
		Amount:  QTTransferAmount,
	}
	// set timeout
	ibcTimeout := ibc.IBCTimeout{NanoSeconds: 1} // setting lowest timeoutTimestamp
	// Build memo field
	msgMap := map[string]interface{}{
		"bond": map[string]interface{}{
			"recipient": user.Bech32Address(s.Quasar().Config().Bech32Prefix),
		},
	}
	memoMap := map[string]interface{}{
		"wasm": map[string]interface{}{
			"contract": s.BasicVaultContractAddress,
			"msg":      msgMap,
		},
	}
	memoBytes, err := json.Marshal(memoMap)
	s.Require().NoError(err)
	// execute ibc transfer tx
	tx, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, user.KeyName, amount, ibc.TransferOptions{Timeout: &ibcTimeout, Memo: string(memoBytes)})
	s.Require().NoError(err)
	s.Require().NoError(tx.Validate())

	t.Log("Check user balance after executing IBC transfer expecting to be 0")
	userBalanceAfterTransfer, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QTUserFundAmount-QTTransferAmount-3500, userBalanceAfterTransfer)

	t.Log("Wait for transfer packet to timeout")
	err = testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check user balance after packet timeout expecting to be 0 on Quasar side")
	userBalanceOsmoAfterTimeout, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(0), userBalanceOsmoAfterTimeout)

	t.Log("Check user balance after packet timeout expecting to be the transfer amount")
	userBalanceAfterTimeout, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QTTransferAmount, userBalanceAfterTimeout)
}

// TestQtransferStrategyLpDepositOK tests the LP strategy contract by creating an Interchain Account (ICA) channel between Osmosis and Quasar.
// It involves depositing 10 OSMO (represented as 10000000uosmo) into the user's account on Osmosis, and then initiating an IBC transfer to Quasar.
// This transfer triggers the QTransfer module's IBC hooks on the Quasar side, leading to an interaction with the contract.
func (s *Qtransfer) TestQtransferStrategyLpDeposit() {
	t := s.T()
	ctx := context.Background()

	// Variables
	bondAmount := sdk.NewInt64Coin(s.OsmosisDenomInQuasar, QSLDbondAmount) // this is the bonding amount for the vault deposit
	expectedShares := QSLDexpectedShares                                   // this is the expected amount of shares in $OPRO balance
	expectedDeviation := 0.01                                              // this is the maximum allowed deviation as we cant predict esde cases as slippage or others thing that are gAMM module related

	t.Log("Create an user with fund on Osmosis chain")
	user := s.CreateUserAndFund(ctx, s.Osmosis(), QSLDstartingTokenAmount)

	t.Log("Check user balance before executing IBC transfer expecting to be the funded amount")
	userBalanceOsmo, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount, userBalanceOsmo)

	t.Log("Execute IBC transfer to Quasar with Memo to deposit on LP-Strategy vault for user: ", user.Bech32Address(s.Quasar().Config().Bech32Prefix))
	amountOsmo := ibc.WalletAmount{
		Address: s.BasicVaultContractAddress, // recipient in quasar chain (`wasm["contract"] should be the same as the receiver of the packet`)
		Denom:   s.Osmosis().Config().Denom,
		Amount:  bondAmount.Amount.Int64(),
	}
	// Build memo field -
	// This memo field triggers an ibc-hooks execution.
	// It will execute the bond action for the given recipient and the amountOsmo denom on the Basic Vault LP strategy
	msgMap := map[string]interface{}{
		"bond": map[string]interface{}{
			"recipient": user.Bech32Address(s.Quasar().Config().Bech32Prefix),
		},
	}
	memoMap := map[string]interface{}{
		"wasm": map[string]interface{}{
			"contract": s.BasicVaultContractAddress,
			"msg":      msgMap,
		},
	}
	memoBytes, err := json.Marshal(memoMap)
	s.Require().NoError(err)
	// executing the ibc transfer
	txOsmo, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, user.KeyName, amountOsmo, ibc.TransferOptions{Memo: string(memoBytes)})
	s.Require().NoError(err)
	s.Require().NoError(txOsmo.Validate())

	t.Log("Check user balance before executing IBC transfer expecting to be less than the funded amount")
	userBalanceAfterOsmo, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(QSLDstartingTokenAmount-bondAmount.Amount.Int64()-3500, userBalanceAfterOsmo) // funded amount, less bond amount, less fee

	// executing clear_cache function inside the basic vault contract to ensure forced processing of bond user action
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

	t.Log("Checking $OPRO balance for user: ", user.Bech32Address(s.Quasar().Config().Bech32Prefix))
	var data testsuite.ContractBalanceData
	// querying the user address $OPRO balance after bonding
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

	// Here's how the smart contract calculates the share amount that needs to be minted.
	// It starts by going through each bond stub and the associated primitives of the investment,
	// kicking things off with a zero Uint128. For every pair of bond stub and primitive,
	// it grabs the share amount from the bond response. If the bond response ain't empty,
	// it switches the share amount into a decimal from its Uint128 form.
	// This decimal then gets converted back to Uint128, using the floor rounding method.
	// This share amount is then added to the running total (which starts from zero).
	// If anything goes awry during this process, the contract chucks an error and everything grinds to a halt.
	// All this adding up eventually gives us the total shares that needs to be minted.
	// Related contract logic: smart-contracts/contracts/basic-vault/src/callback.rs::103

	deviationFromExpectedShares := float64(balance)/float64(expectedShares) - 1
	// Verifying the final user balance is within expected range
	// The balance should be approximately equal to the expected shares,
	// accounting for a small deviation to handle unpredictable events like slippage.
	s.Require().InDelta(0, deviationFromExpectedShares, expectedDeviation, "User balance deviates from expected shares by more than the expected deviation.")
}
