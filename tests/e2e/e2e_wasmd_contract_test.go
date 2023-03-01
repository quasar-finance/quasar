package e2e

import (
	"context"
	"fmt"

	"os"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	transfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testconfig "github.com/quasarlabs/quasarnode/tests/e2e/config"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	StartingTokenAmount    int64  = 100_000_000
	IBCTransferAmount      int64  = 10_000
	ProposalTitle          string = "title"
	ProposalDescription    string = "description"
	lpStrategyContractPath        = "../../smart-contracts/artifacts/lp_strategy.wasm"
)

func TestWasmdTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(b.Quasar(), b.Osmosis(), testconfig.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &WasmdTestSuite{E2ETestSuite: b.Build()}
	suite.Run(t, s)
}

type WasmdTestSuite struct {
	*testsuite.E2ETestSuite

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel

	OsmosisDenomInQuasar string
	QuasarDenomInOsmosis string

	LpStrategyContractAddress string
}

func (s *WasmdTestSuite) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testconfig.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]

	// Generate the ibc denom of native tokens in other chains
	s.OsmosisDenomInQuasar = ibcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.QuasarDenomInOsmosis = ibcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// // Setup an account in quasar chain for contract deployment
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Deploy the lp strategy contract
	s.LpStrategyContractAddress = s.deployContract(ctx, quasarAccount, lpStrategyContractPath, "lp_strategy_test", map[string]any{
		"lock_period": "1209600",
		"pool_id":     1,
		"pool_denom":  "gamm/pool/1",
		"denom":       "uosmo",
	})
}

// deployAndInitContract stores the contract, initiates it and returns the contract address.
func (s *WasmdTestSuite) deployContract(ctx context.Context, acc *ibc.Wallet, filePath, label string, initArgs any) string {
	accAddress := acc.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Read the contract from os file
	contract, err := os.ReadFile(filePath)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), acc.KeyName, contract)

	res := s.InstantiateContract(ctx, s.Quasar(), acc.KeyName, codeID, label, accAddress, sdk.NewCoins(), initArgs)
	s.Require().NotEmpty(res.Address)

	return res.Address
}

// TestLpStrategyContract_SuccessfulDeposit tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *WasmdTestSuite) TestLpStrategyContract_SuccessfulDeposit() {
	t := s.T()
	t.Skip()
	ctx := context.Background()

	// Setup an account in quasar chain
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Create a channel between lp-strategy contract and osmosis
	s.CreateChannel(
		ctx,
		testconfig.Quasar2OsmosisPath,
		fmt.Sprintf("wasm.%s", s.LpStrategyContractAddress),
		"icahost",
		ibc.Ordered,
		fmt.Sprintf(
			`{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"%s","host_connection_id":"%s"}`,
			s.Quasar2OsmosisConn.Id,
			s.Quasar2OsmosisConn.Counterparty.ConnectionId,
		),
	)

	// Query contract channels and check if the channel is created
	query := map[string]any{
		"channels": struct{}{},
	}
	var result channelsResponse
	s.QuerySmartWasmContractState(ctx, s.Quasar(), s.LpStrategyContractAddress, query, &result)
	s.Require().Len(result.Channels, 1)
	s.Require().Equal("icahost", result.Channels[0].CounterpartyEndpoint.PortId)
	icaAddress := result.Channels[0].ChannelType.ICA.CounterpartyAddress
	// Check the ica address
	s.Require().NotEmpty(icaAddress)

	// Transfer 1000uqsr coins to ica address of contract through cosmos chain
	s.ExecuteContract(
		ctx,
		s.Quasar(),
		quasarAccount.KeyName,
		s.LpStrategyContractAddress,
		sdk.NewCoins(
			sdk.NewInt64Coin(s.Quasar().Config().Denom, 1000),
		),
		map[string]any{
			"transfer_join_lock": map[string]any{
				"channel":    s.Quasar2OsmosisTransferChan.ChannelId,
				"to_address": icaAddress,
			},
		}, nil)

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// ICA address should now have exactly 1000uqsr
	balance, err := s.Osmosis().GetBalance(ctx, icaAddress, s.QuasarDenomInOsmosis)
	s.Require().NoError(err)
	s.Require().EqualValues(1000, balance)
}

type channelsResponse struct {
	Channels []struct {
		Id                   string `json:"id"`
		ConnectionId         string `json:"connection_id"`
		CounterpartyEndpoint struct {
			ChannelId string `json:"channel_id"`
			PortId    string `json:"port_id"`
		} `json:"counterparty_endpoint"`
		ChannelType struct {
			ICA struct {
				CounterpartyAddress string `json:"counter_party_address"`
			} `json:"ica"`
		} `json:"channel_type"`
	} `json:"channels"`
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
