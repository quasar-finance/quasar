package suite

import (
	"encoding/hex"
	"encoding/json"
	"strings"

	dockerclient "github.com/docker/docker/client"
	"github.com/gogo/protobuf/proto"
	"github.com/pkg/errors"
	ibctest "github.com/strangelove-ventures/ibctest/v5"
	"github.com/strangelove-ventures/ibctest/v5/chain/cosmos"
	"github.com/strangelove-ventures/ibctest/v5/ibc"
	"github.com/strangelove-ventures/ibctest/v5/relayer/rly"
	"github.com/strangelove-ventures/ibctest/v5/test"
	"github.com/strangelove-ventures/ibctest/v5/testreporter"
	"github.com/stretchr/testify/suite"
	"go.uber.org/zap"
	"golang.org/x/net/context"
	"google.golang.org/grpc"

	sdk "github.com/cosmos/cosmos-sdk/types"
	connectiontypes "github.com/cosmos/ibc-go/v5/modules/core/03-connection/types"
	ibctenderminttypes "github.com/cosmos/ibc-go/v5/modules/light-clients/07-tendermint/types"
)

type E2ETestSuite struct {
	suite.Suite

	quasar      *cosmos.CosmosChain
	cosmos      *cosmos.CosmosChain
	osmosis     *cosmos.CosmosChain
	grpcClients map[ibc.Chain]*grpc.ClientConn

	relayer *rly.CosmosRelayer
	paths   map[string]path
	rep     *testreporter.Reporter
	erep    *testreporter.RelayerExecReporter

	dockerClient *dockerclient.Client
	networkID    string

	logger *zap.Logger
}

type path struct {
	chain1 ibc.Chain
	chain2 ibc.Chain
}

// Quasar returns the quasar chain instance. It fails the test if suite haven't been built with quasar chain.
func (s *E2ETestSuite) Quasar() *cosmos.CosmosChain {
	s.Require().NotNil(s.quasar)
	return s.quasar
}

// Cosmos returns the cosmos chain instance. It fails the test if suite haven't been built with cosmos chain.
func (s *E2ETestSuite) Cosmos() *cosmos.CosmosChain {
	s.Require().NotNil(s.cosmos)
	return s.cosmos
}

// Osmosis returns the osmosis chain instance. It fails the test if suite haven't been built with osmosis chain.
func (s *E2ETestSuite) Osmosis() *cosmos.CosmosChain {
	s.Require().NotNil(s.osmosis)
	return s.osmosis
}

// GetPairConnections returns all the available ibc connections between chain1 and chain2.
func (s E2ETestSuite) GetConnectionsByPath(ctx context.Context, pathName string) []*connectiontypes.IdentifiedConnection {
	path := s.getPath(pathName)
	clientIDs := s.GetClientIDsByPath(ctx, pathName)
	conns := s.QueryConnections(ctx, path.chain1)

	var pairConns []*connectiontypes.IdentifiedConnection
	for _, conn := range conns {
		for _, clientID := range clientIDs {
			if clientID == conn.ClientId {
				pairConns = append(pairConns, conn)
			}
		}
	}
	return pairConns
}

// GetClientIDsByPath returns array fo client ids for the given pathName (common client ids between two chains)
func (s E2ETestSuite) GetClientIDsByPath(ctx context.Context, pathName string) []string {
	path := s.getPath(pathName)
	clients := s.QueryClients(ctx, path.chain1)

	var clientIDs []string
	for _, client := range clients {
		// TODO: Investigate why clienttypes.UnpackClientState doesn't work
		var tendermintState ibctenderminttypes.ClientState
		err := proto.Unmarshal(client.ClientState.Value, &tendermintState)
		s.Require().NoError(err)

		if tendermintState.ChainId == path.chain2.Config().ChainID {
			clientIDs = append(clientIDs, client.ClientId)
		}
	}
	return clientIDs
}

func (s E2ETestSuite) getPath(pathName string) path {
	path, ok := s.paths[pathName]
	s.Require().Truef(ok, "could not find the path with name %s", path)
	return path
}

// CreateUserAndFund creates a user with the given amount of native tokens on chain.
func (s *E2ETestSuite) CreateUserAndFund(ctx context.Context, chain ibc.Chain, amount int64) *ibc.Wallet {
	user := ibctest.GetAndFundTestUsers(s.T(), ctx, strings.ReplaceAll(s.T().Name(), " ", "-"), amount, chain)[0]

	// Wait a few blocks
	err := test.WaitForBlocks(ctx, 5, chain)
	s.Require().NoError(err)
	return user
}

// ExecQuery executes the q command on the given chain and unmarshals the json stdout to resp.
func (s *E2ETestSuite) ExecQuery(ctx context.Context, chain *cosmos.CosmosChain, resp any, cmd ...string) {
	tn := GetFullNode(chain)
	stdout, stderr, err := tn.ExecQuery(ctx, cmd...)
	s.Require().NoError(err)
	s.Require().Empty(string(stderr))

	err = json.Unmarshal(stdout, &resp)
	s.Require().NoError(err)
}

// ExecTx executes the tx command on the given chain and returns the hex encoded txhash if successful.
func (s *E2ETestSuite) ExecTx(ctx context.Context, chain *cosmos.CosmosChain, keyName string, cmd ...string) string {
	tn := GetFullNode(chain)
	txhash, err := tn.ExecTx(ctx, keyName, cmd...)
	s.Require().NoError(err)

	return txhash
}

// AssertSuccessfulResultTx find the tx from txhash and verifies that the ResultTx of tx is successful tx and unmarshals the response.
func (s *E2ETestSuite) AssertSuccessfulResultTx(ctx context.Context, chain *cosmos.CosmosChain, txhash string, resp proto.Message) {
	tn := GetFullNode(chain)

	txhashBytes, err := hex.DecodeString(txhash)
	s.Require().NoError(err)
	res, err := tn.Client.Tx(ctx, txhashBytes, false)
	s.Require().NoErrorf(err, "failed to find tx result %s", txhash)
	s.Require().Zerof(res.TxResult.Code, "tx has non-zero code (%d) with log: %s", res.TxResult.Code, res.TxResult.Log)

	// Only unmarshal result if user wants to
	if resp != nil {
		err = unmarshalTxResult(res.TxResult.Data, resp)
		s.Require().NoError(err)
	}
}

func unmarshalTxResult(bz []byte, ptr proto.Message) error {
	var msgData sdk.TxMsgData
	err := proto.Unmarshal(bz, &msgData)
	if err != nil {
		return errors.Wrap(err, "could not unmarshal bz to sdk.TxMsgData")
	}
	switch {
	case len(msgData.Data) == 1:
		err = proto.Unmarshal(msgData.Data[0].Data, ptr)
	case len(msgData.MsgResponses) == 1:
		err = proto.Unmarshal(msgData.MsgResponses[0].Value, ptr)
	default:
		return errors.Errorf("invalid sdk.TxMsgData len(msgData.Data) = %v, len(msgData.MsgResponses) = %v", len(msgData.Data), len(msgData.MsgResponses))
	}
	return err
}

func GetFullNode(chain *cosmos.CosmosChain) *cosmos.ChainNode {
	if len(chain.FullNodes) > 0 {
		// use first full node
		return chain.FullNodes[0]
	}
	// use first validator
	return chain.Validators[0]
}

// CreateChannel creates a channel between two chains.
func (s *E2ETestSuite) CreateChannel(
	ctx context.Context,
	pathName string,
	srcPort string,
	dstPort string,
	order ibc.Order,
	version string,
) {
	err := s.relayer.CreateChannel(ctx, s.erep, pathName, ibc.CreateChannelOptions{
		SourcePortName: srcPort,
		DestPortName:   dstPort,
		Order:          order,
		Version:        version,
	})
	s.Require().NoError(err)
}
