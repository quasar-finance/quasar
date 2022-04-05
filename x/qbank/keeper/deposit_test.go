package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetAddSubUserDeposit(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	keeper.AddUserDeposit(ctx, depositorAddr, coin1)
	// Add same denom
	keeper.AddUserDeposit(ctx, depositorAddr, coin2)
	keeper.AddUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got, found := keeper.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, 2, got.Coins.Len())
	// Coins are sorted by denom
	require.Equal(t, nullify.Fill(&coin1), nullify.Fill(&got.Coins[0]))
	require.Equal(t, sdk.NewInt(101), got.Coins[1].Amount)

	// subtract 10 from ABC
	keeper.SubUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(10)))
	got, found = keeper.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(40), got.Coins[0].Amount)

	// subtract all from DEF
	keeper.SubUserDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(101)))
	got, found = keeper.GetUserDepositAmt(ctx, depositorAddr)
	require.True(t, found)
	require.Equal(t, 1, got.Coins.Len())
	require.Equal(t, sdk.NewInt(0), got.Coins.AmountOf(denom2))
}

func TestGetTotalDeposits(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	keeper.AddUserDeposit(ctx, depositorAddr, coin1)
	keeper.AddUserDeposit(ctx, depositorAddr, coin2)

	coins := keeper.GetTotalDeposits(ctx)
	require.Equal(t, sdk.NewInt(50), coins.AmountOf(denom1))
	require.Equal(t, sdk.NewInt(100), coins.AmountOf(denom2))
}

func TestGetUserDepositInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	_, found := keeper.GetUserDepositAmt(ctx, "invalid")
	require.False(t, found)
}

func TestSubUserDepositInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	keeper.SubUserDeposit(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubUserDenomDeposit(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	keeper.AddUserDenomDeposit(ctx, depositorAddr, coin1)

	got, found := keeper.GetUserDenomDepositAmt(ctx, depositorAddr, denom1)
	require.True(t, found)
	require.Equal(t, nullify.Fill(&coin1), nullify.Fill(&got))

	// Add same denom
	keeper.AddUserDenomDeposit(ctx, depositorAddr, coin2)
	keeper.AddUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(1)))

	got, found = keeper.GetUserDenomDepositAmt(ctx, depositorAddr, denom2)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(101), got.Amount)

	// subtract 10 from ABC
	keeper.SubUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom1, sdk.NewInt(10)))
	got, found = keeper.GetUserDenomDepositAmt(ctx, depositorAddr, denom1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(40), got.Amount)

	// subtract all from DEF
	keeper.SubUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(101)))
	got, found = keeper.GetUserDenomDepositAmt(ctx, depositorAddr, denom2)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetUserDenomDepositInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	_, found := keeper.GetUserDenomDepositAmt(ctx, "invalid", "invalid")
	require.False(t, found)
}

func TestSubUserDenomDepositInvalidKey(t *testing.T) {
	defer func() { recover() }()
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	keeper.SubUserDenomDeposit(ctx, "invalid", sdk.NewCoin("ABC", sdk.NewInt(100)))
	t.Errorf("did not panic")
}

func TestGetAddSubEpochLockupUserDenomDeposit(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	// same denom, different lockup
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_7)
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_21)

	got, found := keeper.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom1, uint64(1), types.LockupTypes_Days_7)
	require.True(t, found)
	require.Equal(t, nullify.Fill(&coin1), nullify.Fill(&got))

	// same denom, same lockup, should add up the balances
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(1), types.LockupTypes_Months_1)
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(1), types.LockupTypes_Months_1)

	got, found = keeper.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(200), got.Amount)

	// different denom, different epoch and lockups, not found
	got, found = keeper.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(2), types.LockupTypes_Months_1)
	require.False(t, found)
	got, found = keeper.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_3)
	require.False(t, found)

	// subtract all from DEF
	keeper.SubEpochLockupUserDenomDeposit(ctx, depositorAddr, sdk.NewCoin(denom2, sdk.NewInt(200)), uint64(1), types.LockupTypes_Months_1)
	got, found = keeper.GetEpochLockupUserDenomDepositAmt(ctx, depositorAddr, denom2, uint64(1), types.LockupTypes_Months_1)
	require.True(t, found)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}

func TestGetTotalEpochDeposits(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	coin1 := sdk.NewCoin(denom1, sdk.NewInt(50))
	coin2 := sdk.NewCoin(denom2, sdk.NewInt(100))

	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(1), types.LockupTypes_Days_7)
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin1, uint64(2), types.LockupTypes_Days_7)
	keeper.AddEpochLockupUserDenomDeposit(ctx, depositorAddr, coin2, uint64(2), types.LockupTypes_Days_7)

	got := keeper.GetTotalEpochDeposits(ctx, uint64(1))
	require.Equal(t, sdk.NewInt(50), got.AmountOf(denom1))
	got = keeper.GetTotalEpochDeposits(ctx, uint64(2))
	require.Equal(t, sdk.NewInt(50), got.AmountOf(denom1))
	require.Equal(t, sdk.NewInt(100), got.AmountOf(denom2))
}

func TestGetEpochLockupUserDenomDepositInvalidKey(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).QbankKeeper()
	_, found := keeper.GetEpochLockupUserDenomDepositAmt(ctx, "invalid", "invalid", uint64(0), types.LockupTypes_Days_7)
	require.False(t, found)
}
