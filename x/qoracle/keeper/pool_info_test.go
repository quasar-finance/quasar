package keeper_test

import (
	"fmt"
	"testing"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func createNPoolInfo(k *keeper.Keeper, ctx sdk.Context, n int) []types.PoolInfo {
	items := make([]types.PoolInfo, n)
	for i := range items {
		items[i].Creator = sample.AccAddressStr()
		items[i].PoolId = fmt.Sprintf("%d", i)
		items[i].LastUpdatedTime = 1 + uint64(i)

		k.SetPoolInfo(ctx, items[i])
	}
	return items
}

func TestPoolInfoGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolInfo(&k, ctx, 10)
	for _, item := range items {
		rst, found := k.GetPoolInfo(ctx,
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
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolInfo(&k, ctx, 10)
	for _, item := range items {
		k.RemovePoolInfo(ctx,
			item.PoolId,
		)
		_, found := k.GetPoolInfo(ctx,
			item.PoolId,
		)
		require.False(t, found)
	}
}

func TestPoolInfoGetAll(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolInfo(&k, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(k.GetAllPoolInfo(ctx)),
	)
}
