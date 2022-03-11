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

func createTestLpPosition(keeper *keeper.Keeper, ctx sdk.Context) types.LpPosition {
	item := types.LpPosition{}
	keeper.SetLpPosition(ctx, item)
	return item
}

func TestLpPositionGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestLpPosition(keeper, ctx)
	rst, found := keeper.GetLpPosition(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestLpPositionRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestLpPosition(keeper, ctx)
	keeper.RemoveLpPosition(ctx)
	_, found := keeper.GetLpPosition(ctx)
	require.False(t, found)
}
