package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
}
