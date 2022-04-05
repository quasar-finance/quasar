package types_test

import (
	"testing"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func createSamplePoolMetricsSlice() []types.PoolMetrics {
	res := make([]types.PoolMetrics, 2)
	res[0].HighestAPY = "1.5"
	res[0].TVL = "1.5usd"
	res[0].GaugeAPYs = []*types.GaugeAPY{
		&types.GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
		&types.GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
	}
	res[1].HighestAPY = "2.5"
	res[1].TVL = "2.5usd"
	res[1].GaugeAPYs = []*types.GaugeAPY{
		&types.GaugeAPY{GaugeId: 3, Duration: "1s", APY: "2.1"},
		&types.GaugeAPY{GaugeId: 4, Duration: "2s", APY: "2.2"},
	}
	return res
}

func TestGenesisState_Validate(t *testing.T) {
	samplePoolMetricsSlice := createSamplePoolMetricsSlice()

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
						PoolId:          "1",
						Metrics:         &samplePoolMetricsSlice[0],
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "2",
						Metrics:         &samplePoolMetricsSlice[1],
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
						Metrics:         &samplePoolMetricsSlice[0],
						LastUpdatedTime: 1,
					},
					{
						PoolId:          "0",
						Metrics:         &samplePoolMetricsSlice[1],
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
