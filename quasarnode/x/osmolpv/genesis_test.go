package osmolpv_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/osmolpv"
	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		FeeData: &types.FeeData{
			FeeCollector: "8",
			FeeType:      42,
			BlockHeight:  66,
			Memo:         "52",
		},
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.OsmolpvKeeper(t)
	osmolpv.InitGenesis(ctx, *k, genesisState)
	got := osmolpv.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.Equal(t, genesisState.FeeData, got.FeeData)
	// this line is used by starport scaffolding # genesis/test/assert
}
