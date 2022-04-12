package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubClaimAmt(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	vaultId := "orion"
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	coins := sdk.NewCoins(coin1, coin2)
	var got types.QCoins
	var found = false

	keeper.AddUserClaimReward(ctx, depositorAddr, vaultId, coin1)
	keeper.AddUserClaimRewards(ctx, depositorAddr, vaultId, coins)

	got, found = keeper.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(100), got.Coins.AmountOf("ABC"))
	require.Equal(t, sdk.NewInt(100), got.Coins.AmountOf("DEF"))

	// Sub some amount
	keeper.SubUserClaimReward(ctx, depositorAddr, vaultId, sdk.NewCoin(denom1, sdk.NewInt(58)))
	got, found = keeper.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(42), got.Coins.AmountOf("ABC"))

	// Sub all
	keeper.SubUserClaimReward(ctx, depositorAddr, vaultId, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got, found = keeper.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Coins.AmountOf("ABC"))
}

func TestGetClaimAmtInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	_, found := keeper.GetUserClaimAmt(ctx, "invalid", "orion")
	// Invalid key should result in a not found
	require.False(t, found)
}

func TestSubClaimAmtInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	keeper.SubUserClaimReward(ctx, "invalid", "orion", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestClaimAll(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	vaultId := "orion"
	denom1 := "ABC"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	var got types.QCoins
	var found = false

	keeper.AddUserClaimReward(ctx, depositorAddr, vaultId, coin1)

	got, found = keeper.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(50), got.Coins.AmountOf("ABC"))

	// Claim all
	keeper.ClaimAll(ctx, depositorAddr, vaultId)
	got, found = keeper.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.False(t, found)
}
