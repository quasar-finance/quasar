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

func createTestRewardCollection(keeper *keeper.Keeper, ctx sdk.Context) types.RewardCollection {
	item := types.RewardCollection{}
	keeper.SetRewardCollection(ctx, item)
	return item
}

func TestRewardCollectionGet(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	item := createTestRewardCollection(keeper, ctx)
	rst, found := keeper.GetRewardCollection(ctx)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestRewardCollectionRemove(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	createTestRewardCollection(keeper, ctx)
	keeper.RemoveRewardCollection(ctx)
	_, found := keeper.GetRewardCollection(ctx)
	require.False(t, found)
}
