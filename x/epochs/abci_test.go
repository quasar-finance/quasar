package epochs_test

import (
	"testing"
	"time"

	"github.com/quasar-finance/quasar/testutil"
	"github.com/quasar-finance/quasar/x/epochs"
	"github.com/quasar-finance/quasar/x/epochs/types"
	"github.com/stretchr/testify/require"
)

func TestEpochInfoChangesBeginBlockerAndInitGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper
	var epochInfo types.EpochInfo

	now := time.Now()

	tests := []struct {
		expCurrentEpochStartTime   time.Time
		expCurrentEpochStartHeight int64
		expCurrentEpoch            int64
		expInitialEpochStartTime   time.Time
		fn                         func()
	}{
		{
			// Only advance 2 seconds, do not increment epoch
			expCurrentEpochStartHeight: 2,
			expCurrentEpochStartTime:   now,
			expCurrentEpoch:            1,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
			},
		},
		{
			expCurrentEpochStartHeight: 2,
			expCurrentEpochStartTime:   now,
			expCurrentEpoch:            1,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
			},
		},
		{
			expCurrentEpochStartHeight: 2,
			expCurrentEpochStartTime:   now,
			expCurrentEpoch:            1,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(time.Hour * 24 * 31))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
			},
		},
		// Test that incrementing _exactly_ 1 month increments the epoch count.
		{
			expCurrentEpochStartHeight: 3,
			expCurrentEpochStartTime:   now.Add(time.Hour * 24 * 31),
			expCurrentEpoch:            2,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(time.Hour * 24 * 32))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
			},
		},
		{
			expCurrentEpochStartHeight: 3,
			expCurrentEpochStartTime:   now.Add(time.Hour * 24 * 31),
			expCurrentEpoch:            2,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(time.Hour * 24 * 32))
				epochs.BeginBlocker(ctx, k)
				ctx.WithBlockHeight(4).WithBlockTime(now.Add(time.Hour * 24 * 33))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
			},
		},
		{
			expCurrentEpochStartHeight: 3,
			expCurrentEpochStartTime:   now.Add(time.Hour * 24 * 31),
			expCurrentEpoch:            2,
			expInitialEpochStartTime:   now,
			fn: func() {
				ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
				epochs.BeginBlocker(ctx, k)
				ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(time.Hour * 24 * 32))
				epochs.BeginBlocker(ctx, k)
				numBlocksSinceStart, _ := k.NumBlocksSinceEpochStart(ctx, "monthly")
				require.Equal(t, int64(0), numBlocksSinceStart)
				ctx = ctx.WithBlockHeight(4).WithBlockTime(now.Add(time.Hour * 24 * 33))
				epochs.BeginBlocker(ctx, k)
				epochInfo = k.GetEpochInfo(ctx, "monthly")
				numBlocksSinceStart, _ = k.NumBlocksSinceEpochStart(ctx, "monthly")
				require.Equal(t, int64(1), numBlocksSinceStart)
			},
		},
	}

	for _, test := range tests {
		// TODO use initialized keeper + context
		// On init genesis, default epochs information is set
		// To check init genesis again, should make it fresh status
		epochInfos := k.AllEpochInfos(ctx)
		for _, epochInfo := range epochInfos {
			k.DeleteEpochInfo(ctx, epochInfo.Identifier)
		}

		ctx = ctx.WithBlockHeight(1).WithBlockTime(now)

		// check init genesis
		epochs.InitGenesis(ctx, k, types.GenesisState{
			Epochs: []types.EpochInfo{
				{
					Identifier:              "monthly",
					StartTime:               time.Time{},
					Duration:                time.Hour * 24 * 31,
					CurrentEpoch:            0,
					CurrentEpochStartHeight: ctx.BlockHeight(),
					CurrentEpochStartTime:   time.Time{},
					EpochCountingStarted:    false,
				},
			},
		})

		test.fn()

		require.Equal(t, epochInfo.Identifier, "monthly")
		require.Equal(t, epochInfo.StartTime.UTC().String(), test.expInitialEpochStartTime.UTC().String())
		require.Equal(t, epochInfo.Duration, time.Hour*24*31)
		require.Equal(t, epochInfo.CurrentEpoch, test.expCurrentEpoch)
		require.Equal(t, epochInfo.CurrentEpochStartHeight, test.expCurrentEpochStartHeight)
		require.Equal(t, epochInfo.CurrentEpochStartTime.UTC().String(), test.expCurrentEpochStartTime.UTC().String())
		require.Equal(t, epochInfo.EpochCountingStarted, true)
	}
}

func TestEpochStartingOneMonthAfterInitGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper

	// On init genesis, default epochs information is set
	// To check init genesis again, should make it fresh status
	epochInfos := k.AllEpochInfos(ctx)
	for _, epochInfo := range epochInfos {
		k.DeleteEpochInfo(ctx, epochInfo.Identifier)
	}

	now := time.Now()
	week := time.Hour * 24 * 7
	month := time.Hour * 24 * 30
	initialBlockHeight := int64(1)
	ctx = ctx.WithBlockHeight(initialBlockHeight).WithBlockTime(now)

	epochs.InitGenesis(ctx, k, types.GenesisState{
		Epochs: []types.EpochInfo{
			{
				Identifier:              "monthly",
				StartTime:               now.Add(month),
				Duration:                time.Hour * 24 * 30,
				CurrentEpoch:            0,
				CurrentEpochStartHeight: ctx.BlockHeight(),
				CurrentEpochStartTime:   time.Time{},
				EpochCountingStarted:    false,
			},
		},
	})

	// epoch not started yet
	epochInfo := k.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo.CurrentEpoch, int64(0))
	require.Equal(t, epochInfo.CurrentEpochStartHeight, initialBlockHeight)
	require.Equal(t, epochInfo.CurrentEpochStartTime, time.Time{})
	require.Equal(t, epochInfo.EpochCountingStarted, false)

	// after 1 week
	ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(week))
	epochs.BeginBlocker(ctx, k)

	// epoch not started yet
	epochInfo = k.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo.CurrentEpoch, int64(0))
	require.Equal(t, epochInfo.CurrentEpochStartHeight, initialBlockHeight)
	require.Equal(t, epochInfo.CurrentEpochStartTime, time.Time{})
	require.Equal(t, epochInfo.EpochCountingStarted, false)

	// after 1 month
	ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(month))
	epochs.BeginBlocker(ctx, k)

	// epoch started
	epochInfo = k.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo.CurrentEpoch, int64(1))
	require.Equal(t, epochInfo.CurrentEpochStartHeight, ctx.BlockHeight())
	require.Equal(t, epochInfo.CurrentEpochStartTime.UTC().String(), now.Add(month).UTC().String())
	require.Equal(t, epochInfo.EpochCountingStarted, true)
}

// This test ensures legacy EpochInfo messages will not throw errors via InitGenesis and BeginBlocker
func TestLegacyEpochSerialization(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.EpochsKeeper

	// Legacy Epoch Info message - without CurrentEpochStartHeight property
	legacyEpochInfo := types.EpochInfo{
		Identifier:            "monthly",
		StartTime:             time.Time{},
		Duration:              time.Hour * 24 * 31,
		CurrentEpoch:          0,
		CurrentEpochStartTime: time.Time{},
		EpochCountingStarted:  false,
	}

	now := time.Now()

	// On init genesis, default epochs information is set
	// To check init genesis again, should make it fresh status
	epochInfos := k.AllEpochInfos(ctx)
	for _, epochInfo := range epochInfos {
		k.DeleteEpochInfo(ctx, epochInfo.Identifier)
	}

	ctx = ctx.WithBlockHeight(1).WithBlockTime(now)

	// check init genesis
	epochs.InitGenesis(ctx, k, types.GenesisState{
		Epochs: []types.EpochInfo{legacyEpochInfo},
	})

	// Do not increment epoch
	ctx = ctx.WithBlockHeight(2).WithBlockTime(now.Add(time.Second))
	epochs.BeginBlocker(ctx, k)

	// Increment epoch
	ctx = ctx.WithBlockHeight(3).WithBlockTime(now.Add(time.Hour * 24 * 32))
	epochs.BeginBlocker(ctx, k)
	epochInfo := k.GetEpochInfo(ctx, "monthly")

	require.NotEqual(t, epochInfo.CurrentEpochStartHeight, int64(0))
}
