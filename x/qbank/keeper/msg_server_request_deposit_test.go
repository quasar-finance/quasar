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
	tks := keepertest.NewTestKeeperState(t)
	server, ctx := setupMsgServer(tks)
	tks.LoadKVStores()
	userAddr := sample.AccAddress()
	mintAmount := int64(100)
	initialBalance := int64(50)
	targetAmount := int64(42)

	tks.GetAccountKeeper().NewAccountWithAddress(tks.Ctx, userAddr)
	tks.GetBankKeeper().MintCoins(
		tks.Ctx,
		keepertest.QbankMaccName,
		sdk.NewCoins(sdk.NewCoin("QSR", sdk.NewInt(mintAmount))),
	)
	tks.GetBankKeeper().SendCoinsFromModuleToAccount(
		tks.Ctx,
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

	res, err := server.RequestDeposit(ctx, d)
	require.NoError(t, err)
	require.NotNil(t, res)

	balance := tks.GetBankKeeper().GetBalance(tks.Ctx, userAddr, "QSR")
	require.Equal(t, sdk.NewInt(initialBalance-targetAmount), balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
