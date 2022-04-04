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

func createTestFeeData(keeper *keeper.Keeper, ctx sdk.Context) types.FeeData {
	item := types.FeeData{}
	keeper.SetFeeData(ctx, item)
	return item
}

func TestFeeDataGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestFeeData(keeper, ctx)
	rst, found := keeper.GetFeeData(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestFeeDataRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestFeeData(keeper, ctx)
	keeper.RemoveFeeData(ctx)
	_, found := keeper.GetFeeData(ctx)
	require.False(t, found)
}
