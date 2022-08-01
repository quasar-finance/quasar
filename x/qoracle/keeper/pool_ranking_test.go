package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func createTestPoolRanking(k *keeper.Keeper, ctx sdk.Context) types.PoolRanking {
	item := types.PoolRanking{
		Creator:            sample.AccAddressStr(),
		PoolIdsSortedByAPY: []string{"1", "2", "3"},
		PoolIdsSortedByTVL: []string{"2", "1", "3"},
		LastUpdatedTime:    1,
	}
	k.SetPoolRanking(ctx, item)
	return item
}

func TestPoolRankingGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	item := createTestPoolRanking(&k, ctx)
	rst, found := k.GetPoolRanking(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestPoolRankingRemove(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	createTestPoolRanking(&k, ctx)
	k.RemovePoolRanking(ctx)
	_, found := k.GetPoolRanking(ctx)
	require.False(t, found)
}
