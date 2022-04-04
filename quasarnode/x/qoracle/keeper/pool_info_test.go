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

func createNPoolInfo(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.PoolInfo {
	items := make([]types.PoolInfo, n)
	for i := range items {
		items[i].PoolId = strconv.Itoa(i)

		keeper.SetPoolInfo(ctx, items[i])
	}
	return items
}

func TestPoolInfoGet(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolInfo(keeper, ctx, 10)
	for _, item := range items {
		rst, found := keeper.GetPoolInfo(ctx,
			item.PoolId,
		)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&rst),
		)
	}
}
func TestPoolInfoRemove(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolInfo(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemovePoolInfo(ctx,
			item.PoolId,
		)
		_, found := keeper.GetPoolInfo(ctx,
			item.PoolId,
		)
		require.False(t, found)
	}
}

func TestPoolInfoGetAll(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolInfo(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllPoolInfo(ctx)),
	)
}
