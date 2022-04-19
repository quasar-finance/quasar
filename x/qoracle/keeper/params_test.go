package keeper_test

import (
	"testing"

	testkeeper "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestGetParams(t *testing.T) {
	k, ctx := testkeeper.QoracleKeeper(t)
	params := types.DefaultParams()

	k.SetParams(ctx, params)

	require.EqualValues(t, params, k.GetParams(ctx))
	require.EqualValues(t, params.OracleAccounts, k.OracleAccounts(ctx))
	require.EqualValues(t, params.StableDenoms, k.StableDenoms(ctx))
	require.EqualValues(t, params.OneHopDenomMap, k.OneHopDenomMap(ctx))
}
