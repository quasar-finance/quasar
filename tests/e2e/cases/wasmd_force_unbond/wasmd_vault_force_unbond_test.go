package wasmd_deposit

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
	BondAmount                     int64 = 10_000_000
	SharesAmount                   int64 = 10_000_000
	lpStrategyContractPath               = "../../../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath       = "../../../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath             = "../../../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path                     = "../_utils/pools/high_liquidity/balancer_pool1.json"
	osmosisPool2Path                     = "../_utils/pools/high_liquidity/balancer_pool2.json"
	osmosisPool3Path                     = "../_utils/pools/high_liquidity/balancer_pool3.json"
)

var (
	// Join
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

	ContractsDeploymentWallet *ibc.Wallet

	RewardsStoreID   uint64
	PrimitiveStoreID uint64
	VaultStoreID     uint64
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

	// Set up an account in quasar chain for contract deployment
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)
	s.CreatePools(ctx)
}

func (s *WasmdTestSuite) TestVaultContract_ForceUnbond() {
	t := s.T()
	ctx := context.Background()
	basicVaultAddress, lpAddresses := s.deployContracts(ctx, []map[string]any{init1, init2, init3})

	acc1 := s.createUserAndCheckBalances(ctx, 10_000_000)
	acc2 := s.createUserAndCheckBalances(ctx, 20_000_000)
	acc3 := s.createUserAndCheckBalances(ctx, 100_000_000)

	user1 := acc1.Bech32Address(s.Quasar().Config().Bech32Prefix)
	t.Log("Get user1 addr", user1)

	user2 := acc2.Bech32Address(s.Quasar().Config().Bech32Prefix)
	t.Log("Get user2 addr", user2)

	icaAddresses := s.getPrimitiveIcaAddresses(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	t.Log("Primitive1 address", lpAddresses[0])
	t.Log("Primitive2 address", lpAddresses[1])
	t.Log("Primitive3 address", lpAddresses[2])

	t.Log("uosmo balance of the primitives should be 0")
	balanceIca1, err := s.Osmosis().GetBalance(ctx, icaAddresses[0], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca1)
	balanceIca2, err := s.Osmosis().GetBalance(ctx, icaAddresses[1], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca2)
	balanceIca3, err := s.Osmosis().GetBalance(ctx, icaAddresses[2], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca3)

	// bond 3 users, 1 stays bonded so pool is not emptied
	s.executeBond(ctx, acc3, basicVaultAddress, 100_000_000)

	t.Log("Execute first clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	s.executeBond(ctx, acc1, basicVaultAddress, 10_000_000)

	t.Log("Execute first clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	s.executeBond(ctx, acc2, basicVaultAddress, 20_000_000)

	t.Log("Execute first clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	shares3AfterBond := s.getUserSharesBalance(ctx, acc3, basicVaultAddress)
	s.Require().Equal(int64(100_000_000-1), shares3AfterBond)

	shares1AfterBond := s.getUserSharesBalance(ctx, acc1, basicVaultAddress)
	s.Require().Equal(int64(10_050_250), shares1AfterBond) //8368200

	shares2AfterBond := s.getUserSharesBalance(ctx, acc2, basicVaultAddress)
	s.Require().Equal(int64(20_109_683), shares2AfterBond)

	t.Log("Execute force unbond for both users")
	s.executeForceUnbond(
		ctx,
		basicVaultAddress,
		[]string{acc1.Bech32Address(s.Quasar().Config().Bech32Prefix), acc2.Bech32Address(s.Quasar().Config().Bech32Prefix)},
	)

	t.Log("Execute first clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Check that the user1 shares balance is now 0")
	shares1AfterUnbond := s.getUserSharesBalance(ctx, acc1, basicVaultAddress)
	s.Require().Equal(int64(0), shares1AfterUnbond)

	t.Log("Check that the user2 shares balance is now 0")
	shares2AfterUnbond := s.getUserSharesBalance(ctx, acc2, basicVaultAddress)
	s.Require().Equal(int64(0), shares2AfterUnbond)

	balance1BeforeClaim, err := s.Quasar().GetBalance(ctx, acc1.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(balance1BeforeClaim, int64(0))

	balance2BeforeClaim, err := s.Quasar().GetBalance(ctx, acc2.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(balance2BeforeClaim, int64(0))

	//s.executeClaim(ctx, acc1, basicVaultAddress)
	//s.executeClaim(ctx, acc2, basicVaultAddress)

	t.Log("Execute force claim for both users")
	s.executeForceClaim(
		ctx,
		basicVaultAddress,
		[]string{acc1.Bech32Address(s.Quasar().Config().Bech32Prefix), acc2.Bech32Address(s.Quasar().Config().Bech32Prefix)},
	)

	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Execute second force claim for both users")
	s.executeForceClaim(
		ctx,
		basicVaultAddress,
		[]string{acc1.Bech32Address(s.Quasar().Config().Bech32Prefix), acc2.Bech32Address(s.Quasar().Config().Bech32Prefix)},
	)

	t.Log("Execute clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	trappedErrorsAfterSecondBond := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[0]))
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[1]))
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[2]))

	balance1AfterWithdraw, err := s.Quasar().GetBalance(ctx, acc1.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(9_937_767), balance1AfterWithdraw)

	balance2AfterWithdraw, err := s.Quasar().GetBalance(ctx, acc2.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(21_548_481), balance2AfterWithdraw)
}

func (s *WasmdTestSuite) deployContracts(ctx context.Context, inits []map[string]any) (string, []string) {
	t := s.T()

	t.Log("Deploy the lp strategy contract")
	lpAddress1, lpAddress2, lpAddress3 := s.deployPrimitives(ctx, s.ContractsDeploymentWallet, lpStrategyContractPath, "lp_strategy_test", inits[0], inits[1], inits[2])

	t.Log("Deploy reward contract")
	s.deployRewardsContract(ctx, s.ContractsDeploymentWallet, vaultRewardsContractPath)

	t.Log("Deploy basic vault contract")
	basicVaultAddress := s.deployVault(ctx, s.ContractsDeploymentWallet, basicVaultStrategyContractPath, "basic_vault",
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
					"address": lpAddress1,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": inits[0],
					},
				},
				{
					"address": lpAddress2,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": inits[1],
					},
				},
				{
					"address": lpAddress3,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": inits[2],
					},
				},
			},
		})

	t.Log("Set depositors for all the primitives")
	s.setDepositorForContracts(ctx, s.ContractsDeploymentWallet,
		map[string]any{
			"set_depositor": map[string]any{
				"depositor": basicVaultAddress,
			},
		},
		[]string{lpAddress1, lpAddress2, lpAddress3},
	)

	return basicVaultAddress, []string{lpAddress1, lpAddress2, lpAddress3}
}

func (s *WasmdTestSuite) createUserAndCheckBalances(ctx context.Context, amount int64) *ibc.Wallet {
	t := s.T()

	t.Log("Create testing account on Quasar chain with some QSR tokens for fees")
	acc := s.CreateUserAndFund(ctx, s.Quasar(), 10_000_000) // unused qsr, just for tx fees

	t.Log("Fund testing account with uosmo via IBC transfer from Osmosis chain Treasury account")
	walletAmount := ibc.WalletAmount{Address: acc.Bech32Address(s.Quasar().Config().Bech32Prefix), Denom: s.Osmosis().Config().Denom, Amount: amount}
	transfer, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, walletAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	t.Log("Wait for packet transfer and the ibc transfer to occur")
	err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check tester accounts uosmo balance after executing IBC transfer")
	balanceTester, err := s.Quasar().GetBalance(ctx, acc.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(amount, balanceTester)

	return acc
}

func (s *WasmdTestSuite) executeBond(ctx context.Context, acc *ibc.Wallet, basicVaultAddress string, amount int64) {
	t := s.T()

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		acc.KeyName,
		basicVaultAddress,
		sdk.NewCoins(sdk.NewInt64Coin(s.OsmosisDenomInQuasar, amount)),
		map[string]any{"bond": map[string]any{}},
		nil,
	)

	t.Log("Wait 5 blocks on quasar and osmosis to settle up ICA packet transfer and the IBC transfer (bond)")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)
}

func (s *WasmdTestSuite) executeUnbond(ctx context.Context, acc *ibc.Wallet, basicVaultAddress string) {
	t := s.T()

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		acc.KeyName,
		basicVaultAddress,
		sdk.NewCoins(),
		map[string]any{"unbond": map[string]any{"amount": "5000000"}},
		nil,
	)

	t.Log("Wait 5 blocks on quasar and osmosis to settle up ICA packet unbond and the IBC transfer (unbond)")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)
}

func (s *WasmdTestSuite) executeClaim(ctx context.Context, acc *ibc.Wallet, basicVaultAddress string) {
	t := s.T()

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		acc.KeyName,
		basicVaultAddress,
		sdk.NewCoins(),
		map[string]any{"claim": map[string]any{}},
		nil,
	)
	t.Log("Wait 5 block on quasar and osmosis to settle up ICA packet claim and the IBC transfer (claim)")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)
}

func (s *WasmdTestSuite) executeClearCache(ctx context.Context, basicVaultAddress string) {
	t := s.T()

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.ContractsDeploymentWallet.KeyName,
		basicVaultAddress,
		sdk.Coins{},
		map[string]any{"clear_cache": map[string]any{}},
		nil,
	)

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the IBC transfer (clear_cache)")
	err := testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)
}

func (s *WasmdTestSuite) executeSandwichAttackJoin(ctx context.Context) {
	// Sandwich-attack as we know in this test how we are going to swap, we clone the tx and we execute it before the ICQ/ICA is doing the job simulating a front-run sandwich attack
	s.SwapTokenOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, "3333333uosmo", "1", "stake1", "1")
	s.SwapTokenOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, "3333333uosmo", "1", "usdc", "2")
	s.SwapTokenOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, "3333333uosmo", "1", "fakestake", "3")
}

func (s *WasmdTestSuite) executeForceUnbond(ctx context.Context, basicVaultAddress string, accs []string) {
	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.ContractsDeploymentWallet.KeyName,
		basicVaultAddress,
		sdk.NewCoins(), // empty amount
		map[string]any{"force_unbond": map[string]any{
			"addresses": accs,
		}},
		nil,
	)
}

func (s *WasmdTestSuite) executeForceClaim(ctx context.Context, basicVaultAddress string, accs []string) {
	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.ContractsDeploymentWallet.KeyName,
		basicVaultAddress,
		sdk.NewCoins(), // empty amount
		map[string]any{"force_claim": map[string]any{
			"addresses": accs,
		}},
		nil,
	)
}

func (s *WasmdTestSuite) getUserSharesBalance(ctx context.Context, acc *ibc.Wallet, basicVaultAddress string) int64 {
	var data testsuite.ContractBalanceData
	balanceBytes := s.ExecuteContractQuery(
		ctx,
		s.Quasar(),
		basicVaultAddress,
		map[string]any{
			"balance": map[string]any{
				"address": acc.Bech32Address(s.Quasar().Config().Bech32Prefix),
			},
		},
	)
	err := json.Unmarshal(balanceBytes, &data)
	s.Require().NoError(err)
	balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
	s.Require().NoError(err)

	return balance
}

func (s *WasmdTestSuite) getPrimitiveIcaAddresses(ctx context.Context, lpAddresses []string) []string {
	var icaAddresses []string
	for _, lpAddress := range lpAddresses {
		var icaAddress testsuite.ContractIcaAddressData
		icaAddress1Bytes := s.ExecuteContractQuery(
			ctx,
			s.Quasar(),
			lpAddress,
			map[string]any{
				"ica_address": map[string]any{},
			},
		)
		err := json.Unmarshal(icaAddress1Bytes, &icaAddress)
		s.Require().NoError(err)
		icaAddresses = append(icaAddresses, icaAddress.Data.Address)
	}

	return icaAddresses
}

func (s *WasmdTestSuite) getTrappedErrors(ctx context.Context, lpAddresses []string) []map[string]interface{} {
	var trappedErrors []map[string]interface{}
	for _, lpAddress := range lpAddresses {
		var trappedErrors1 testsuite.ContractTrappedErrorsData
		trappedErrors1Bytes := s.ExecuteContractQuery(
			ctx,
			s.Quasar(),
			lpAddress,
			map[string]any{
				"trapped_errors": map[string]any{},
			},
		)
		err := json.Unmarshal(trappedErrors1Bytes, &trappedErrors1)
		s.Require().NoError(err)
		trappedErrors = append(trappedErrors, trappedErrors1.Data.TrappedErrors)
	}

	return trappedErrors
}
