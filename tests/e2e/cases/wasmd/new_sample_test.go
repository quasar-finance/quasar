package wasmd

import (
	"context"
	"encoding/json"
	transfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	"os"
	"strconv"
	"testing"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	testCasesHelper "github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
	testSuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/stretchr/testify/require"
	"github.com/stretchr/testify/suite"
	"go.uber.org/zap"
)

type TestE2eTestBuilderSuite struct {
	*testSuite.E2eTestBuilder
	suite.Suite
}

func TestE2eTestBuilder(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testSuite.NewE2eTestBuilder(t)

	// add quasar chain and genesis tokens to initialise a treasury account with
	genesisTokensQuasar := sdk.NewCoins().Add(sdk.NewInt64Coin(testSuite.QuasarChain.Denom, 100_000_000_000_000_000))
	b.AddChain(testSuite.QuasarChain, genesisTokensQuasar, 1, 0, true)

	// add osmosis chain and genesis tokens to initialise a treasury account with
	genesisTokensOsmosis := sdk.NewCoins().Add(sdk.NewInt64Coin(testSuite.OsmosisChain.Denom, 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("fakestake", 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("stake1", 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("usdc", 100_000_000_000_000_000))
	b.AddChain(testSuite.OsmosisChain, genesisTokensOsmosis, 1, 0, false)

	quasar, found := b.GetChain("quasar")
	require.True(t, found)

	osmosis, found := b.GetChain("osmosis")
	require.True(t, found)

	b.AddRelayer(quasar.Chain, osmosis.Chain, b.Relayer, testSuite.Quasar2OsmosisPath, ibc.CreateChannelOptions{}, ibc.CreateClientOptions{})
	b.AutomatedRelay()

	s := &TestE2eTestBuilderSuite{
		E2eTestBuilder: b.Build(),
	}
	suite.Run(t, s)
}

func (s *TestE2eTestBuilderSuite) TestBonds() {
	ctx := context.Background()

	// find Osmosis chain
	osmosis, found := s.GetChain("osmosis")
	s.Require().True(found)

	// create pool 1 on Osmosis chain
	poolBz, err := os.ReadFile(osmosisPool1Path)
	s.Require().NoError(err)

	tx, err := osmosis.ExecTx(
		ctx,
		[]string{
			"gamm", "create-pool",
			"--gas", "20000000",
		},
		osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		"pool.json",
		"--pool-file",
		poolBz,
		sdk.Coins{},
		s.Logger.With(
			zap.String("chain_id", osmosis.Chain.Config().ChainID),
			zap.String("test", testSuite.GetFullNode(osmosis.Chain).TestName)),
	)
	s.Require().NoError(err)

	err = osmosis.AssertSuccessfulResultTx(ctx, tx, nil)
	s.Require().NoError(err)

	// create pool2 on Osmosis chain
	poolBz, err = os.ReadFile(osmosisPool2Path)
	s.Require().NoError(err)

	tx, err = osmosis.ExecTx(
		ctx,
		[]string{
			"gamm", "create-pool",
			"--gas", "20000000",
		},
		osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		"pool.json",
		"--pool-file",
		poolBz,
		sdk.Coins{},
		s.Logger.With(
			zap.String("chain_id", osmosis.Chain.Config().ChainID),
			zap.String("test", testSuite.GetFullNode(osmosis.Chain).TestName)),
	)
	s.Require().NoError(err)

	err = osmosis.AssertSuccessfulResultTx(ctx, tx, nil)
	s.Require().NoError(err)

	// create pool3 on Osmosis chain
	poolBz, err = os.ReadFile(osmosisPool3Path)
	s.Require().NoError(err)

	tx, err = osmosis.ExecTx(
		ctx,
		[]string{
			"gamm", "create-pool",
			"--gas", "20000000",
		},
		osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		"pool.json",
		"--pool-file",
		poolBz,
		sdk.Coins{},
		s.Logger.With(
			zap.String("chain_id", osmosis.Chain.Config().ChainID),
			zap.String("test", testSuite.GetFullNode(osmosis.Chain).TestName)),
	)
	s.Require().NoError(err)

	err = osmosis.AssertSuccessfulResultTx(ctx, tx, nil)
	s.Require().NoError(err)

	// deploy contracts on Quasar chain
	// store contract code
	quasar, found := s.GetChain("quasar")
	s.Require().True(found)

	// store lp strategy contract code
	lpStrategyCodeID, err := testSuite.StoreContractCode(ctx, quasar.Chain, lpStrategyContractPath, quasar.ChainAccount[testSuite.AuthorityKeyName].KeyName, s.Logger)
	s.Require().NoError(err)

	// store rewards contract code
	rewardsContractCodeID, err := testSuite.StoreContractCode(ctx, quasar.Chain, vaultRewardsContractPath, quasar.ChainAccount[testSuite.AuthorityKeyName].KeyName, s.Logger)
	s.Require().NoError(err)

	// store basic vault contract code
	basicVaultContractCodeID, err := testSuite.StoreContractCode(ctx, quasar.Chain, basicVaultStrategyContractPath, quasar.ChainAccount[testSuite.AuthorityKeyName].KeyName, s.Logger)
	s.Require().NoError(err)

	// set new contracts in quasar chain
	newConrtacts := []*testSuite.Contract{
		testSuite.NewContract(init1, "primitive-1", lpStrategyCodeID),
		testSuite.NewContract(init2, "primitive-2", lpStrategyCodeID),
		testSuite.NewContract(init3, "primitive-3", lpStrategyCodeID),
	}
	err = quasar.SetContracts(newConrtacts)
	s.Require().NoError(err)

	// instantiate all the contracts
	for _, c := range newConrtacts {
		// instantiate primitives
		err = c.InstantiateContract(ctx, quasar.ChainAccount[testSuite.AuthorityKeyName], quasar.Chain, sdk.Coins{})
		s.Require().NoError(err)

		// create ICQ channel for primitives
		err = c.CreateICQChannel(ctx, s.Relayer, s.Erep)
		s.Require().NoError(err)

		// create ICA channel for primitives
		quasarConnections, err := s.Relayer.GetConnections(ctx, s.Erep, quasar.Chain.Config().ChainID)
		s.Require().NoError(err)

		err = c.CreateICAChannel(ctx, s.Relayer, s.Erep, quasarConnections[0].ID, quasarConnections[0].Counterparty.ConnectionId)
		s.Require().NoError(err)
	}

	// get all the primitives by their type
	prim1, err := quasar.FindContractByLabel("primitive-1")
	s.Require().NoError(err)

	prim2, err := quasar.FindContractByLabel("primitive-2")
	s.Require().NoError(err)

	prim3, err := quasar.FindContractByLabel("primitive-3")
	s.Require().NoError(err)

	vaultInit := map[string]any{
		"total_cap":                     "200000000000",
		"thesis":                        "e2e",
		"vault_rewards_code_id":         rewardsContractCodeID,
		"reward_token":                  map[string]any{"native": "uqsr"},
		"reward_distribution_schedules": []string{},
		"decimals":                      6,
		"symbol":                        "ORN",
		"min_withdrawal":                "1",
		"name":                          "ORION",
		"primitives": []map[string]any{
			{
				"address": prim1.GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init1,
				},
			},
			{
				"address": prim2.GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init2,
				},
			},
			{
				"address": prim3.GetContractAddress(),
				"weight":  "0.333333333333",
				"init": map[string]any{
					"l_p": init3,
				},
			},
		},
	}

	// add vault contract to quasar contracts
	vaultContracts := []*testSuite.Contract{
		testSuite.NewContract(vaultInit, "vault", basicVaultContractCodeID),
	}
	err = quasar.SetContracts(vaultContracts)
	s.Require().NoError(err)

	// initialize vaultContract
	for _, c := range vaultContracts {
		err = c.InstantiateContract(ctx, quasar.ChainAccount[testSuite.AuthorityKeyName], quasar.Chain, sdk.Coins{})
		s.Require().NoError(err)
	}

	// get vault contract by label
	vaultContract, err := quasar.FindContractByLabel("vault")
	s.Require().NoError(err)

	// set depositors for all primitives before executing test cases
	for _, c := range newConrtacts {
		_, err = c.ExecuteContract(ctx,
			quasar.Chain,
			map[string]any{"set_depositor": map[string]any{"depositor": vaultContract.GetContractAddress()}},
			nil,
			sdk.Coins{},
			quasar.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		)
		s.Require().NoError(err)
	}

	// transfer osmo to treasury account on quasar chain
	ibcTransferAmount := ibc.WalletAmount{
		Address: quasar.ChainAccount[testSuite.AuthorityKeyName].Address,
		Denom:   "uosmo",
		Amount:  100_000_000_000_000,
	}
	transfer, err := osmosis.Chain.SendIBCTransfer(ctx, "channel-0", osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName, ibcTransferAmount, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	bondUser, err := quasar.CreateUserAndFund(ctx, "bondUser", 10000000)
	s.Require().NoError(err)

	// get all the channels on quassr
	quasarChannels, err := s.Relayer.GetChannels(ctx, s.Erep, quasar.Chain.Config().ChainID)
	s.Require().NoError(err)

	// determine osmo denom in quasar from the above channels
	osmosisDenomInQuasar := transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(quasarChannels[0].PortID, quasarChannels[0].ChannelID, osmosis.Chain.Config().Denom)).IBCDenom()

	// execute a bank send from treasury to new bond account
	tx, err = quasar.ExecTx(
		ctx,
		[]string{
			"bank", "send",
			quasar.ChainAccount[testSuite.AuthorityKeyName].Address,
			bondUser.Bech32Address(quasar.Chain.Config().Bech32Prefix),
			"1000000000" + osmosisDenomInQuasar,
			"--gas", "20000000",
		},
		quasar.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		"",
		"",
		nil,
		sdk.Coins{},
		s.Logger.With(
			zap.String("chain_id", osmosis.Chain.Config().ChainID),
			zap.String("test", testSuite.GetFullNode(osmosis.Chain).TestName)),
	)
	s.Require().NoError(err)
	err = quasar.AssertSuccessfulResultTx(ctx, tx, nil)
	s.Require().NoError(err)

	// generate test cases
	testCases, err := testCasesHelper.GenerateTestCases(100120, 5, 1, 200000000)
	s.Require().NoError(err)

	for _, tc := range testCases {
		// inputs
		var Result testSuite.ContractBalanceData

		tc.Input.Account = *bondUser
		tc.Input.PreTxnInputCommand = []string{
			"wasm", "execute",
			vaultContract.GetContractAddress(),
		}
		tc.Input.PostTxnInputCommand = []string{
			"--gas", "20000000",
		}

		// outputs
		queryArgs := map[string]any{
			"balance": map[string]any{
				"address": bondUser.Bech32Address(quasar.Chain.Config().Bech32Prefix),
			},
		}
		queryArgsBz, err := json.Marshal(queryArgs)
		s.Require().NoError(err)

		tc.Output.Result = &Result
		tc.Output.PreQueryCommand = []string{
			"wasm", "contract-state", "smart",
			vaultContract.GetContractAddress(),
		}
		tc.Output.QueryCommand = queryArgsBz
		tc.Output.PostQueryCommand = []string{
			"--output", "json",
		}
		tc.Output.OperationOnResult = func() bool {
			balance, err := strconv.ParseInt(Result.Data.Balance, 10, 64)
			s.Require().NoError(err)

			if balance > 10000 {
				return true
			} else {
				return false
			}
		}
	}

	firstCase := testCases[0:1]
	otherCases := testCases[1 : len(testCases)-1]

	err = firstCase.ExecuteCases(quasar.Chain, ctx)
	s.Require().NoError(err)

	// wait for sometime before doing a second bond
	// todo : remove this and add a function that periodically runs any actions in parallel (like clear cache or clear packets)
	time.Sleep(time.Second * 20)

	err = otherCases.ExecuteCases(quasar.Chain, ctx)
	s.Require().NoError(err)
}
