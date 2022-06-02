package orion_test

import (
	"testing"
	"time"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion"
	"github.com/abag/quasarnode/x/orion/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		LpPosition: &types.LpPosition{
			LpID:                   85,
			LockID:                 35,
			BondingStartEpochDay:   9,
			BondDuration:           44,
			UnbondingStartEpochDay: 2,
			UnbondingDuration:      83,
			PoolID:                 48,
		},
		RewardCollection: &types.RewardCollection{
			TimeCollected: time.Time{},
		},
		LpStat: &types.LpStat{
			LpCount: 90,
		},
		// this line is used by starport scaffolding # genesis/test/state
	}

	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	orion.InitGenesis(ctx, k, genesisState)
	got := orion.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.Equal(t, genesisState.Params, got.Params)
	require.Equal(t, genesisState.LpPosition, got.LpPosition)
	require.Equal(t, genesisState.RewardCollection, got.RewardCollection)
	require.Equal(t, genesisState.LpStat, got.LpStat)
	// this line is used by starport scaffolding # genesis/test/assert
}
