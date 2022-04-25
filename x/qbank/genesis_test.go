package qbank_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/qbank"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper

	genesisState := types.GenesisState{
		Params: types.DefaultParams(),
		// this line is used by starport scaffolding # genesis/test/state
	}

	qbank.InitGenesis(ctx, k, genesisState)
	setParams := k.GetParams(ctx)
	require.Equal(t, genesisState.Params, setParams)
	got := qbank.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	//nullify.Fill(&genesisState)
	//nullify.Fill(got)

	require.Equal(t, genesisState.Params, got.Params)
	require.ElementsMatch(t,
		genesisState.Params.WhiteListedDenomsInOrion,
		got.Params.WhiteListedDenomsInOrion)
	// this line is used by starport scaffolding # genesis/test/assert
}
