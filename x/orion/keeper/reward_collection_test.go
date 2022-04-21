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

func createTestRewardCollection(keeper *keeper.Keeper, ctx sdk.Context) (types.RewardCollection, uint64) {
	item := types.RewardCollection{
		TimeCollected: time.Now().UTC(),
		Coins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
	}
	epochDay := uint64(42)
	keeper.SetRewardCollection(ctx, epochDay, item)
	return item, epochDay
}

func TestRewardCollection(t *testing.T) {
	ctx, k := keepertest.NewTestSetup(t).GetOrionKeeper()
	item, epochDay := createTestRewardCollection(&k, ctx)
	rst, found := k.GetRewardCollection(ctx, epochDay)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestRemoveRewardCollection(t *testing.T) {
	ctx, k := keepertest.NewTestSetup(t).GetOrionKeeper()
	_, epochDay := createTestRewardCollection(&k, ctx)
	k.RemoveRewardCollection(ctx, epochDay)
	_, found := k.GetRewardCollection(ctx, epochDay)
	require.False(t, found)
}
