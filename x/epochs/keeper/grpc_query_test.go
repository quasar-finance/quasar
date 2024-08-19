package keeper_test

import (
	"testing"
	"time"

	"github.com/quasar-finance/quasar/testutil"
	"github.com/quasar-finance/quasar/x/epochs"
	"github.com/quasar-finance/quasar/x/epochs/keeper"
	"github.com/quasar-finance/quasar/x/epochs/types"
	"github.com/stretchr/testify/require"
)

func TestQueryEpochInfos(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper
	epochs.InitGenesis(ctx, k, *types.DefaultGenesis())

	chainStartTime := ctx.BlockTime()

	querier := keeper.NewQuerier(*k)

	// Invalid param
	epochInfosResponse, err := querier.EpochInfos(ctx, &types.QueryEpochsInfoRequest{})
	require.NoError(t, err)
	require.Len(t, epochInfosResponse.Epochs, 4)

	// check that EpochInfos are correct
	// Epochs come ordered alphabetically by identifier
	require.Equal(t, epochInfosResponse.Epochs[0].Identifier, "day")
	require.Equal(t, epochInfosResponse.Epochs[0].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].Duration, time.Hour*24)
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[1].Identifier, "hour")
	require.Equal(t, epochInfosResponse.Epochs[1].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].Duration, time.Hour)
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[2].Identifier, "minute")
	require.Equal(t, epochInfosResponse.Epochs[2].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[2].Duration, time.Minute)
	require.Equal(t, epochInfosResponse.Epochs[2].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[2].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[2].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[3].Identifier, "week")
	require.Equal(t, epochInfosResponse.Epochs[3].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[3].Duration, time.Hour*24*7)
	require.Equal(t, epochInfosResponse.Epochs[3].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[3].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[3].EpochCountingStarted, false)
}
