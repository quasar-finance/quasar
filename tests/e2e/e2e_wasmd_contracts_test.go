package e2e

import (
	"context"
	"io"
	"net/http"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/stretchr/testify/suite"
)

const (
	testContractURL = "https://github.com/CosmWasm/cw-plus/releases/download/v0.16.0/cw20_base.wasm"
)

func TestWasmdTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)

	s := &WasmdTestSuite{E2ETestSuite: b.Build()}
	suite.Run(t, s)
}

type WasmdTestSuite struct {
	*testsuite.E2ETestSuite
}

func (s *WasmdTestSuite) TestDeployContract_SuccessfulDeployment() {
	t := s.T()
	t.Parallel()
	ctx := context.Background()

	// Setup an account of quasar
	quasarAccount := s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)
	quasarAddress := quasarAccount.Bech32Address(s.Quasar().Config().Bech32Prefix)

	// Check the codes endpoint and ensure that it's empty at the beginning
	codes := s.QueryWasmCodes(ctx, s.Quasar())
	s.Require().Empty(codes)

	// Get the cw20_base v0.16 contract from cw-plus release page
	resp, err := http.Get(testContractURL)
	s.Require().NoErrorf(err, "could not http get the contract at %s", testContractURL)
	defer resp.Body.Close()
	contract, err := io.ReadAll(resp.Body)
	s.Require().NoError(err)

	// Store the contract in chain
	codeID := s.StoreContractCode(ctx, s.Quasar(), quasarAccount.KeyName, contract)

	// Check if the contract is actually deployed
	codes = s.QueryWasmCodes(ctx, s.Quasar())
	s.Require().Len(codes, 1)
	s.Require().Equal(codes[0].CodeID, codeID)

	// Schema from https://github.com/CosmWasm/cw-plus/releases/download/v0.16.0/cw20-base.json
	args := map[string]any{
		"decimals": 6,
		"initial_balances": []any{
			map[string]any{
				"address": quasarAddress,
				"amount":  "100",
			},
		},
		"name":   "TestToken",
		"symbol": "TTT",
	}
	res := s.InstantiateContract(ctx, s.Quasar(), quasarAccount.KeyName, codeID, "test-label", quasarAddress, sdk.NewCoins(), args)
	s.Require().NotEmpty(res.Address)
}
