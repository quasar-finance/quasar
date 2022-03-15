package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/osmolpv/keeper"
	"github.com/abag/quasarnode/x/osmolpv/types"
)

func createTestLpStat(keeper *keeper.Keeper, ctx sdk.Context) types.LpStat {
	item := types.LpStat{}
	keeper.SetLpStat(ctx, item)
	return item
}

func TestLpStatGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestLpStat(keeper, ctx)
	rst, found := keeper.GetLpStat(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestLpStatRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestLpStat(keeper, ctx)
	keeper.RemoveLpStat(ctx)
	_, found := keeper.GetLpStat(ctx)
	require.False(t, found)
}
