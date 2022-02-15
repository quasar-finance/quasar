package qoracle_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		PoolPosition: &types.PoolPosition{
			APY:             1,
			TVL:             41,
			LastUpdatedTime: 91,
		},
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.QoracleKeeper(t)
	qoracle.InitGenesis(ctx, *k, genesisState)
	got := qoracle.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.Equal(t, genesisState.PoolPosition, got.PoolPosition)
	// this line is used by starport scaffolding # genesis/test/assert
}
