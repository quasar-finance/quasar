package keeper_test

import (
	"testing"

	gocontext "context"
	"time"

	"github.com/abag/quasarnode/x/epochs/keeper"
	"github.com/abag/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestQueryEpochInfos(t *testing.T) {
	// TODO use TestKeepers context
	var ctx sdk.Context
	// TODO use TestKeepers
	var epochsKeeper keeper.Keeper

	chainStartTime := ctx.BlockTime()

	querier := keeper.NewQuerier(epochsKeeper)

	// queryHelper := baseapp.NewQueryServerTestHelper(suite.ctx, suite.app.InterfaceRegistry())
	// types.RegisterQueryServer(queryHelper, keeper.NewQuerier(*suite.app.EpochsKeeper))
	// queryClient := types.NewQueryClient(queryHelper)

	// Invalid param
	epochInfosResponse, err := querier.EpochInfos(gocontext.Background(), &types.QueryEpochsInfoRequest{})
	require.NoError(t, err)
	require.Len(t, epochInfosResponse.Epochs, 2)

	// check if EpochInfos are correct
	require.Equal(t, epochInfosResponse.Epochs[0].Identifier, "day")
	require.Equal(t, epochInfosResponse.Epochs[0].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].Duration, time.Hour*24)
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[0].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[0].EpochCountingStarted, false)
	require.Equal(t, epochInfosResponse.Epochs[1].Identifier, "week")
	require.Equal(t, epochInfosResponse.Epochs[1].StartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].Duration, time.Hour*24*7)
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpoch, int64(0))
	require.Equal(t, epochInfosResponse.Epochs[1].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, epochInfosResponse.Epochs[1].EpochCountingStarted, false)
}
