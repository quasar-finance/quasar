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

func TestClaimRewards(t *testing.T) {
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
		oriontypes.CreateOrionRewardGloablMaccName(),
		sdk.NewCoins(sdk.NewCoin("QSR", mintAmount)),
	)
	require.NoError(t, err)

	// Give a claim of targetAmount of these coins for a user on orion vault
	k.AddUserClaimReward(setup.Ctx, userAddr.String(), "orion", sdk.NewCoin("QSR", targetAmount))

	// Then withdraw a target amount
	w := types.NewMsgClaimRewards(
		userAddr.String(),
		"orion",
	)
	res, err := server.ClaimRewards(srvCtx, w)
	require.NoError(t, err)
	require.NotNil(t, res)

	ctx := sdk.UnwrapSDKContext(srvCtx)
	eventtest.AssertEventEmitted(t, ctx, types.TypeEvtClaimRewards)

	balance := setup.Keepers.BankKeeper.GetBalance(setup.Ctx, userAddr, "QSR")
	totalClaimed, _ := k.GetUserClaimedAmt(setup.Ctx, userAddr.String(), "orion")

	require.Equal(t, targetAmount, balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
	require.Equal(t, sdk.NewCoins(sdk.NewCoin("QSR", targetAmount)), totalClaimed.Coins)
}
