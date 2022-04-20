package keeper_test

import (
	"github.com/stretchr/testify/require"
	"testing"
	"time"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func createTestRewardCollection(keeper *keeper.Keeper, ctx sdk.Context) types.RewardCollection {
	item := types.RewardCollection{
		TimeCollected: time.Now().UTC(),
		Coins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
	}
	keeper.SetRewardCollection(ctx, 42, item)
	return item
}

func TestRewardCollection(t *testing.T) {
	k, ctx := keepertest.OrionKeeper(t)
	item := createTestRewardCollection(k, ctx)
	rst, found := k.GetRewardCollection(ctx, 42)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestRemoveRewardCollection(t *testing.T) {
	k, ctx := keepertest.OrionKeeper(t)
	createTestRewardCollection(k, ctx)
	k.RemoveRewardCollection(ctx, 42)
	_, found := k.GetRewardCollection(ctx, 42)
	require.False(t, found)
}
