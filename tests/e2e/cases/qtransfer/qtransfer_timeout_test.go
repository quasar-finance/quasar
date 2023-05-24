package qtransfer

import (
	"context"
	"github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"testing"

	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	UserFundAmount    int64  = 1_002_000
	IBCTransferAmount int64  = 1_000_000
	IBCTransferMemo   string = "\"{ \"wasm\": { \"contract\": \"osmo1contractAddr\", \"msg\": { \"execute_IBC_receive\": \"raw_message_data\"}}}"
)

func TestQtransferTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &QtransferTestSuite{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type QtransferTestSuite struct {
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	*testsuite.E2ETestSuite

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection
	Osmosis2QuasarConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel
	Osmosis2QuasarTransferChan *channeltypes.IdentifiedChannel

	OsmosisDenomInQuasar string
	QuasarDenomInOsmosis string
}

func (s *QtransferTestSuite) SetupSuite() {
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
}

// TestQtransfer_Timeout
func (s *QtransferTestSuite) TestQtransfer_Timeout() {
	t := s.T()
	ctx := context.Background()

	t.Log("Create an user with fund on Quasar chain")
	user := s.CreateUserAndFund(ctx, s.Quasar(), UserFundAmount)

	t.Log("Check user balance before executing IBC transfer expecting to be the funded amount")
	userBalanceBefore, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.Quasar().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(UserFundAmount, userBalanceBefore)

	t.Log("Execute IBC Transfer from previously created user")
	amount := ibc.WalletAmount{
		Address: user.Bech32Address(s.Quasar().Config().Bech32Prefix),
		Denom:   s.Quasar().Config().Denom,
		Amount:  IBCTransferAmount,
	}
	ibcTimeout := ibc.IBCTimeout{NanoSeconds: 0, Height: 0} // TODO check if this 0-0 is right
	//options := ibc.TransferOptions{Timeout: &ibcTimeout, Memo: IBCTransferMemo}
	options := ibc.TransferOptions{Timeout: &ibcTimeout, Memo: "somethingHereAsPlainMessage"}
	tx, err := s.Quasar().SendIBCTransfer(ctx, s.Quasar2OsmosisTransferChan.ChannelId, user.KeyName, amount, options)
	s.Require().NoError(err)
	s.Require().NoError(tx.Validate())

	t.Log("Check user balance after executing IBC transfer expecting to be 0")
	userBalanceAfterTransfer, err := s.Quasar().GetBalance(ctx, user.Bech32Address(s.Quasar().Config().Bech32Prefix), s.Quasar().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(0, userBalanceAfterTransfer)

	t.Log("Wait for transfer packet to timeout")
	err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check user balance after packet timeout expecting to be the original funded amount")
	userBalanceAfterTimeout, err := s.Quasar().GetBalance(ctx, user.Address, s.Quasar().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(IBCTransferAmount, userBalanceAfterTimeout)
}
