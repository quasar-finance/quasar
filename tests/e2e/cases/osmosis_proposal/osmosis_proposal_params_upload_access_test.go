package osmosis_proposal

import (
	"context"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/stretchr/testify/require"
	"testing"

	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/stretchr/testify/suite"
)

const (
	userFunds            = int64(10_000_000_000)
	proposalPath         = "./_utils/proposal_params_upload_access.json"
	paramsBeforeProposal = "{\"permission\":\"Everybody\"}"
	paramsAfterProposal  = "{\"permission\":\"AnyOfAddresses\",\"addresses\":[\"osmo1rlq43kswpawqhutgrn6dfqumnk534sh896x8vj\"]}"
)

func TestOsmosisProposal(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &OsmosisProposal{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type OsmosisProposal struct {
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	*testsuite.E2ETestSuite
}

// TestOsmosisProposal_Timeout
func (s *OsmosisProposal) TestOsmosisProposal() {
	t := s.T()
	ctx := context.Background()

	t.Log("Create an user with fund on Osmosis chain")
	user := s.CreateUserAndFund(ctx, s.Osmosis(), userFunds)

	t.Log("Check user balance expecting to be the funded amount")
	userBalance, err := s.Osmosis().GetBalance(ctx, user.Bech32Address(s.Osmosis().Config().Bech32Prefix), s.Osmosis().Config().Denom)
	s.Require().NoError(err)
	s.Require().Equal(userFunds, userBalance)

	param, _ := s.Osmosis().QueryParam(ctx, "wasm", "uploadAccess")
	require.Equal(t, paramsBeforeProposal, param.Value, "Wasm Params before proposals are not correct")

	paramTx, err := s.Osmosis().ParamChangeProposal(ctx, user.KeyName, proposalPath)
	require.NoError(t, err, "error submitting param change proposal tx")

	err = s.Osmosis().VoteOnProposalAllValidators(ctx, paramTx.ProposalID, cosmos.ProposalVoteYes)
	require.NoError(t, err, "failed to submit votes")

	height, _ := s.Osmosis().Height(ctx)
	_, err = cosmos.PollForProposalStatus(ctx, s.Osmosis(), height, height+10, paramTx.ProposalID, cosmos.ProposalStatusPassed)
	require.NoError(t, err, "proposal status did not change to passed in expected number of blocks")

	param, _ = s.Osmosis().QueryParam(ctx, "wasm", "uploadAccess")
	require.Equal(t, paramsAfterProposal, param.Value, "Wasm Params after proposals are not correct")
}
