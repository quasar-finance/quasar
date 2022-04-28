package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	eventtest "github.com/abag/quasarnode/testutil/event"
	"github.com/abag/quasarnode/testutil/sample"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestRequestWithdraw(t *testing.T) {
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

	// Give a claim of targetAmount of these coins for a user
	k.AddActualWithdrawableAmt(setup.Ctx, userAddr.String(), sdk.NewCoin("QSR", targetAmount))

	// Then withdraw a target amount
	w := types.NewMsgRequestWithdraw(
		userAddr.String(),
		"HIGH",
		"orion",
		sdk.NewCoin("QSR", targetAmount),
	)
	res, err := server.RequestWithdraw(srvCtx, w)
	require.NoError(t, err)
	require.NotNil(t, res)

	ctx := sdk.UnwrapSDKContext(srvCtx)
	eventtest.AssertEventEmitted(t, ctx, types.TypeEvtWithdraw)

	balance := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "QSR")
	require.Equal(t, targetAmount, balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
