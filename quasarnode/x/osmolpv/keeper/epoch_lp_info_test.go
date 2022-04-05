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

func createTestEpochLPInfo(keeper *keeper.Keeper, ctx sdk.Context) types.EpochLPInfo {
	item := types.EpochLPInfo{
		EpochDay: uint64(42),
	}
	keeper.SetEpochLPInfo(ctx, item)
	return item
}

func TestEpochLPInfoGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestEpochLPInfo(keeper, ctx)
	rst, found := keeper.GetEpochLPInfo(ctx, uint64(42))
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestEpochLPInfoRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestEpochLPInfo(keeper, ctx)
	keeper.RemoveEpochLPInfo(ctx, uint64(42))
	_, found := keeper.GetEpochLPInfo(ctx, uint64(42))
	require.False(t, found)
}
