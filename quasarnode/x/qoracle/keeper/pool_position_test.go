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

func createTestPoolPosition(keeper *keeper.Keeper, ctx sdk.Context) types.PoolPosition {
	item := types.PoolPosition{}
	keeper.SetPoolPosition(ctx, item)
	return item
}

func TestPoolPositionGet(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	item := createTestPoolPosition(keeper, ctx)
	rst, found := keeper.GetPoolPosition(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestPoolPositionRemove(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	createTestPoolPosition(keeper, ctx)
	keeper.RemovePoolPosition(ctx)
	_, found := keeper.GetPoolPosition(ctx)
	require.False(t, found)
}
