package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	params := types.DefaultParams()

	k.SetParams(ctx, params)
	require.True(t, params.Equal(k.GetParams(ctx)))

	params.Enabled = false
	params.LpEpochId = "day"
	params.PerfFeePer = sdk.NewDecWithPrec(5, 2)
	params.MgmtFeePer = sdk.NewDecWithPrec(3, 3)

	k.SetParams(ctx, params)
	require.True(t, params.Equal(k.GetParams(ctx)))
}
