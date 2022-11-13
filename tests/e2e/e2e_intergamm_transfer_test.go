package e2e

import (
	"context"
	"testing"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types/v1beta1"
	transfertypes "github.com/cosmos/ibc-go/v5/modules/apps/transfer/types"
	connectiontypes "github.com/cosmos/ibc-go/v5/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	testconfig "github.com/quasarlabs/quasarnode/tests/e2e/config"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/strangelove-ventures/ibctest/v5/ibc"
	"github.com/strangelove-ventures/ibctest/v5/test"
	"github.com/stretchr/testify/suite"
)

const (
	StartingTokenAmount int64  = 100_000_000
	IBCTransferAmount   int64  = 10_000
	ProposalTitle       string = "title"
	ProposalDescription string = "description"
)

func TestIntergammTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseCosmos()
	b.UseOsmosis()
	b.Link(b.Quasar(), b.Cosmos(), testconfig.Quasar2CosmosPath)
	b.Link(b.Cosmos(), b.Osmosis(), testconfig.Cosmos2OsmosisPath)
	b.Link(b.Quasar(), b.Osmosis(), testconfig.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &IntergammTestSuite{E2ETestSuite: b.Build()}
	suite.Run(t, s)
}

type IntergammTestSuite struct {
	*testsuite.E2ETestSuite

	Quasar2CosmosConn  *connectiontypes.IdentifiedConnection
	Cosmos2OsmosisConn *connectiontypes.IdentifiedConnection
	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection

	Quasar2CosmosTransferChan  *channeltypes.IdentifiedChannel
	Cosmos2OsmosisTransferChan *channeltypes.IdentifiedChannel
	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel

	CosmosDenomInQuasar  string
	OsmosisDenomInQuasar string
	CosmosDenomInOsmosis string
	QuasarDenomInOsmosis string
}

func (s *IntergammTestSuite) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	t.Log("Wait for chains to settle up the ibc connection states")
	err := test.WaitForBlocks(ctx, 10, s.Quasar(), s.Cosmos(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2CosmosConn = s.GetConnectionsByPath(ctx, testconfig.Quasar2CosmosPath)[0]
	s.Cosmos2OsmosisConn = s.GetConnectionsByPath(ctx, testconfig.Cosmos2OsmosisPath)[0]
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testconfig.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2CosmosTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2CosmosConn.Id)[0]
	s.Cosmos2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Cosmos(), s.Cosmos2OsmosisConn.Id)[0]
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]

	// Generate the ibc denom of native tokens in other chains
	s.CosmosDenomInQuasar = ibcDenomFromChannel(s.Quasar2CosmosTransferChan, s.Cosmos().Config().Denom)
	s.OsmosisDenomInQuasar = ibcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.CosmosDenomInOsmosis = ibcDenomFromChannelCounterparty(s.Cosmos2OsmosisTransferChan, s.Cosmos().Config().Denom)
	s.QuasarDenomInOsmosis = ibcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// Setup an account of quasar as the proposer to propose
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Below we execute a couple of param-change governance proposals against quasar chain to config intergamm module
	// with generated connections/channels info.

	t.Run("RegisterQuasarDenomToNativeZoneIDMap", func(t *testing.T) {
		proposal := testsuite.NewParamChangeProposalJSON(
			ProposalTitle,
			ProposalDescription,
			intergammtypes.StoreKey,
			intergammtypes.KeyQuasarDenomToNativeZoneIdMap,
			map[string]string{
				s.Quasar().Config().Denom: s.Quasar().Config().Name,
				s.CosmosDenomInQuasar:     s.Cosmos().Config().Name,
				s.OsmosisDenomInQuasar:    s.Osmosis().Config().Name,
			},
			sdk.NewCoins(sdk.NewCoin(s.Quasar().Config().Denom, govtypes.DefaultMinDepositTokens)),
		)
		s.ExecParamChangeProposal(ctx, s.Quasar(), quasarAccount.KeyName, proposal)
	})

	t.Run("RegisterOsmosisDenomToQuasarDenomMap", func(t *testing.T) {
		proposal := testsuite.NewParamChangeProposalJSON(
			ProposalTitle,
			ProposalDescription,
			intergammtypes.StoreKey,
			intergammtypes.KeyOsmosisDenomToQuasarDenomMap,
			map[string]string{
				s.QuasarDenomInOsmosis:     s.Quasar().Config().Denom,
				s.CosmosDenomInOsmosis:     s.CosmosDenomInQuasar,
				s.Osmosis().Config().Denom: s.OsmosisDenomInQuasar,
			},
			sdk.NewCoins(sdk.NewCoin(s.Quasar().Config().Denom, govtypes.DefaultMinDepositTokens)),
		)
		s.ExecParamChangeProposal(ctx, s.Quasar(), quasarAccount.KeyName, proposal)
	})

	t.Run("RegisterCompleteZoneInfoMap", func(t *testing.T) {
		proposal := testsuite.NewParamChangeProposalJSON(
			ProposalTitle,
			ProposalDescription,
			intergammtypes.StoreKey,
			intergammtypes.KeyCompleteZoneInfoMap,
			map[string]any{
				"osmosis": map[string]any{
					"zone_route_info": map[string]string{
						"zone_id":                    s.Quasar().Config().Name,
						"chain_id":                   s.Quasar().Config().ChainID,
						"counterparty_zone_id":       s.Osmosis().Config().Name,
						"counterparty_chain_id":      s.Osmosis().Config().ChainID,
						"connection_id":              s.Quasar2OsmosisConn.Id,
						"port_id":                    s.Quasar2OsmosisTransferChan.PortId,
						"channel_id":                 s.Quasar2OsmosisTransferChan.ChannelId,
						"counterparty_connection_id": s.Quasar2OsmosisConn.Counterparty.ConnectionId,
						"counterparty_port_id":       s.Quasar2OsmosisTransferChan.Counterparty.PortId,
						"counterparty_channel_id":    s.Quasar2OsmosisTransferChan.Counterparty.ChannelId,
						"counterparty_version":       s.Quasar2OsmosisTransferChan.Version,
					},
					"next_zone_route_map": map[string]string{},
				},
				"cosmos": map[string]any{
					"zone_route_info": map[string]string{
						"zone_id":                    s.Quasar().Config().Name,
						"chain_id":                   s.Quasar().Config().ChainID,
						"counterparty_zone_id":       s.Cosmos().Config().Name,
						"counterparty_chain_id":      s.Cosmos().Config().ChainID,
						"connection_id":              s.Quasar2CosmosConn.Id,
						"port_id":                    s.Quasar2CosmosTransferChan.PortId,
						"channel_id":                 s.Quasar2CosmosTransferChan.ChannelId,
						"counterparty_connection_id": s.Quasar2CosmosConn.Counterparty.ConnectionId,
						"counterparty_port_id":       s.Quasar2CosmosTransferChan.Counterparty.PortId,
						"counterparty_channel_id":    s.Quasar2CosmosTransferChan.Counterparty.ChannelId,
						"counterparty_version":       s.Quasar2CosmosTransferChan.Version,
					},
					"next_zone_route_map": map[string]any{
						"osmosis": map[string]string{
							"zone_id":                    s.Cosmos().Config().Name,
							"chain_id":                   s.Cosmos().Config().ChainID,
							"counterparty_zone_id":       s.Osmosis().Config().Name,
							"counterparty_chain_id":      s.Osmosis().Config().ChainID,
							"connection_id":              s.Cosmos2OsmosisConn.Id,
							"port_id":                    s.Cosmos2OsmosisTransferChan.PortId,
							"channel_id":                 s.Cosmos2OsmosisTransferChan.ChannelId,
							"counterparty_connection_id": s.Cosmos2OsmosisConn.Counterparty.ConnectionId,
							"counterparty_port_id":       s.Cosmos2OsmosisTransferChan.Counterparty.PortId,
							"counterparty_channel_id":    s.Cosmos2OsmosisTransferChan.Counterparty.ChannelId,
							"counterparty_version":       s.Cosmos2OsmosisTransferChan.Version,
						},
					},
				},
			},
			sdk.NewCoins(sdk.NewCoin(s.Quasar().Config().Denom, govtypes.DefaultMinDepositTokens)),
		)
		s.ExecParamChangeProposal(ctx, s.Quasar(), quasarAccount.KeyName, proposal)
	})
}

// ibcDenomFromChannel returns ibc denom according to the given channel port, id and denom
// this function generates the ibc denom for the main direction as an example if there is a channel from
// chain1 <-> chain2 knowing that chain1 has a denom named denom1 this function will return the ibc denom of denom1 in chain2.
func ibcDenomFromChannel(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.PortId, ch.ChannelId, baseDenom)).IBCDenom()
}

// ibcDenomFromChannelCounterparty does same as ibcDenomFromChannel but in reverse so it generates
// the ibc denom of denom2 from chain2 (counterparty chain) in chain1
func ibcDenomFromChannelCounterparty(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.Counterparty.PortId, ch.Counterparty.ChannelId, baseDenom)).IBCDenom()
}

// TestICATransfer_SuccessfulTransfer tests intergamm ability to:
// 1. Register an ICA account on behalf of given native account on the given zone
// 2. Transfer some specific amount of quasar native tokens to created ICA account
// 3. Transmit a transfer command to ICA account to transfer back all the transferred tokens.
func (s *IntergammTestSuite) TestICATransfer_SuccessfulTransfer() {
	t := s.T()
	t.Parallel()
	ctx := context.Background()

	// Setup an account of quasar
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// We need this variable to store address of the quasar ICA account in osmosis chain
	var qusasrICAAddessonOsmosis string

	t.Run("RegisterICAAccountOnOsmosis", func(t *testing.T) {
		txhash := s.ExecTx(ctx, s.Quasar(), quasarAccount.KeyName, "intergamm", "register-ica-on-zone", "osmosis")
		s.AssertSuccessfulResultTx(ctx, s.Quasar(), txhash, nil)

		t.Log("Wait for quasar and osmosis chain to settle up the ICA account creation")
		err := test.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
		s.Require().NoError(err)

		// Query the ICA account address
		var icaAddressResponse intergammtypes.QueryICAAddressOnZoneResponse
		s.ExecQuery(
			ctx,
			s.Quasar(),
			&icaAddressResponse,
			"intergamm",
			"ica-address-on-zone",
			quasarAccount.Bech32Address(s.Quasar().Config().Bech32Prefix),
			"osmosis",
		)
		qusasrICAAddessonOsmosis = icaAddressResponse.IcaAddress
	})

	t.Run("IBCTransferTokenFromQuasarICAAccountOnOsmosis", func(t *testing.T) {
		txhash := s.ExecTx(
			ctx,
			s.Quasar(),
			quasarAccount.KeyName,
			"intergamm",
			"send-token-to-ica",
			"osmosis",
			sdk.NewInt64Coin(s.Quasar().Config().Denom, IBCTransferAmount).String(),
		)
		s.AssertSuccessfulResultTx(ctx, s.Quasar(), txhash, nil)

		t.Log("Wait for quasar and osmosis to settle up the ibc transfer")
		err := test.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
		s.Require().NoError(err)

		// Now ICA account balance should be equal to transferred amount
		balance, err := s.Osmosis().GetBalance(ctx, qusasrICAAddessonOsmosis, s.QuasarDenomInOsmosis)
		s.Require().NoError(err)
		s.Require().EqualValues(IBCTransferAmount, balance)
	})

	t.Run("TransmitIBCTransferCommandToICAAccountOnOsmosis", func(t *testing.T) {
		txhash := s.ExecTx(
			ctx,
			s.Quasar(),
			quasarAccount.KeyName,
			"intergamm",
			"transmit-ica-transfer",
			quasarAccount.Bech32Address(s.Quasar().Config().Bech32Prefix),
			sdk.NewInt64Coin(s.QuasarDenomInOsmosis, IBCTransferAmount).String(),
		)
		s.AssertSuccessfulResultTx(ctx, s.Quasar(), txhash, nil)

		t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
		err := test.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
		s.Require().NoError(err)

		// We transferred all the tokens back to quasar chain so ICA account's balance should be zero
		balance, err := s.Osmosis().GetBalance(ctx, qusasrICAAddessonOsmosis, s.QuasarDenomInOsmosis)
		s.Require().NoError(err)
		s.Require().Zero(balance)
	})
}

// TestForwardTransfer_SuccessfulTransfer tests intergamm ability to transfer uatoms from
// quasar to osmosis chain forwarded by cosmos-hub so that the ibc denom of uatom in osmosis
// become a one-hop ibc denom.
func (s *IntergammTestSuite) TestForwardTransfer_SuccessfulTransfer() {
	t := s.T()
	// TODO: Fix the problem with packet forwarder in gaia docker image
	t.Skip()
	t.Parallel()

	ctx := context.Background()

	// Setup an account of cosmos-hub
	cosmosAccount := s.CreateUserAndFund(ctx, s.Cosmos(), StartingTokenAmount)
	cosmosAddress := cosmosAccount.Bech32Address(s.Cosmos().Config().Bech32Prefix)
	// Setup an account of quasar
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)
	quasarAddress := quasarAccount.Bech32Address(s.Quasar().Config().Bech32Prefix)
	// Setup an account of osmosis
	osmosisAccount := s.CreateUserAndFund(ctx, s.Osmosis(), StartingTokenAmount)
	osmosisAddress := osmosisAccount.Bech32Address(s.Osmosis().Config().Bech32Prefix)

	t.Run("RegisterICAAccountOnCosmos", func(t *testing.T) {
		txhash := s.ExecTx(ctx, s.Quasar(), quasarAccount.KeyName, "intergamm", "register-ica-on-zone", "cosmos")
		s.AssertSuccessfulResultTx(ctx, s.Quasar(), txhash, nil)

		t.Log("Wait for quasar and cosmos chain to settle up the ICA account creation")
		err := test.WaitForBlocks(ctx, 5, s.Quasar(), s.Cosmos())
		s.Require().NoError(err)

		// Query the ICA account address
		s.ExecQuery(
			ctx,
			s.Quasar(),
			nil,
			"intergamm",
			"ica-address-on-zone",
			quasarAddress,
			"cosmos",
		)
	})

	t.Run("TransferAtomsFromCosmosToQuasar", func(t *testing.T) {
		// Default ibc transfer timeout
		ibcTransferTimeout := &ibc.IBCTimeout{
			NanoSeconds: uint64(time.Minute.Nanoseconds()),
		}

		tx, err := s.Cosmos().SendIBCTransfer(
			ctx,
			s.Quasar2CosmosTransferChan.Counterparty.ChannelId,
			cosmosAccount.KeyName,
			ibc.WalletAmount{
				Address: quasarAddress,
				Denom:   s.Cosmos().Config().Denom,
				Amount:  IBCTransferAmount,
			},
			ibcTransferTimeout)
		s.Require().NoError(err)
		s.Require().NotZero(tx.Packet.Sequence)

		t.Log("Wait for cosmos and quasar chains to settle up the ibc transfer")
		err = test.WaitForBlocks(ctx, 5, s.Cosmos(), s.Quasar())
		s.Require().NoError(err)

		cosmosBalance, err := s.Cosmos().GetBalance(ctx, cosmosAddress, s.Cosmos().Config().Denom)
		s.Require().NoError(err)
		s.Require().EqualValues(StartingTokenAmount-IBCTransferAmount, cosmosBalance)

		quasarBalance, err := s.Quasar().GetBalance(ctx, quasarAddress, s.CosmosDenomInQuasar)
		s.Require().NoError(err)
		s.Require().EqualValues(IBCTransferAmount, quasarBalance)
	})

	t.Run("ForwardTransferAtomsFromQuasarToOsmosis", func(t *testing.T) {
		txhash := s.ExecTx(
			ctx,
			s.Quasar(),
			quasarAccount.KeyName,
			"intergamm",
			"send-token",
			"osmosis",
			osmosisAddress,
			sdk.NewInt64Coin(s.CosmosDenomInQuasar, IBCTransferAmount).String(),
		)
		s.AssertSuccessfulResultTx(ctx, s.Quasar(), txhash, nil)

		t.Log("Wait for quasar, cosmos and osmosis to settle up the ibc transfer")
		err := test.WaitForBlocks(ctx, 10, s.Quasar(), s.Cosmos(), s.Osmosis())
		s.Require().NoError(err)

		quasarBalance, err := s.Quasar().GetBalance(ctx, quasarAddress, s.CosmosDenomInQuasar)
		s.Require().NoError(err)
		s.Require().Zero(quasarBalance)

		osmosisBalance, err := s.Osmosis().GetBalance(ctx, osmosisAddress, s.CosmosDenomInOsmosis)
		s.Require().NoError(err)
		s.Require().EqualValues(IBCTransferAmount, osmosisBalance)
	})
}
