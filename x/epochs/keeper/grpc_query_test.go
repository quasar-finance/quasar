package keeper_test

import (
	"testing"

	"time"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/epochs"
	epochskeeper "github.com/abag/quasarnode/x/epochs/keeper"
	"github.com/abag/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestQueryEpochInfos(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetEpochsKeeper()
	epochs.InitGenesis(ctx, keeper, *types.DefaultGenesis())
	goCtx := sdk.WrapSDKContext(ctx)

	chainStartTime := ctx.BlockTime()

	querier := epochskeeper.NewQuerier(keeper)

	// Invalid param
	epochInfosResponse, err := querier.EpochInfos(goCtx, &types.QueryEpochsInfoRequest{})
	require.NoError(t, err)
	require.Len(t, epochInfosResponse.Epochs, 3)

	// check that EpochInfos are correct
	// Epochs come ordered alphabetically by identifier
	require.Equal(t, epochInfosResponse.Epochs[0].Identifier, "day")
	require.Equal(t, epochInfosResponse.Epochs[0].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].Duration, time.Hour*24)
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[1].Identifier, "minute")
	require.Equal(t, epochInfosResponse.Epochs[1].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].Duration, time.Minute)
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[2].Identifier, "week")
	require.Equal(t, epochInfosResponse.Epochs[2].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[2].Duration, time.Hour*24*7)
	require.Equal(t, epochInfosResponse.Epochs[2].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[2].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[2].EpochCountingStarted, false)
}
