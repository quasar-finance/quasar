package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	errortest "github.com/abag/quasarnode/testutil/error"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubUserDeposit(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	k.AddUserDeposit(ctx, depositorAddr, coin1)
	// Add same denom
	k.AddUserDeposit(ctx, depositorAddr, coin2)
	k.AddUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got, found := k.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, 2, got.Coins.Len())
	// Coins are sorted by denom
	require.Equal(t, coin1, got.Coins[0])
	require.Equal(t, sdk.NewInt(101), got.Coins[1].Amount)

	// subtract 10 from ABC
	k.SubUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(10)))
	got, found = k.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(40), got.Coins[0].Amount)

	// subtract all from DEF
	k.SubUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(101)))
	got, found = k.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, 1, got.Coins.Len())
	require.Equal(t, sdk.NewInt(0), got.Coins.AmountOf(denom2))
}

func TestGetTotalDeposits(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	k.AddUserDeposit(ctx, depositorAddr, coin1)
	k.AddUserDeposit(ctx, depositorAddr, coin2)

	coins := k.GetTotalDeposits(ctx)
	require.Equal(t, sdk.NewInt(50), coins.AmountOf(denom1))
	require.Equal(t, sdk.NewInt(100), coins.AmountOf(denom2))
}

func TestGetUserDepositInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	_, found := k.GetUserDepositAmt(ctx, "invalid")
	require.False(t, found)
}

func TestSubUserDepositInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubUserDeposit(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubUserDenomDeposit(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	k.AddUserDenomDeposit(ctx, depositorAddr, coin1)

	got, found := k.GetUserDenomDepositAmt(ctx, depositorAddr, denom1)
	require.True(t, found)
	require.Equal(t, nullify.Fill(&coin1), nullify.Fill(&got))

	// Add same denom
	k.AddUserDenomDeposit(ctx, depositorAddr, coin2)
	k.AddUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got, found = k.GetUserDenomDepositAmt(ctx, depositorAddr, denom2)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// subtract 10 from ABC
	k.SubUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(10)))
	got, found = k.GetUserDenomDepositAmt(ctx, depositorAddr, denom1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(40), got.Amount)

	// subtract all from DEF
	k.SubUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(101)))
	got, found = k.GetUserDenomDepositAmt(ctx, depositorAddr, denom2)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetUserDenomDepositInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	_, found := k.GetUserDenomDepositAmt(ctx, "invalid", "invalid")
	require.False(t, found)
}

func TestSubUserDenomDepositInvalidKey(t *testing.T) {
	defer errortest.RecoverExpectedPanic()
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	k.SubUserDenomDeposit(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubEpochLockupUserDenomDeposit(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	// same denom, different lockup
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_7)
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_21)

	got, found := k.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom1, uint64(1), types.LockupTypes_Days_7)
	require.True(t, found)
	require.Equal(t, nullify.Fill(&coin1), nullify.Fill(&got))

	// same denom, same lockup, should add up the balances
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(1), types.LockupTypes_Months_1)
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(1), types.LockupTypes_Months_1)

	got, found = k.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(200), got.Amount)

	// different denom, different epoch and lockups, not found
	got, found = k.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(2), types.LockupTypes_Months_1)
	require.False(t, found)
	got, found = k.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_3)
	require.False(t, found)

	// subtract all from DEF
	k.SubEpochLockupUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(200)), uint64(1), types.LockupTypes_Months_1)
	got, found = k.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetTotalEpochDeposits(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_7)
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(2), types.LockupTypes_Days_7)
	k.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(2), types.LockupTypes_Days_7)

	got := k.GetTotalEpochDeposits(ctx, uint64(1))
	require.Equal(t, sdk.NewInt(50), got.AmountOf(denom1))
	got = k.GetTotalEpochDeposits(ctx, uint64(2))
	require.Equal(t, sdk.NewInt(50), got.AmountOf(denom1))
	require.Equal(t, sdk.NewInt(100), got.AmountOf(denom2))
}

func TestGetEpochLockupUserDenomDepositInvalidKey(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper
	_, found := k.GetEpochLockupUserDenomDepositAmt(ctx, "invalid", "invalid", uint64(0), types.LockupTypes_Days_7)
	require.False(t, found)
}
