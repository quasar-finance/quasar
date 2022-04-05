package keeper_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/sample"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestWithdrawGet(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	depositorAddr := sample.AccAddressStr()
	denom := "QSR"

	got := keeper.GetWithdrawableAmt(ctx, depositorAddr, denom)
	require.Equal(t, sdk.NewInt(0), got.Amount)
}
