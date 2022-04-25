package types_test

import (
	"testing"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func Test_AddressPrefix(t *testing.T) {
	config := sdk.GetConfig()
	config.SetBech32PrefixForAccount("quasar", "quasarpub")

	addr1, err1 := sdk.AccAddressFromBech32("quasar1yl6hdjhmkf37639730gffanpzndzdpmhquv56x")
	require.NoError(t, err1)
	require.Equal(t, "quasar1yl6hdjhmkf37639730gffanpzndzdpmhquv56x", sdk.MustBech32ifyAddressBytes("quasar", addr1))

}

func TestGenesisState_Validate(t *testing.T) {
	for _, tc := range []struct {
		desc     string
		genState *types.GenesisState
		valid    bool
	}{
		{
			desc:     "default is valid",
			genState: types.DefaultGenesis(),
			valid:    true,
		},
		{
			desc:     "invalid empty genesis state",
			genState: &types.GenesisState{
				// this line is used by starport scaffolding # types/genesis/validField
			},
			valid: false, // Genesis params can not be nil
		},
		// this line is used by starport scaffolding # types/genesis/testcase
	} {
		t.Run(tc.desc, func(t *testing.T) {
			err := tc.genState.Validate()
			if tc.valid {
				require.NoError(t, err)
			} else {
				require.Error(t, err)
			}
		})
	}
}
