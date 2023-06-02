package suite

import (
	"context"
	"encoding/json"
	"path/filepath"
	"strconv"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	paramsutils "github.com/cosmos/cosmos-sdk/x/params/client/utils"
	"github.com/quasarlabs/quasarnode/tests/e2e/dockerutil"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"go.uber.org/zap"
)

// NewParamChangeProposalJSON creates a json implementation of gov legacy proposal
// to be used with cli command "tx gov submit-legacy-proposal param-change"
func NewParamChangeProposalJSON(
	title, description, subspace string,
	key []byte,
	value any,
	deposit sdk.Coins) *paramsutils.ParamChangeProposalJSON {
	changeValue, err := json.Marshal(value)
	if err != nil {
		panic(err)
	}

	return &paramsutils.ParamChangeProposalJSON{
		Title:       "title",
		Description: "description",
		Changes: paramsutils.ParamChangesJSON{
			paramsutils.NewParamChangeJSON(subspace, string(key), changeValue),
		},
		Deposit: deposit.String(),
	}
}

// ExecParamChangeProposal writes the proposal as json in the chain's node, submits the proposal using cli command and
// vote on the proposal on behalf of all available validators to pass the proposal.
func (s *E2ETestSuite) ExecParamChangeProposal(ctx context.Context, chain *cosmos.CosmosChain, keyName string, proposal *paramsutils.ParamChangeProposalJSON) {
	tn := GetFullNode(chain)

	logger := s.logger.With(
		zap.String("chain_id", tn.Chain.Config().ChainID),
		zap.String("test", tn.TestName),
	)

	proposalFile := "proposal.json"
	proposalbz, err := json.Marshal(proposal)
	s.Require().NoError(err)
	fw := dockerutil.NewFileWriter(logger, tn.DockerClient, tn.TestName)
	err = fw.WriteFile(ctx, tn.VolumeName, proposalFile, proposalbz)
	s.Require().NoError(err, "failed to write pool file")

	txhash, err := tn.ExecTx(ctx, keyName,
		"gov", "submit-legacy-proposal", "param-change",
		filepath.Join(tn.HomeDir(), proposalFile),
		"--gas", "auto",
	)
	s.Require().NoError(err, "failed to create pool")

	var resp govtypes.MsgSubmitProposalResponse
	s.AssertSuccessfulResultTx(ctx, chain, txhash, &resp)

	s.CompleteGovProposal(ctx, chain, resp.ProposalId)
}

// CompleteGovProposal issues vote on the proposal on behalf of all available validators to pass the proposal.
func (s *E2ETestSuite) CompleteGovProposal(ctx context.Context, chain *cosmos.CosmosChain, proposalId uint64) {
	proposal := s.QueryProposal(ctx, chain, proposalId)
	s.Require().Equal(govtypes.StatusVotingPeriod, proposal.Status)

	err := chain.VoteOnProposalAllValidators(ctx, strconv.FormatUint(proposalId, 10), cosmos.ProposalVoteYes)
	s.Require().NoError(err)

	// ensure voting period has not passed before validators finished voting
	proposal = s.QueryProposal(ctx, chain, proposalId)
	s.Require().Equal(govtypes.StatusVotingPeriod, proposal.Status)

	time.Sleep(VotingPeriod) // pass proposal

	proposal = s.QueryProposal(ctx, chain, proposalId)
	s.Require().Equal(govtypes.StatusPassed, proposal.Status)
}
