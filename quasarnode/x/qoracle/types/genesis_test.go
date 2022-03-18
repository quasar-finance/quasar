package types_test

import (
	"testing"

	"github.com/abag/quasarnode/x/qoracle/types"
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

				PoolPositionList: []types.PoolPosition{
					{
						PoolId:          "0",
						Metrics:         &types.PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "1",
						Metrics:         &types.PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
						LastUpdatedTime: 1,
					},
				},
				PoolRanking: &types.PoolRanking{
					PoolIdsSortedByAPY: []string{"1"},
					PoolIdsSortedByTVL: []string{"1"},
					LastUpdatedTime:    1,
				},
				PoolSpotPriceList: []types.PoolSpotPrice{
					{
						PoolId:          "0",
						DenomIn:         "abc",
						DenomOut:        "cba",
						Price:           "1.2",
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "1",
						DenomIn:         "xyz",
						DenomOut:        "zyx",
						Price:           "1.2",
						LastUpdatedTime: 1,
					},
				},
				PoolInfoList: []types.PoolInfo{
					{
						PoolId: "0",
					},
					{
						PoolId: "1",
					},
				},
				// this line is used by starport scaffolding # types/genesis/validField
			},
			valid: true,
		},
		{
			desc: "duplicated poolPosition",
			genState: &types.GenesisState{
				PoolPositionList: []types.PoolPosition{
					{
						PoolId:          "0",
						Metrics:         &types.PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "0",
						Metrics:         &types.PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
						LastUpdatedTime: 1,
					},
				},
			},
			valid: false,
		},
		{
			desc: "duplicated poolSpotPrice",
			genState: &types.GenesisState{
				PoolSpotPriceList: []types.PoolSpotPrice{
					{
						PoolId:          "0",
						DenomIn:         "abc",
						DenomOut:        "cba",
						Price:           "1.2",
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "0",
						DenomIn:         "abc",
						DenomOut:        "cba",
						Price:           "1.2",
						LastUpdatedTime: 1,
					},
				},
			},
			valid: false,
		},
		{
			desc: "duplicated poolInfo",
			genState: &types.GenesisState{
				PoolInfoList: []types.PoolInfo{
					{
						PoolId: "0",
					},
					{
						PoolId: "0",
					},
				},
			},
			valid: false,
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
