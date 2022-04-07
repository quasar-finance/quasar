package keeper_test

import (
	"testing"
	"time"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/epochs"
	"github.com/abag/quasarnode/x/epochs/types"
	"github.com/stretchr/testify/require"
)

func TestEpochLifeCycle(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetEpochsKeeper()
	epochs.InitGenesis(ctx, keeper, *types.DefaultGenesis())

	epochInfo := types.EpochInfo{
		Identifier:            "monthly",
		StartTime:             time.Time{},
		Duration:              time.Hour * 24 * 30,
		CurrentEpoch:          0,
		CurrentEpochStartTime: time.Time{},
		EpochCountingStarted:  false,
	}
	keeper.SetEpochInfo(ctx, epochInfo)
	epochInfoSaved := keeper.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo, epochInfoSaved)

	allEpochs := keeper.AllEpochInfos(ctx)
	require.Len(t, allEpochs, 3)
	require.Equal(t, allEpochs[0].Identifier, "day") // alphabetical order
	require.Equal(t, allEpochs[1].Identifier, "monthly")
	require.Equal(t, allEpochs[2].Identifier, "week")
}
