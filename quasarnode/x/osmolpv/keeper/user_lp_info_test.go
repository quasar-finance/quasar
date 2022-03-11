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

func createTestUserLPInfo(keeper *keeper.Keeper, ctx sdk.Context) types.UserLPInfo {
	item := types.UserLPInfo{}
	keeper.SetUserLPInfo(ctx, item)
	return item
}

func TestUserLPInfoGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestUserLPInfo(keeper, ctx)
	rst, found := keeper.GetUserLPInfo(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestUserLPInfoRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestUserLPInfo(keeper, ctx)
	keeper.RemoveUserLPInfo(ctx)
	_, found := keeper.GetUserLPInfo(ctx)
	require.False(t, found)
}
