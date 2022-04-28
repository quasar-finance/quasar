package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
)

func createTestUserLPInfo(keeper *keeper.Keeper, ctx sdk.Context, userAddr string) (types.UserLPInfo, uint64, uint64) {
	item := types.UserLPInfo{
		PositionShare: sdk.NewDecWithPrec(12321, 2),
		Coins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
	}
	epochDay := uint64(42)
	lpId := uint64(1)
	keeper.SetUserLPInfo(ctx, epochDay, lpId, userAddr, item)
	return item, epochDay, lpId
}

func TestUserLPInfoGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	userAddr := sample.AccAddressStr()
	item, epochDay, lpId := createTestUserLPInfo(&k, ctx, userAddr)
	rst, found := k.GetUserLPInfo(ctx, epochDay, lpId, userAddr)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestUserLPInfoRemove(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	userAddr := sample.AccAddressStr()
	_, epochDay, lpId := createTestUserLPInfo(&k, ctx, userAddr)
	k.RemoveUserLPInfo(ctx, epochDay, lpId, userAddr)
	_, found := k.GetUserLPInfo(ctx, epochDay, lpId, userAddr)
	require.False(t, found)
}
