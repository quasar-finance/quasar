package keeper_test

import (
	"testing"

	"github.com/quasarlabs/quasarnode/x/vestingcustom/types"
	"github.com/stretchr/testify/require"
	testkeeper "quasar/testutil/keeper"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.VestingcustomKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}
