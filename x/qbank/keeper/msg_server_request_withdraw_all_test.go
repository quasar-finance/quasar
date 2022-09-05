package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	eventtest "github.com/quasarlabs/quasarnode/testutil/event"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/stretchr/testify/require"
)

func TestRequestWithdrawAll(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	k := setup.Keepers.QbankKeeper
	userAddr := sample.AccAddress()
	mintAmount := sdk.NewInt(int64(1000000000))
	targetAmount := sdk.NewInt(int64(42))
	server, srvCtx := setupMsgServer(setup.Ctx, k)
	var err error

	// Mint some coins for orion account
	setup.Keepers.AccountKeeper.NewAccountWithAddress(setup.Ctx, userAddr)
	err = setup.Keepers.BankKeeper.MintCoins(
		setup.Ctx,
		oriontypes.ModuleName,
		sdk.NewCoins(sdk.NewCoin("QSR", mintAmount)),
	)
	require.NoError(t, err)
	err = setup.Keepers.BankKeeper.MintCoins(
		setup.Ctx,
		oriontypes.ModuleName,
		sdk.NewCoins(sdk.NewCoin("FOO", mintAmount)),
	)
	require.NoError(t, err)

	// Give a claim of targetAmount of these coins for a user
	k.AddActualWithdrawableAmt(setup.Ctx, userAddr.String(), sdk.NewCoin("QSR", targetAmount))
	k.AddActualWithdrawableAmt(setup.Ctx, userAddr.String(), sdk.NewCoin("FOO", targetAmount))

	// Then withdraw a target amount
	w := types.NewMsgRequestWithdrawAll(
		userAddr.String(),
		"orion",
	)
	res, err := server.RequestWithdrawAll(srvCtx, w)
	require.NoError(t, err)
	require.NotNil(t, res)

	ctx := sdk.UnwrapSDKContext(srvCtx)
	eventtest.AssertEventEmitted(t, ctx, types.TypeEvtWithdrawAll)

	balance1 := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "QSR")
	require.Equal(t, targetAmount, balance1.Amount)
	require.Equal(t, "QSR", balance1.Denom)

	balance2 := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "FOO")
	require.Equal(t, targetAmount, balance2.Amount)
	require.Equal(t, "FOO", balance2.Denom)

	totalWithdraw, _ := k.GetTotalWithdrawAmt(ctx, userAddr.String(), "orion")
	require.Equal(t,
		sdk.NewCoins(
			sdk.NewCoin("QSR", targetAmount),
			sdk.NewCoin("FOO", targetAmount)),
		totalWithdraw.Coins)
}
