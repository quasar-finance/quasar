package keeper_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	testkeeper "github.com/quasarlabs/quasarnode/testutil/keeper"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.QVestingKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}
