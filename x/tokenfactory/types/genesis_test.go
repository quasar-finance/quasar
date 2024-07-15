package types_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/cmd/quasarnoded/cmd"
	"github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func TestGenesisState_Validate(t *testing.T) {
	cmd.InitTestConfig()
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
			desc: "valid genesis state",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
						},
					},
				},
			},
			valid: true,
		},
		{
			desc: "different admin from creator",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu",
						},
					},
				},
			},
			valid: true,
		},
		{
			desc: "empty admin",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "",
						},
					},
				},
			},
			valid: true,
		},
		{
			desc: "no admin",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
					},
				},
			},
			valid: true,
		},
		{
			desc: "invalid admin",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "moose",
						},
					},
				},
			},
			valid: false,
		},
		{
			desc: "multiple denoms",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "",
						},
					},
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/litecoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "",
						},
					},
				},
			},
			valid: true,
		},
		{
			desc: "duplicate denoms",
			genState: &types.GenesisState{
				FactoryDenoms: []types.GenesisDenom{
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "",
						},
					},
					{
						Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
						AuthorityMetadata: types.DenomAuthorityMetadata{
							Admin: "",
						},
					},
				},
			},
			valid: false,
		},
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
