package qoracle_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
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

	setup := testutil.NewTestSetup(t)
	ctx, keeper := setup.Ctx, setup.Keepers.QoracleKeeper
	qoracle.InitGenesis(ctx, keeper, genesisState)
	setParams := keeper.GetParams(ctx)
	require.Equal(t, genesisState.Params, setParams)
	got := qoracle.ExportGenesis(ctx, keeper)
	//nullify.Fill(&genesisState)
	//nullify.Fill(got)

	require.NotNil(t, got)
	require.ElementsMatch(t, genesisState.PoolPositionList, got.PoolPositionList)
	require.Equal(t, genesisState.PoolRanking, got.PoolRanking)
	require.ElementsMatch(t, genesisState.PoolSpotPriceList, got.PoolSpotPriceList)
	require.ElementsMatch(t, genesisState.PoolInfoList, got.PoolInfoList)
	require.Equal(t, genesisState.Params, got.Params)
	// this line is used by starport scaffolding # genesis/test/assert
}
