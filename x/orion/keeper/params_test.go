package keeper_test

import (
	"testing"

	testkeeper "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.OrionKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)
	require.EqualValues(t, params, k.GetParams(ctx))

	params.Enabled = false
	params.LpEpochId = "day"
	params.PerfFeePer = sdk.NewDecWithPrec(5, 2)
	params.MgmtFeePer = sdk.NewDecWithPrec(3, 3)

	k.SetParams(ctx, params)
	require.EqualValues(t, params, k.GetParams(ctx))

}
