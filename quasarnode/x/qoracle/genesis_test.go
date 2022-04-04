package qoracle_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		PoolPositionList: []types.PoolPosition{
			{
				PoolId: "0",
			},
			{
				PoolId: "1",
			},
		},
		PoolRanking: &types.PoolRanking{
			PoolIdsSortedByAPY: []string{"90"},
			PoolIdsSortedByTVL: []string{"81"},
			LastUpdatedTime:    8,
		},
		PoolSpotPriceList: []types.PoolSpotPrice{
			{
				PoolId:   "0",
				DenomIn:  "0",
				DenomOut: "0",
			},
			{
				PoolId:   "1",
				DenomIn:  "1",
				DenomOut: "1",
			},
		},
		PoolInfoList: []types.PoolInfo{
			{
				PoolId: "0",
			},
			{
				PoolId: "1",
			},
		},
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.QoracleKeeper(t)
	qoracle.InitGenesis(ctx, *k, genesisState)
	got := qoracle.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.ElementsMatch(t, genesisState.PoolPositionList, got.PoolPositionList)
	require.Equal(t, genesisState.PoolRanking, got.PoolRanking)
	require.ElementsMatch(t, genesisState.PoolSpotPriceList, got.PoolSpotPriceList)
	require.ElementsMatch(t, genesisState.PoolInfoList, got.PoolInfoList)
	// this line is used by starport scaffolding # genesis/test/assert
}
