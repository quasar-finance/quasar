package keeper_test

import (
	"fmt"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func createNPoolPosition(k *keeper.Keeper, ctx sdk.Context, n int) []types.PoolPosition {
	items := make([]types.PoolPosition, n)
	for i := range items {
		items[i].Creator = sample.AccAddressStr()
		items[i].PoolId = fmt.Sprintf("%d", i)
		items[i].Metrics = &types.PoolMetrics{
			HighestAPY: sdk.NewDec(int64(i)).String(),
			TVL:        sdk.NewDecCoin("usd", sdk.NewInt(int64(i))).String(),
			GaugeAPYs: []*types.GaugeAPY{
				{GaugeId: 3*uint64(i) + 1, Duration: "1s", APY: fmt.Sprintf("%f", 1+float32(i)+0.1)},
				{GaugeId: 3*uint64(i) + 2, Duration: "2s", APY: fmt.Sprintf("%f", 1+float32(i)+0.1)},
			},
		}
		items[i].LastUpdatedTime = 1 + uint64(i)

		k.SetPoolPosition(ctx, items[i])
	}
	return items
}

func TestPoolPositionGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolPosition(&k, ctx, 10)
	for _, item := range items {
		rst, found := k.GetPoolPosition(ctx,
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
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolPosition(&k, ctx, 10)
	for _, item := range items {
		k.RemovePoolPosition(ctx,
			item.PoolId,
		)
		_, found := k.GetPoolPosition(ctx,
			item.PoolId,
		)
		require.False(t, found)
	}
}

func TestPoolPositionGetAll(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolPosition(&k, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(k.GetAllPoolPosition(ctx)),
	)
}
