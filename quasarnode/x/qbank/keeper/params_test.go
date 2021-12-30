package keeper_test

import (
	"testing"

	testkeeper "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.QbankKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}
