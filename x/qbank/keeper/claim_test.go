package keeper_test

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil"
	errortest "github.com/quasarlabs/quasarnode/testutil/error"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubClaimAmt(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	vaultId := "orion"
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	coins := sdk.NewCoins(coin1, coin2)
	var got types.QCoins
	var found bool

	k.AddUserClaimReward(ctx, depositorAddr, vaultId, coin1)
	k.AddUserClaimRewards(ctx, depositorAddr, vaultId, coins)

	got, found = k.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(100), got.Coins.AmountOf("ABC"))
	require.Equal(t, sdk.NewInt(100), got.Coins.AmountOf("DEF"))

	// Sub some amount
	k.SubUserClaimReward(ctx, depositorAddr, vaultId, sdk.NewCoin(denom1, sdk.NewInt(58)))
	got, found = k.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(42), got.Coins.AmountOf("ABC"))

	// Sub all
	k.SubUserClaimReward(ctx, depositorAddr, vaultId, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got, found = k.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Coins.AmountOf("ABC"))
}

func TestGetClaimAmtInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	_, found := k.GetUserClaimAmt(ctx, "invalid", "orion")
	// Invalid key should result in a not found
	require.False(t, found)
}

func TestSubClaimAmtInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubUserClaimReward(ctx, "invalid", "orion", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestClaimAll(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	vaultId := "orion"
	denom1 := "ABC"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	var got types.QCoins
	var found bool

	k.AddUserClaimReward(ctx, depositorAddr, vaultId, coin1)

	got, found = k.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(50), got.Coins.AmountOf("ABC"))

	// Claim all
	k.ClaimAll(ctx, depositorAddr, vaultId)
	got, found = k.GetUserClaimAmt(ctx, depositorAddr, vaultId)
	require.False(t, found)
}
