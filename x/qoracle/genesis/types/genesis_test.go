package types_test

import (
	"testing"

	"github.com/quasarlabs/quasarnode/x/qoracle/genesis/types"
	"github.com/stretchr/testify/require"
)

// TestGenesisState_Validate tests ValidateGenesis
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
	} { // for end
		t.Run(tc.desc, func(t *testing.T) {
			err := tc.genState.Validate()
			if tc.valid {
				require.NoError(t, err)
			} else {
				require.Error(t, err)
			}
		})
	} // for end
} // func end
