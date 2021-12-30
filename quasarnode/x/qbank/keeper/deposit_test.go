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

func createNDeposit(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.Deposit {
	items := make([]types.Deposit, n)
	for i := range items {
		items[i].Id = keeper.AppendDeposit(ctx, items[i])
	}
	return items
}

func TestDepositGet(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNDeposit(keeper, ctx, 10)
	for _, item := range items {
		got, found := keeper.GetDeposit(ctx, item.Id)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&got),
		)
	}
}

func TestDepositRemove(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNDeposit(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemoveDeposit(ctx, item.Id)
		_, found := keeper.GetDeposit(ctx, item.Id)
		require.False(t, found)
	}
}

func TestDepositGetAll(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNDeposit(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllDeposit(ctx)),
	)
}

func TestDepositCount(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	items := createNDeposit(keeper, ctx, 10)
	count := uint64(len(items))
	require.Equal(t, count, keeper.GetDepositCount(ctx))
}
