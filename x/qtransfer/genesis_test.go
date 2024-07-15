package qtransfer_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/qtransfer"
	"github.com/quasarlabs/quasarnode/x/qtransfer/types"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),
	}

	setup := testutil.NewTestSetup(t)
	ctx, keeper := setup.Ctx, setup.Keepers.QTransfer
	qtransfer.InitGenesis(ctx, keeper, genesisState)
	setParams := keeper.GetParams(ctx)
	require.Equal(t, genesisState.Params, setParams)
	got := qtransfer.ExportGenesis(ctx, keeper)

	require.NotNil(t, got)
	require.Equal(t, genesisState.Params, got.Params)
}
