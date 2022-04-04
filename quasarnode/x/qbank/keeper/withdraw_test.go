package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func createNWithdraw(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.Withdraw {
	items := make([]types.Withdraw, n)
	for i := range items {
		items[i].Id = keeper.AppendWithdraw(ctx, items[i])
	}
	return items
}

func TestWithdrawGet(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNWithdraw(keeper, ctx, 10)
	for _, item := range items {
		got, found := keeper.GetWithdraw(ctx, item.Id)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&got),
		)
	}
}

func TestWithdrawRemove(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNWithdraw(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemoveWithdraw(ctx, item.Id)
		_, found := keeper.GetWithdraw(ctx, item.Id)
		require.False(t, found)
	}
}

func TestWithdrawGetAll(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNWithdraw(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllWithdraw(ctx)),
	)
}

func TestWithdrawCount(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNWithdraw(keeper, ctx, 10)
	count := uint64(len(items))
	require.Equal(t, count, keeper.GetWithdrawCount(ctx))
}
