package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubWithdrawableAmt(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	keeper.AddWithdrawableAmt(ctx, depositorAddr, coin1)
	keeper.AddWithdrawableAmt(ctx, depositorAddr, coin2)
	// Add same denom
	keeper.AddWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got = keeper.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)
	got = keeper.GetWithdrawableAmt(ctx, depositorAddr, denom2)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	keeper.SubWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)))
	got = keeper.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	keeper.SubWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got = keeper.GetWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetWithdrawalAmtInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	got := keeper.GetWithdrawableAmt(ctx, "invalid", "ABC")
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubWithdrawalAmtInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	keeper.SubWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubLockupWithdrawableAmt(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	keeper.AddLockupWithdrawableAmt(ctx, depositorAddr, coin1, types.LockupTypes_Days_7)
	keeper.AddLockupWithdrawableAmt(ctx, depositorAddr, coin2, types.LockupTypes_Months_1)
	// Add same denom / lockup
	keeper.AddLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)), types.LockupTypes_Months_1)

	got = keeper.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, coin1, got)
	got = keeper.GetLockupWithdrawableAmt(ctx, depositorAddr, denom2, types.LockupTypes_Months_1)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	keeper.SubLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)), types.LockupTypes_Days_7)
	got = keeper.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	keeper.SubLockupWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)), types.LockupTypes_Days_7)
	got = keeper.GetLockupWithdrawableAmt(ctx, depositorAddr, denom1, types.LockupTypes_Days_7)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetLockupWithdrawalAmtInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	got := keeper.GetLockupWithdrawableAmt(ctx, "invalid", "ABC", types.LockupTypes_Days_7)
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubLockupWithdrawalAmtInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	keeper.SubLockupWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)), types.LockupTypes_Days_7)
	t.Errorf("did not panic")
}

func TestGetAddSubActualWithdrawableAmt(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))
	var got sdk.Coin

	keeper.AddActualWithdrawableAmt(ctx, depositorAddr, coin1)
	keeper.AddActualWithdrawableAmt(ctx, depositorAddr, coin2)
	// Add same denom
	keeper.AddActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)
	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom2)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// Sub some amount
	keeper.SubActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(8)))
	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(42), got.Amount)

	// Sub all
	keeper.SubActualWithdrawableAmt(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(42)))
	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetActualWithdrawalAmtInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	got := keeper.GetActualWithdrawableAmt(ctx, "invalid", "ABC")
	// Invalid key should yield a zero amount coin
	require.Equal(t, sdk.NewInt(0), got.Amount)
	require.Equal(t, "ABC", got.Denom)
}

func TestSubActualWithdrawalAmtInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	keeper.SubActualWithdrawableAmt(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestEmptyActualWithdrawableAmt(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	var got sdk.Coin

	keeper.AddActualWithdrawableAmt(ctx, depositorAddr, coin1)

	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, coin1, got)

	// Empty all denom
	keeper.EmptyActualWithdrawableAmt(ctx, depositorAddr, denom1)

	got = keeper.GetActualWithdrawableAmt(ctx, depositorAddr, denom1)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}
