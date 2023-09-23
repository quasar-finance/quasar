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
	osmosisPool1Path                     = "../_utils/pools/low_liquidity/balancer_pool1.json"
	osmosisPool2Path                     = "../_utils/pools/low_liquidity/balancer_pool2.json"
	osmosisPool3Path                     = "../_utils/pools/low_liquidity/balancer_pool3.json"
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
	// Exit
	init4 = map[string]any{
		"lock_period": 6, "pool_id": 4, "pool_denom": "gamm/pool/4", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "stake1",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init5 = map[string]any{
		"lock_period": 6, "pool_id": 5, "pool_denom": "gamm/pool/5", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "usdc",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init6 = map[string]any{
		"lock_period": 6, "pool_id": 6, "pool_denom": "gamm/pool/6", "base_denom": "uosmo",
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

	// Create Pools twice in order to create them in a range from poolId 1 to 6 for both test cases as Join and Exit
	s.CreatePools(ctx)
	s.CreatePools(ctx)
}

func (s *WasmdTestSuite) TestVaultContract_ForceUnbond() {
	t := s.T()
	ctx := context.Background()
	basicVaultAddress, lpAddresses := s.deployContracts(ctx, []map[string]any{init1, init2, init3})

	// create user and check his balance
	acc1 := s.createUserAndCheckBalances(ctx, 10_000_000)
	acc2 := s.createUserAndCheckBalances(ctx, 20_000_000)

	t.Log("Execute 1st bond transaction, for user 1")
	s.executeBond(ctx, acc1, basicVaultAddress, 10_000_000)

	t.Log("Execute 2nd bond transaction, for user 2")
	s.executeBond(ctx, acc2, basicVaultAddress, 20_000_000)

	t.Log("Execute first clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Check that the user1 shares balance is 10 OSMO")
	balance1 := s.getUserSharesBalance(ctx, acc1, basicVaultAddress)
	s.Require().Equal(int64(10_000_000-1), balance1)

	t.Log("Check that the user2 shares balance is 20 OSMO")
	balance2 := s.getUserSharesBalance(ctx, acc2, basicVaultAddress)
	s.Require().Equal(int64(20_000_000-1), balance2)

	t.Log("Get the counterparty ICA osmo1 addresses for each one of the primitive and check their uosmo balances after executing bond that failed")
	icaAddresses := s.getPrimitiveIcaAddresses(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})

	t.Log("Check uOSMO balance of the primitives looking for BOND_AMOUNT/3 on each one of them")
	balanceIca1, err := s.Osmosis().GetBalance(ctx, icaAddresses[0], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(BondAmount/3, balanceIca1)
	balanceIca2, err := s.Osmosis().GetBalance(ctx, icaAddresses[1], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(BondAmount/3, balanceIca2)
	balanceIca3, err := s.Osmosis().GetBalance(ctx, icaAddresses[2], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(BondAmount/3, balanceIca3)

	t.Log("Query trapped errors for each one of the primitives")
	trappedErrors := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})

	t.Log("Parsing trapped errors to obtain seq number and channel id and checking length of each is 1")
	seqError1, channelIdError1 := helpers.ParseTrappedError(trappedErrors[0])
	seqError2, channelIdError2 := helpers.ParseTrappedError(trappedErrors[1])
	seqError3, channelIdError3 := helpers.ParseTrappedError(trappedErrors[2])
	s.Require().Equal(1, len(trappedErrors[0]))
	s.Require().Equal(1, len(trappedErrors[1]))
	s.Require().Equal(1, len(trappedErrors[2]))

	t.Log("Execute retry endpoints against each one of the primitives to enqueue previously failed join pools")
	s.executeRetry(
		ctx,
		acc2,
		[]string{lpAddresses[0], lpAddresses[1], lpAddresses[2]},
		[]uint64{seqError1, seqError2, seqError3},
		[]string{channelIdError1, channelIdError2, channelIdError3},
	)

	t.Log("Query again trapped errors for each one of the primitives")
	trappedErrorsAfter := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	s.Require().Equal(0, len(trappedErrorsAfter[0]))
	s.Require().Equal(0, len(trappedErrorsAfter[1]))
	s.Require().Equal(0, len(trappedErrorsAfter[2]))

	t.Log("Execute second bond transaction, this should work and also trigger the join_pool we enqueued previously via retry endpoint")
	s.executeBond(ctx, acc2, basicVaultAddress, 10_000_000)
	t.Log("Execute third clear cache to perform the joinPool on the osmosis side")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Query again trapped errors for each one of the primitives")
	trappedErrorsAfterSecondBond := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[0]))
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[1]))
	s.Require().Equal(0, len(trappedErrorsAfterSecondBond[2]))

	t.Log("Check that the user shares balance is ~20 as both join pools should have worked")
	balanceAfter := s.getUserSharesBalance(ctx, acc2, basicVaultAddress)
	s.Require().Equal(BondAmount*2-1-1, balanceAfter)

	t.Log("Check uOSMO balance of the primitives looking for ~0 on each one of them as they should be emptied")
	balanceIca1After, err := s.Osmosis().GetBalance(ctx, icaAddresses[0], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca1After)
	balanceIca2After, err := s.Osmosis().GetBalance(ctx, icaAddresses[1], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca2After)
	balanceIca3After, err := s.Osmosis().GetBalance(ctx, icaAddresses[2], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca3After)
}

func (s *WasmdTestSuite) TestLpStrategyContract_ExitPoolRetry() {
	t := s.T()
	ctx := context.Background()
	basicVaultAddress, lpAddresses := s.deployContracts(ctx, []map[string]any{init4, init5, init6})

	acc := s.createUserAndCheckBalances(ctx, 10_000_000)

	t.Log("Execute first bond transaction, this should work")
	s.executeBond(ctx, acc, basicVaultAddress, 10_000_000)

	t.Log("Execute first clear cache to perform the joinPool on the osmosis side")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Check that the user shares balance is ~10 as the joinPool should have worked")
	balance := s.getUserSharesBalance(ctx, acc, basicVaultAddress)
	s.Require().True(int64(9999999) == balance)

	t.Log("Get ICA addresses for each one of the primitive")
	icaAddresses := s.getPrimitiveIcaAddresses(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})

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

	t.Log("Query trapped errors for each primitive & check length should be 0")
	trappedErrors := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	s.Require().Equal(0, len(trappedErrors[0]))
	s.Require().Equal(0, len(trappedErrors[1]))
	s.Require().Equal(0, len(trappedErrors[2]))

	t.Log("Execute unbond, this should work")
	s.executeUnbond(ctx, acc, basicVaultAddress)

	t.Log("Execute second clear cache to perform the begin unlocking on osmosis")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Execute claim before ICA/ICQ. Claim should fail due to slippage")
	s.executeClaim(ctx, acc, basicVaultAddress)

	t.Log("Execute third clear cache")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Query trapped errors for each one of the primitives")
	trappedErrorsAfterClaim := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})

	t.Log("Parsing trapped errors to obtain seq number and channel id and checking length of each is 1")
	seqError1Claim, channelIdError1Claim := helpers.ParseTrappedError(trappedErrorsAfterClaim[0])
	seqError2Claim, channelIdError2Claim := helpers.ParseTrappedError(trappedErrorsAfterClaim[1])
	seqError3Claim, channelIdError3Claim := helpers.ParseTrappedError(trappedErrorsAfterClaim[2])
	s.Require().Equal(1, len(trappedErrorsAfterClaim[0]))
	s.Require().Equal(1, len(trappedErrorsAfterClaim[1]))
	s.Require().Equal(1, len(trappedErrorsAfterClaim[2]))

	t.Log("Execute retry endpoints against all primitives")
	s.executeRetry(
		ctx,
		acc,
		[]string{lpAddresses[0], lpAddresses[1], lpAddresses[2]},
		[]uint64{seqError1Claim, seqError2Claim, seqError3Claim},
		[]string{channelIdError1Claim, channelIdError2Claim, channelIdError3Claim},
	)

	t.Log("Query again trapped errors for each one of the primitives")
	trappedErrorsAfter := s.getTrappedErrors(ctx, []string{lpAddresses[0], lpAddresses[1], lpAddresses[2]})
	s.Require().Equal(0, len(trappedErrorsAfter[0]))
	s.Require().Equal(0, len(trappedErrorsAfter[1]))
	s.Require().Equal(0, len(trappedErrorsAfter[2]))

	t.Log("Fund the Osmosis pools to increase pool assets amount and reduce slippage for next retry")
	// Preparing array fo payloads to joinPools, those are magic numbers based on the test's values so any change to initial setup will cause a fail here
	poolIds := []string{"4", "5", "6"}
	maxAmountsIn := []string{"3876858349171stake1,6461430323493uosmo", "6461430323493uosmo,3876858349171usdc", "3876858349171fakestake,6461430323493uosmo"}
	sharesAmountOut := []string{"99999900000000000000000000", "99999900000000000000000000", "99999900000000000000000000"}
	s.JoinPools(ctx, poolIds, maxAmountsIn, sharesAmountOut)

	t.Log("Execute fourth clear cache to perform the exit pool on osmosis")
	s.executeClearCache(ctx, basicVaultAddress)

	t.Log("Check that the user shares balance is ~5 shares")
	balanceAfter := s.getUserSharesBalance(ctx, acc, basicVaultAddress)
	s.Require().Equal(BondAmount/2-1, balanceAfter)

	t.Log("Check uOSMO balance of the primitives looking for ~0 on each one of them as they should be emptied")
	balanceIca1After, err := s.Osmosis().GetBalance(ctx, icaAddresses[0], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca1After)
	balanceIca2After, err := s.Osmosis().GetBalance(ctx, icaAddresses[1], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca2After)
	balanceIca3After, err := s.Osmosis().GetBalance(ctx, icaAddresses[2], "uosmo")
	s.Require().NoError(err)
	s.Require().Equal(int64(0), balanceIca3After)

	t.Log("Check uOSMO balance of the primitives looking for ~0 on each one of them as they should be emptied")
	balanceVault, err := s.Quasar().GetBalance(ctx, basicVaultAddress, s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(int64(1), balanceVault)

	t.Log("Check uOSMO balance of the user on their wallet. Should be greater than 15")
	userBalance, err := s.Quasar().GetBalance(ctx, acc.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().True(userBalance > int64(14_999_999))
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

	t.Log("Wait 3 blocks on quasar and osmosis to settle up ICA packet transfer and the IBC transfer (bond)")
	err := testutil.WaitForBlocks(ctx, 3, s.Quasar(), s.Osmosis())
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
	t.Log("Wait 3 block on quasar and osmosis to settle up ICA packet claim and the IBC transfer (claim)")
	err := testutil.WaitForBlocks(ctx, 3, s.Quasar(), s.Osmosis())
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

func (s *WasmdTestSuite) executeRetry(ctx context.Context, acc *ibc.Wallet, lpAddresses []string, seqs []uint64, chans []string) {
	for i := range seqs {
		s.ExecuteContract(
			ctx,
			s.Quasar(),
			acc.KeyName,
			lpAddresses[i],
			sdk.NewCoins(), // empty amount
			map[string]any{"retry": map[string]any{
				"seq":     seqs[i],
				"channel": chans[i],
			}},
			nil,
		)
	}
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
