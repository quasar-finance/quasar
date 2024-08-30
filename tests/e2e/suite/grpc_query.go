package suite

import (
	"context"
	"encoding/json"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	connectiontypes "github.com/cosmos/ibc-go/v8/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v8/modules/core/04-channel/types"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"google.golang.org/grpc"
)

// QueryProposal queries the governance proposal on the given chain with the given proposal ID.
func (s *E2ETestSuite) QueryProposal(ctx context.Context, chain *cosmos.CosmosChain, proposalID uint64) govtypes.Proposal {
	cc := s.GetGRPCClient(chain)
	qc := govtypes.NewQueryClient(cc)

	resp, err := qc.Proposal(ctx, &govtypes.QueryProposalRequest{
		ProposalId: proposalID,
	})
	s.Require().NoError(err)

	return resp.Proposal
}

// QueryClients queries the list of all ibc clients on the given chain.
func (s *E2ETestSuite) QueryClients(ctx context.Context, chain ibc.Chain) []clienttypes.IdentifiedClientState {
	cc := s.GetGRPCClient(chain)
	qc := clienttypes.NewQueryClient(cc)

	// TODO: Calculate pagination
	resp, err := qc.ClientStates(ctx, &clienttypes.QueryClientStatesRequest{})
	s.Require().NoError(err)

	return resp.ClientStates
}

// QueryConnections queries the list of all ibc connections on the given chain.
func (s *E2ETestSuite) QueryConnections(ctx context.Context, chain ibc.Chain) []*connectiontypes.IdentifiedConnection {
	cc := s.GetGRPCClient(chain)
	qc := connectiontypes.NewQueryClient(cc)

	// TODO: Calculate pagination
	resp, err := qc.Connections(ctx, &connectiontypes.QueryConnectionsRequest{})
	s.Require().NoError(err)

	return resp.Connections
}

// QueryConnectionChannels queries the list of all ibc channels on the given chain with specified connection.
func (s E2ETestSuite) QueryConnectionChannels(ctx context.Context, chain ibc.Chain, connection string) []*channeltypes.IdentifiedChannel {
	cc := s.GetGRPCClient(chain)
	qc := channeltypes.NewQueryClient(cc)

	// TODO: Calculate pagination
	resp, err := qc.ConnectionChannels(ctx, &channeltypes.QueryConnectionChannelsRequest{
		Connection: connection,
	})
	s.Require().NoError(err)

	return resp.Channels
}

// QueryWasmCodes returns a list of all wasm codes stored in the chain.
func (s E2ETestSuite) QueryWasmCodes(ctx context.Context, chain ibc.Chain) []wasmtypes.CodeInfoResponse {
	cc := s.GetGRPCClient(chain)
	qc := wasmtypes.NewQueryClient(cc)
	resp, err := qc.Codes(ctx, &wasmtypes.QueryCodesRequest{})
	s.Require().NoError(err)

	return resp.CodeInfos
}

// QuerySmartWasmContractState queries the smart contract state on the given chain.
func (s E2ETestSuite) QuerySmartWasmContractState(ctx context.Context, chain ibc.Chain, contractAddr string, query, result any) {
	queryBz, err := json.Marshal(query)
	s.Require().NoError(err)

	cc := s.GetGRPCClient(chain)
	qc := wasmtypes.NewQueryClient(cc)
	resp, err := qc.SmartContractState(ctx, &wasmtypes.QuerySmartContractStateRequest{
		Address:   contractAddr,
		QueryData: queryBz,
	})
	s.Require().NoError(err)

	err = json.Unmarshal(resp.Data, result)
	s.Require().NoError(err)
}

// GetGRPCClient returns a persistent grpc connection to the requested chain.
// Note that caller has no responsibility over the connection and SHOULD NOT close it.
func (s *E2ETestSuite) GetGRPCClient(chain ibc.Chain) *grpc.ClientConn {
	cc := s.grpcClients[chain]
	s.Require().NotNil(cc)
	return cc
}
