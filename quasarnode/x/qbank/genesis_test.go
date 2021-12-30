package qbank_test

import (
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qbank"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params: types.DefaultParams(),

		DepositList: []types.Deposit{
			{
				Id: 0,
			},
			{
				Id: 1,
			},
		},
		DepositCount: 2,
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.QbankKeeper(t)
	qbank.InitGenesis(ctx, *k, genesisState)
	got := qbank.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	require.ElementsMatch(t, genesisState.DepositList, got.DepositList)
	require.Equal(t, genesisState.DepositCount, got.DepositCount)
	// this line is used by starport scaffolding # genesis/test/assert
}
