package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	errortest "github.com/abag/quasarnode/testutil/error"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubWithdrawableAmt(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	k.AddWithdrawableAmt(ctx, depositorAddr, coin1)
	k.AddWithdrawableAmt(ctx, depositorAddr, coin2)
	// Add same denom
	k.AddWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got = k.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)
	got = k.GetWithdrawableAmt(ctx, depositorAddr, denom2)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	k.SubWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)))
	got = k.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	k.SubWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got = k.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetWithdrawalAmtInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	got := k.GetWithdrawableAmt(ctx, "invalid", "ABC")
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubWithdrawalAmtInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubLockupWithdrawableAmt(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	k.AddLockupWithdrawableAmt(ctx, depositorAddr, coin1, types.LockupTypes_Days_7)
	k.AddLockupWithdrawableAmt(ctx, depositorAddr, coin2, types.LockupTypes_Months_1)
	// Add same denom / lockup
	k.AddLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)), types.LockupTypes_Months_1)

	got = k.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, coin1, got)
	got = k.GetLockupWithdrawableAmt(ctx, depositorAddr, denom2, types.LockupTypes_Months_1)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	k.SubLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)), types.LockupTypes_Days_7)
	got = k.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	k.SubLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)), types.LockupTypes_Days_7)
	got = k.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetLockupWithdrawalAmtInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	got := k.GetLockupWithdrawableAmt(ctx, "invalid", "ABC", types.LockupTypes_Days_7)
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubLockupWithdrawalAmtInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubLockupWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)), types.LockupTypes_Days_7)
	t.Errorf("did not panic")
}

func TestGetAddSubActualWithdrawableAmt(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	k.AddActualWithdrawableAmt(ctx, depositorAddr, coin1)
	k.AddActualWithdrawableAmt(ctx, depositorAddr, coin2)
	// Add same denom
	k.AddActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)
	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom2)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	k.SubActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)))
	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	k.SubActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetActualWithdrawalAmtInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	got := k.GetActualWithdrawableAmt(ctx, "invalid", "ABC")
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubActualWithdrawalAmtInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubActualWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestEmptyActualWithdrawableAmt(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	var got sdk.Coin

	k.AddActualWithdrawableAmt(ctx, depositorAddr, coin1)

	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)

	// Empty all denom
	k.EmptyActualWithdrawableAmt(ctx, depositorAddr, denom1)

	got = k.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}
