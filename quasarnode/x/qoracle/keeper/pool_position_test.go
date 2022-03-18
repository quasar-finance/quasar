package keeper_test

import (
	"fmt"
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func createNPoolPosition(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.PoolPosition {
	items := make([]types.PoolPosition, n)
	for i := range items {
		items[i].Creator = sample.AccAddress()
		items[i].PoolId = fmt.Sprintf("%d", i)
		items[i].Metrics = &types.PoolMetrics{APY: sdk.NewDec(int64(i)).String(), TVL: sdk.NewDecCoin("usd", sdk.NewInt(int64(i))).String()}
		items[i].LastUpdatedTime = 1 + uint64(i)

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
