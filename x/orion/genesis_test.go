package orion_test

import (
	"testing"
	"time"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		FeeData: &types.FeeData{
			FeeCollector: "8",
			FeeType:      42,
			BlockHeight:  66,
			Memo:         "52",
		},
		LpPosition: &types.LpPosition{
			LpID:                   85,
			LockID:                 35,
			BondingStartEpochDay:   9,
			BondDuration:           44,
			UnbondingStartEpochDay: 2,
			UnbondingDuration:      83,
			PoolID:                 48,
		},
		EpochLPInfo: &types.EpochLPInfo{
			EpochDay: 16,
			TotalLps: 87,
		},
		RewardCollection: &types.RewardCollection{
			TimeCollected: time.Time{},
		},
		UserLPInfo: &types.UserLPInfo{
			PositionShare: sdk.NewDec(5),
		},
		LpStat: &types.LpStat{
			LpCount: 90,
		},
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.OrionKeeper(t)
	orion.InitGenesis(ctx, *k, genesisState)
	got := orion.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.Equal(t, genesisState.FeeData, got.FeeData)
	require.Equal(t, genesisState.LpPosition, got.LpPosition)
	require.Equal(t, genesisState.EpochLPInfo, got.EpochLPInfo)
	require.Equal(t, genesisState.RewardCollection, got.RewardCollection)
	require.Equal(t, genesisState.UserLPInfo, got.UserLPInfo)
	require.Equal(t, genesisState.LpStat, got.LpStat)
	// this line is used by starport scaffolding # genesis/test/assert
}
