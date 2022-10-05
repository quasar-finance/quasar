package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	eventtest "github.com/quasarlabs/quasarnode/testutil/event"
	keepertest "github.com/quasarlabs/quasarnode/testutil/keeper"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/stretchr/testify/require"
)

func TestRequestDeposit(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	k := setup.Keepers.QbankKeeper
	qoraclekeeper := setup.Keepers.QoracleKeeper
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
	reservedFields := []string{}
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
		"orion",
		sdk.NewCoin("QSR", targetAmount),
		types.LockupTypes_Days_21,
		reservedFields,
	)

	res1, err1 := server.RequestDeposit(srvCtx, d)
	require.Error(t, err1)
	require.Nil(t, res1)

	// Setting QSR stable price to 10 dollar, so target dollar deposit amount to be greater than default min 100dollar
	qoraclekeeper.SetStablePrice(setup.Ctx, "QSR", sdk.MustNewDecFromStr("10"))
	res2, err2 := server.RequestDeposit(srvCtx, d)
	require.NoError(t, err2)
	require.NotNil(t, res2)

	ctx := sdk.UnwrapSDKContext(srvCtx)
	eventtest.AssertEventEmitted(t, ctx, types.TypeEvtDeposit)

	balance := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "QSR")
	require.Equal(t, initialBalance.Sub(targetAmount), balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
