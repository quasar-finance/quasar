package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
	require.EqualValues(t, params.OracleAccounts, k.OracleAccounts(ctx))
	require.EqualValues(t, params.StableDenoms, k.StableDenoms(ctx))
	require.EqualValues(t, params.OneHopDenomMap, k.OneHopDenomMap(ctx))
}
