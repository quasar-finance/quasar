package cl_vaults

import (
	"context"
	"fmt"
	sdk "github.com/cosmos/cosmos-sdk/types"
	testSuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/stretchr/testify/require"
	"github.com/stretchr/testify/suite"
	"go.uber.org/zap"
	"testing"
)

type CLVaultsBuilderSuite struct {
	*testSuite.E2eTestBuilder
	suite.Suite
}

const (
	clVaultContractPath = "../../../../smart-contracts/artifacts/cl_vault-aarch64.wasm"
)

func TestCLVaultBuilder(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testSuite.NewE2eTestBuilder(t)

	// add osmosis chain and genesis tokens to initialise a treasury account with
	genesisTokensOsmosis := sdk.NewCoins().Add(sdk.NewInt64Coin(testSuite.OsmosisChain.Denom, 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("fakestake", 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("stake1", 100_000_000_000_000_000)).
		Add(sdk.NewInt64Coin("usdc", 100_000_000_000_000_000))
	b.AddChain(testSuite.OsmosisChain, genesisTokensOsmosis, 1, 0, true)

	_, found := b.GetChain("osmosis")
	require.True(t, found)

	s := &CLVaultsBuilderSuite{
		E2eTestBuilder: b.Build(),
	}
	suite.Run(t, s)
}

func (s *CLVaultsBuilderSuite) TestCLVault() {
	ctx := context.Background()

	// find Osmosis chain
	osmosis, found := s.GetChain("osmosis")
	s.Require().True(found)

	// deploy concentrated pool on Osmosis
	txHash, err := osmosis.ExecTx(
		ctx,
		[]string{
			"concentratedliquidity", "create-pool", "uosmo", "fakestake", "100", "0.01",
			"--gas", "20000000",
		},
		osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName,
		"",
		"",
		nil,
		sdk.Coins{},
		s.Logger.With(
			zap.String("chain_id", osmosis.Chain.Config().ChainID),
			zap.String("test", testSuite.GetFullNode(osmosis.Chain).TestName)),
	)

	s.Require().NoError(err)

	err = osmosis.AssertSuccessfulResultTx(ctx, txHash, nil)
	s.Require().NoError(err)

	// store lp strategy contract code
	clVaultCodeID, err := testSuite.StoreContractCode(ctx, osmosis.Chain, clVaultContractPath, osmosis.ChainAccount[testSuite.AuthorityKeyName].KeyName, s.Logger)
	s.Require().NoError(err)

	clVaultInit := map[string]any{
		"thesis":      "provide liq",
		"name":        "test-vault",
		"admin":       osmosis.ChainAccount[testSuite.AuthorityKeyName].Address,
		"range_admin": osmosis.ChainAccount[testSuite.AuthorityKeyName].Address,
		"pool_id":     1,
		"config": map[string]any{
			"performance_fee":   "20.0",
			"treasury":          osmosis.ChainAccount[testSuite.AuthorityKeyName].Address,
			"swap_max_slippage": "0.5",
		},
		"vault_token_subdenom": "cl-vault",
		"initial_lower_tick":   -32500,
		"initial_upper_tick":   -32600,
	}

	newConrtacts := []*testSuite.Contract{
		testSuite.NewContract(clVaultInit, "clVault", clVaultCodeID),
	}

	err = osmosis.SetContracts(newConrtacts)
	s.Require().NoError(err)

	for _, c := range newConrtacts {
		err = c.InstantiateContract(ctx, osmosis.ChainAccount[testSuite.AuthorityKeyName], osmosis.Chain, sdk.Coins{})
		s.Require().NoError(err)
	}

	// get all the primitives by their type
	prim1, err := osmosis.FindContractByLabel("clVault")
	s.Require().NoError(err)

	fmt.Println(prim1)

}
