package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	eventtest "github.com/abag/quasarnode/testutil/event"
	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestRequestDeposit(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	k := setup.Keepers.QbankKeeper
	params := k.GetParams(setup.Ctx)
	// Explicitly enable the qbank
	params.Enabled = true
	k.SetParams(setup.Ctx, params)
	userAddr := sample.AccAddress()
	mintAmount := sdk.NewInt(int64(1000000000))
	initialBalance := sdk.NewInt(int64(50))
	targetAmount := sdk.NewInt(int64(42))
	server, srvCtx := setupMsgServer(setup.Ctx, k)
	var err error

	// Mint coins for qbank module account
	setup.Keepers.AccountKeeper.NewAccountWithAddress(setup.Ctx, userAddr)
	err = setup.Keepers.BankKeeper.MintCoins(
		setup.Ctx,
		keepertest.QbankMaccName,
		sdk.NewCoins(sdk.NewCoin("QSR", mintAmount)),
	)
	require.NoError(t, err)
	// Allocate a portion of these coins to the user
	err = setup.Keepers.BankKeeper.SendCoinsFromModuleToAccount(
		setup.Ctx,
		keepertest.QbankMaccName,
		userAddr,
		sdk.NewCoins(sdk.NewCoin("QSR", initialBalance)),
	)
	require.NoError(t, err)

	// Deposit the targetAmount
	d := types.NewMsgRequestDeposit(
		userAddr.String(),
		"HIGH",
		"orion",
		sdk.NewCoin("QSR", targetAmount),
		types.LockupTypes_Days_21,
	)
	res, err := server.RequestDeposit(srvCtx, d)
	require.NoError(t, err)
	require.NotNil(t, res)

	ctx := sdk.UnwrapSDKContext(srvCtx)
	eventtest.AssertEventEmitted(t, ctx, types.TypeEvtDeposit)

	balance := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "QSR")
	require.Equal(t, initialBalance.Sub(targetAmount), balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
