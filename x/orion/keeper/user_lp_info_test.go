package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
)

func createTestUserLPInfo(keeper *keeper.Keeper, ctx sdk.Context, userAddr string) types.UserLPInfo {
	item := types.UserLPInfo{}
	keeper.SetUserLPInfo(ctx, uint64(42), uint64(0), userAddr, item)
	return item
}

func TestUserLPInfoGet(t *testing.T) {
	keeper, ctx := keepertest.OrionKeeper(t)
	userAddr := sample.AccAddressStr()
	item := createTestUserLPInfo(keeper, ctx, userAddr)
	rst, found := keeper.GetUserLPInfo(ctx, uint64(42), uint64(0), userAddr)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestUserLPInfoRemove(t *testing.T) {
	keeper, ctx := keepertest.OrionKeeper(t)
	userAddr := sample.AccAddressStr()
	createTestUserLPInfo(keeper, ctx, userAddr)
	keeper.RemoveUserLPInfo(ctx, uint64(42), uint64(0), userAddr)
	_, found := keeper.GetUserLPInfo(ctx, uint64(42), uint64(0), userAddr)
	require.False(t, found)
}
