package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
)

func createTestEpochLPInfo(keeper *keeper.Keeper, ctx sdk.Context) types.EpochLPInfo {
	item := types.EpochLPInfo{
		EpochDay: uint64(42),
	}
	keeper.SetEpochLPInfo(ctx, item)
	return item
}

func TestEpochLPInfoGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	item := createTestEpochLPInfo(&k, ctx)
	rst, found := k.GetEpochLPInfo(ctx, item.EpochDay)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestEpochLPInfoRemove(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	item := createTestEpochLPInfo(&k, ctx)
	k.RemoveEpochLPInfo(ctx, item.EpochDay)
	_, found := k.GetEpochLPInfo(ctx, item.EpochDay)
	require.False(t, found)
}
