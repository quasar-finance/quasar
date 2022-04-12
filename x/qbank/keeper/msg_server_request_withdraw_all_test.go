package keeper_test

import (
	"testing"

	eventtest "github.com/abag/quasarnode/testutil/event"
	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestRequestWithdrawAll(t *testing.T) {
	keepers := keepertest.NewTestSetup(t)
	_, keeper := keepers.GetQbankKeeper()
	userAddr := sample.AccAddress()
	mintAmount := sdk.NewInt(int64(1000000000))
	targetAmount := sdk.NewInt(int64(42))
	server, srvCtx := setupMsgServer(keepers.Ctx, keeper)
	var err error

	// Mint some coins for orion account
	keepers.AccountKeeper.NewAccountWithAddress(keepers.Ctx, userAddr)
	err = keepers.BankKeeper.MintCoins(
		keepers.Ctx,
		oriontypes.ModuleName,
		sdk.NewCoins(sdk.NewCoin("QSR", mintAmount)),
	)
	require.NoError(t, err)
	err = keepers.BankKeeper.MintCoins(
		keepers.Ctx,
		oriontypes.ModuleName,
		sdk.NewCoins(sdk.NewCoin("FOO", mintAmount)),
	)
	require.NoError(t, err)

	// Give a claim of targetAmount of these coins for a user
	keeper.AddActualWithdrableAmt(keepers.Ctx, userAddr.String(), sdk.NewCoin("QSR", targetAmount))
	keeper.AddActualWithdrableAmt(keepers.Ctx, userAddr.String(), sdk.NewCoin("FOO", targetAmount))

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

	balance1 := keepers.BankKeeper.GetBalance(keepers.Ctx, userAddr, "QSR")
	require.Equal(t, targetAmount, balance1.Amount)
	require.Equal(t, "QSR", balance1.Denom)

	balance2 := keepers.BankKeeper.GetBalance(keepers.Ctx, userAddr, "FOO")
	require.Equal(t, targetAmount, balance2.Amount)
	require.Equal(t, "FOO", balance2.Denom)
}
