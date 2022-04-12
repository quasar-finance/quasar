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

func TestClaimRewards(t *testing.T) {
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
		oriontypes.CreateOrionRewardGloablMaccName(),
		sdk.NewCoins(sdk.NewCoin("QSR", mintAmount)),
	)
	require.NoError(t, err)

	// Give a claim of targetAmount of these coins for a user on orion vault
	keeper.AddUserClaimReward(keepers.Ctx, userAddr.String(), "orion", sdk.NewCoin("QSR", targetAmount))

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

	balance := keepers.BankKeeper.GetBalance(keepers.Ctx, userAddr, "QSR")
	require.Equal(t, targetAmount, balance.Amount)
	require.Equal(t, "QSR", balance.Denom)
}
