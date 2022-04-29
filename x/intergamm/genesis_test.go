package intergamm_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/intergamm"
	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),
		// this line is used by starport scaffolding # genesis/test/state
	}
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	intergamm.InitGenesis(ctx, k, genesisState)
	got := intergamm.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	// this line is used by starport scaffolding # genesis/test/assert
}
