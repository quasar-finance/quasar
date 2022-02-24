package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
)

func createTestPoolRanking(keeper *keeper.Keeper, ctx sdk.Context) types.PoolRanking {
	item := types.PoolRanking{}
	keeper.SetPoolRanking(ctx, item)
	return item
}

func TestPoolRankingGet(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	item := createTestPoolRanking(keeper, ctx)
	rst, found := keeper.GetPoolRanking(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestPoolRankingRemove(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	createTestPoolRanking(keeper, ctx)
	keeper.RemovePoolRanking(ctx)
	_, found := keeper.GetPoolRanking(ctx)
	require.False(t, found)
}
