package types_test

import (
	"testing"

	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/stretchr/testify/require"
)

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
			desc: "valid genesis state",
			genState: &types.GenesisState{

				FeeData: &types.FeeData{
					FeeCollector: "37",
					FeeType:      17,
					BlockHeight:  95,
					Memo:         "96",
				},
				LpPosition: &types.LpPosition{
					LpID:                   17,
					LockID:                 84,
					BondingStartEpochDay:   30,
					BondDuration:           76,
					UnbondingStartEpochDay: 92,
					UnbondingDuration:      41,
					PoolID:                 6,
				},
				EpochLPInfo: &types.EpochLPInfo{
					EpochDay: 19,
					TotalLps: 39,
				},
				RewardCollection: &types.RewardCollection{
					TimeCollected: 21,
				},
				UserLPInfo: &types.UserLPInfo{
					PositionShare: 22,
				},
				LpStat: &types.LpStat{
					LpCount: 18,
				},
				// this line is used by starport scaffolding # types/genesis/validField
			},
			valid: true,
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
