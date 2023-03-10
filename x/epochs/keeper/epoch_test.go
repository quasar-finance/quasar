package keeper_test

import (
	"testing"
	"time"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/epochs"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
	"github.com/stretchr/testify/require"
)

func TestEpochLifeCycle(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper
	epochs.InitGenesis(ctx, k, *types.DefaultGenesis())

	epochInfo := types.EpochInfo{
		Identifier:            "monthly",
		StartTime:             time.Time{},
		Duration:              time.Hour * 24 * 30,
		CurrentEpoch:          0,
		CurrentEpochStartTime: time.Time{},
		EpochCountingStarted:  false,
	}
	k.SetEpochInfo(ctx, epochInfo)
	epochInfoSaved := k.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo, epochInfoSaved)

	allEpochs := k.AllEpochInfos(ctx)
	require.Len(t, allEpochs, 5)
	require.Equal(t, allEpochs[0].Identifier, "day") // alphabetical order
	require.Equal(t, allEpochs[1].Identifier, "hour")
	require.Equal(t, allEpochs[2].Identifier, "minute")
	require.Equal(t, allEpochs[3].Identifier, "monthly")
	require.Equal(t, allEpochs[4].Identifier, "week")
}
