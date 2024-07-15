package epochs_test

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/epochs"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
)

func TestEpochsExportGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper
	epochs.InitGenesis(ctx, k, *types.DefaultGenesis())

	chainStartTime := ctx.BlockTime()
	chainStartHeight := ctx.BlockHeight()

	genesis := epochs.ExportGenesis(ctx, k)
	require.Len(t, genesis.Epochs, 4)

	require.Equal(t, genesis.Epochs[0].Identifier, "day")
	require.Equal(t, genesis.Epochs[0].StartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[0].Duration, time.Hour*24)
	require.Equal(t, genesis.Epochs[0].CurrentEpoch, int64(0))
	require.Equal(t, genesis.Epochs[0].CurrentEpochStartHeight, chainStartHeight)
	require.Equal(t, genesis.Epochs[0].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[0].EpochCountingStarted, false)
	require.Equal(t, genesis.Epochs[1].Identifier, "hour")
	require.Equal(t, genesis.Epochs[1].StartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[1].Duration, time.Hour)
	require.Equal(t, genesis.Epochs[1].CurrentEpoch, int64(0))
	require.Equal(t, genesis.Epochs[1].CurrentEpochStartHeight, chainStartHeight)
	require.Equal(t, genesis.Epochs[1].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[1].EpochCountingStarted, false)
	require.Equal(t, genesis.Epochs[2].Identifier, "minute")
	require.Equal(t, genesis.Epochs[2].StartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[2].Duration, time.Minute)
	require.Equal(t, genesis.Epochs[2].CurrentEpoch, int64(0))
	require.Equal(t, genesis.Epochs[2].CurrentEpochStartHeight, chainStartHeight)
	require.Equal(t, genesis.Epochs[2].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[2].EpochCountingStarted, false)
	require.Equal(t, genesis.Epochs[3].Identifier, "week")
	require.Equal(t, genesis.Epochs[3].StartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[3].Duration, time.Hour*24*7)
	require.Equal(t, genesis.Epochs[3].CurrentEpoch, int64(0))
	require.Equal(t, genesis.Epochs[3].CurrentEpochStartHeight, chainStartHeight)
	require.Equal(t, genesis.Epochs[3].CurrentEpochStartTime, chainStartTime)
	require.Equal(t, genesis.Epochs[3].EpochCountingStarted, false)
}

func TestEpochsInitGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper

	// On init genesis, default epochs information is set
	// To check init genesis again, should make it fresh status
	epochInfos := k.AllEpochInfos(ctx)
	for _, epochInfo := range epochInfos {
		k.DeleteEpochInfo(ctx, epochInfo.Identifier)
	}

	now := time.Now()
	ctx = ctx.WithBlockHeight(1)
	ctx = ctx.WithBlockTime(now)

	//test genesisState validation
	genesisState := types.GenesisState{
		Epochs: []types.EpochInfo{
			{
				Identifier:              "monthly",
				StartTime:               time.Time{},
				Duration:                time.Hour * 24,
				CurrentEpoch:            0,
				CurrentEpochStartHeight: ctx.BlockHeight(),
				CurrentEpochStartTime:   time.Time{},
				EpochCountingStarted:    true,
			},
			{
				Identifier:              "monthly",
				StartTime:               time.Time{},
				Duration:                time.Hour * 24,
				CurrentEpoch:            0,
				CurrentEpochStartHeight: ctx.BlockHeight(),
				CurrentEpochStartTime:   time.Time{},
				EpochCountingStarted:    true,
			},
		},
	}
	require.EqualError(t, genesisState.Validate(), "epoch identifier should be unique")

	genesisState = types.GenesisState{
		Epochs: []types.EpochInfo{
			{
				Identifier:              "monthly",
				StartTime:               time.Time{},
				Duration:                time.Hour * 24,
				CurrentEpoch:            0,
				CurrentEpochStartHeight: ctx.BlockHeight(),
				CurrentEpochStartTime:   time.Time{},
				EpochCountingStarted:    true,
			},
		},
	}

	epochs.InitGenesis(ctx, k, genesisState)
	epochInfo := k.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo.Identifier, "monthly")
	require.Equal(t, epochInfo.StartTime.UTC().String(), now.UTC().String())
	require.Equal(t, epochInfo.Duration, time.Hour*24)
	require.Equal(t, epochInfo.CurrentEpoch, int64(0))
	require.Equal(t, epochInfo.CurrentEpochStartHeight, ctx.BlockHeight())
	require.Equal(t, epochInfo.CurrentEpochStartTime.UTC().String(), time.Time{}.String())
	require.Equal(t, epochInfo.EpochCountingStarted, true)
}
