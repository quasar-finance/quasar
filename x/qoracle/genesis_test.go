package qoracle_test

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/qoracle"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),
		// this line is used by starport scaffolding # genesis/test/state
	}

	setup := testutil.NewTestSetup(t)
	ctx, keeper := setup.Ctx, setup.Keepers.QoracleKeeper
	qoracle.InitGenesis(ctx, keeper, genesisState)
	setParams := keeper.GetParams(ctx)
	require.Equal(t, genesisState.Params, setParams)
	got := qoracle.ExportGenesis(ctx, keeper)

	require.NotNil(t, got)
	require.Equal(t, genesisState.Params, got.Params)
	// this line is used by starport scaffolding # genesis/test/assert
}
