package keeper_test

import (
	"strconv"
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

// Prevent strconv unused error
var _ = strconv.IntSize

func createNPoolPosition(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.PoolPosition {
	items := make([]types.PoolPosition, n)
	for i := range items {
		items[i].PoolId = strconv.Itoa(i)

		keeper.SetPoolPosition(ctx, items[i])
	}
	return items
}

func TestPoolPositionGet(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolPosition(keeper, ctx, 10)
	for _, item := range items {
		rst, found := keeper.GetPoolPosition(ctx,
			item.PoolId,
		)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&rst),
		)
	}
}

func TestPoolPositionRemove(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolPosition(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemovePoolPosition(ctx,
			item.PoolId,
		)
		_, found := keeper.GetPoolPosition(ctx,
			item.PoolId,
		)
		require.False(t, found)
	}
}

func TestPoolPositionGetAll(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolPosition(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllPoolPosition(ctx)),
	)
}
