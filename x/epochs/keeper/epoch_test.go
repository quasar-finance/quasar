package keeper_test

import (
	"testing"
	"time"

	"github.com/abag/quasarnode/x/epochs/keeper"
	"github.com/abag/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestEpochLifeCycle(t *testing.T) {
	// TODO use TestKeepers context
	var ctx sdk.Context
	// TODO use TestKeepers
	var epochsKeeper keeper.Keeper

	epochInfo := types.EpochInfo{
		Identifier:            "monthly",
		StartTime:             time.Time{},
		Duration:              time.Hour * 24 * 30,
		CurrentEpoch:          0,
		CurrentEpochStartTime: time.Time{},
		EpochCountingStarted:  false,
	}
	epochsKeeper.SetEpochInfo(ctx, epochInfo)
	epochInfoSaved := epochsKeeper.GetEpochInfo(ctx, "monthly")
	require.Equal(t, epochInfo, epochInfoSaved)

	allEpochs := epochsKeeper.AllEpochInfos(ctx)
	require.Len(t, allEpochs, 3)
	require.Equal(t, allEpochs[0].Identifier, "day") // alphabetical order
	require.Equal(t, allEpochs[1].Identifier, "monthly")
	require.Equal(t, allEpochs[2].Identifier, "week")
}
