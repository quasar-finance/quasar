package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestRequestDeposit(t *testing.T) {
	keepers := keepertest.NewTestSetup(t)
	_, keeper := keepers.GetQbankKeeper()
	server, srvCtx := setupMsgServer(keepers.Ctx, keeper)
	userAddr := sample.AccAddress()
	mintAmount := int64(100)
	initialBalance := int64(50)
	targetAmount := int64(42)

	keepers.AccountKeeper.NewAccountWithAddress(keepers.Ctx, userAddr)
	keepers.BankKeeper.MintCoins(
		keepers.Ctx,
		keepertest.QbankMaccName,
		sdk.NewCoins(sdk.NewCoin("QSR", sdk.NewInt(mintAmount))),
	)
	keepers.BankKeeper.SendCoinsFromModuleToAccount(
		keepers.Ctx,
		keepertest.QbankMaccName,
		userAddr,
		sdk.NewCoins(sdk.NewCoin("QSR", sdk.NewInt(initialBalance))),
	)

	d := types.NewMsgRequestDeposit(
		userAddr.String(),
		"HIGH",
		"orion",
		sdk.NewCoin("QSR", sdk.NewInt(targetAmount)),
		types.LockupTypes_Days_21,
	)

	res, err := server.RequestDeposit(srvCtx, d)
	require.NoError(t, err)
	require.NotNil(t, res)

	balance := keepers.BankKeeper.GetBalance(keepers.Ctx, userAddr, "QSR")
	require.Equal(t, sdk.NewInt(initialBalance-targetAmount), balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
