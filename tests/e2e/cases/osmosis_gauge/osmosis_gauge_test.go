package osmosis_gauge

import (
	"context"
	"fmt"
	"github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"os"
	"strconv"
	"testing"
	"time"

	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	osmosisPool1Path        = "../_utils/sample_pool1.json"
	userFunds         int64 = int64(100_000_000_000)
	ibcTransferAmount int64 = int64(10_000_000_000)
)

func TestOsmosisGauge(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &OsmosisGauge{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type OsmosisGauge struct {
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	*testsuite.E2ETestSuite

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel

	QuasarDenomInOsmosis string
}

func (s *OsmosisGauge) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	// Wait for IBC connections to be established
	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]

	// Generate the ibc denom of native tokens in other chains
	s.QuasarDenomInOsmosis = helpers.IbcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// Send tokens to the respective account and create the required pools
	s.CreatePools(ctx)
}

func (s *OsmosisGauge) TestOsmosisGauge() {
	t := s.T()
	ctx := context.Background()

	t.Log("Create an user with fund on Quasar chain")
	userQuasar := s.CreateUserAndFund(ctx, s.Quasar(), userFunds)
	t.Log("Check user \"uqsr\" balance expecting to be the funded amount")
	userQuasarBalance, err := s.Quasar().GetBalance(ctx, userQuasar.Bech32Address(s.Quasar().Config().Bech32Prefix), s.Quasar().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(userFunds, userQuasarBalance)

	t.Log("Create an user with fund on Osmosis chain")
	userOsmosis := s.CreateUserAndFund(ctx, s.Osmosis(), userFunds)
	t.Log("Check user \"uosmo\" balance expecting to be the funded amount")
	userOsmosisBalance, err := s.Osmosis().GetBalance(ctx, userOsmosis.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(userFunds, userOsmosisBalance)

	t.Log("IBC Transfer of \"uqsr\" from Quasar to Osmosis chain account we use to create the gauge")
	amountQsr := ibc.WalletAmount{
		Address: userOsmosis.Bech32Address(s.Osmosis().Config().Bech32Prefix),
		Denom:   s.Quasar().Config().Denom,
		Amount:  ibcTransferAmount,
	}
	txQuasarToOsmosis, err := s.Quasar().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, userQuasar.KeyName, amountQsr, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(txQuasarToOsmosis.Validate())

	t.Log("Wait for quasar and osmosis block and relayer to relay IBC transfer")
	err = testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check Osmosis user \"uqsr\" balance after executing IBC transfer from Quasar")
	userOsmosisBalanceQsr, err := s.Osmosis().GetBalance(ctx, userOsmosis.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.QuasarDenomInOsmosis)
	s.Require().NoError(err)
	s.Require().Equal(ibcTransferAmount, userOsmosisBalanceQsr)

	t.Log("Check Liquidity Provider user \"uqsr\" balance before creating the Gauge expecting to be 0")
	lpBalanceQsr, err := s.Osmosis().GetBalance(ctx, s.E2EBuilder.OsmosisAccounts.Treasury.Address, s.QuasarDenomInOsmosis)
	s.Require().NoError(err)
	s.Require().Equal(int64(0), lpBalanceQsr)

	t.Log("Create the gauge for poolId 1 with duration of 1s, start time now for 1 epoch")
	nowTimestamp := time.Now().Unix()
	cmds := []string{"incentives", "create-gauge", "gamm/pool/1", fmt.Sprintf("%d%s", ibcTransferAmount/2, s.QuasarDenomInOsmosis),
		"--duration", "1h", "--start-time", strconv.FormatInt(nowTimestamp, 10), "--epochs", "1", // TODO duration doesnt work due to not enough time elapsed till here
		"--gas", "20000000",
	}
	txHashCreate := s.ExecTx(ctx, s.Osmosis(), userOsmosis.KeyName, cmds...)
	fmt.Println(txHashCreate)

	// TODO wait for 1 epoch (in blocks)
	err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	nowTimestamp2 := time.Now().Unix()
	fmt.Println(nowTimestamp2)

	// TODO check the uqsr balance of treasury to be ibcTransferAmount/2
	t.Log("Check Liquidity Provider user \"uqsr\" balance after creating the Gauge and time passed")
	lpBalanceQsr, err = s.Osmosis().GetBalance(ctx, s.E2EBuilder.OsmosisAccounts.Treasury.Address, s.QuasarDenomInOsmosis)
	s.Require().NoError(err)
	s.Require().Equal(ibcTransferAmount/2, lpBalanceQsr)

	// todo comment
	nowTimestamp = time.Now().Unix()
	cmds = []string{"incentives", "add-to-gauge", "1", fmt.Sprintf("%d%s", ibcTransferAmount/2, s.QuasarDenomInOsmosis),
		"--gas", "20000000", // TODO check this
	}
	txHashAdd := s.ExecTx(ctx, s.Osmosis(), userOsmosis.KeyName, cmds...)
	fmt.Println(txHashAdd)

	// TODO wait for other 1 epoch (in blocks)
	err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// check the uqsr balance of treasury to be ibcTransferAmount
	t.Log("Check Liquidity Provider user \"uqsr\" balance after adding a second amount to the Gauge and time passed")
	lpBalanceQsr, err = s.Osmosis().GetBalance(ctx, s.E2EBuilder.OsmosisAccounts.Treasury.Address, s.QuasarDenomInOsmosis)
	s.Require().NoError(err)
	s.Require().Equal(ibcTransferAmount/2, lpBalanceQsr)
}

func (s *OsmosisGauge) CreatePools(ctx context.Context) {
	// Read the pool details from os file
	poolBz, err := os.ReadFile(osmosisPool1Path)
	s.Require().NoError(err)
	s.CreatePoolsOnOsmosis(ctx, s.Osmosis(), s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, poolBz)
}
