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

func createNPoolSpotPrice(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.PoolSpotPrice {
	items := make([]types.PoolSpotPrice, n)
	for i := range items {
		items[i].Creator = sample.AccAddress()
		items[i].PoolId = fmt.Sprintf("%d", i)
		items[i].DenomIn = fmt.Sprintf("abc%d", i)
		items[i].DenomOut = fmt.Sprintf("cba%d", i)
		items[i].Price = fmt.Sprintf("%f", 1.5*float32(i))
		items[i].LastUpdatedTime = 1 + uint64(i)

		keeper.SetPoolSpotPrice(ctx, items[i])
	}
	return items
}

func TestPoolSpotPriceGet(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolSpotPrice(keeper, ctx, 10)
	for _, item := range items {
		rst, found := keeper.GetPoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&rst),
		)
	}
}
func TestPoolSpotPriceRemove(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolSpotPrice(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemovePoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		_, found := keeper.GetPoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		require.False(t, found)
	}
}

func TestPoolSpotPriceGetAll(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	items := createNPoolSpotPrice(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllPoolSpotPrice(ctx)),
	)
}
