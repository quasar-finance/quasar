package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func createSamplePoolMetricsMap() map[string]*PoolMetrics {
	return map[string]*PoolMetrics{
		"valid": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"invalid HighestAPY": &PoolMetrics{
			HighestAPY: "a",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"negative HighestAPY": &PoolMetrics{
			HighestAPY: "-10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"invalid TVL 1": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"invalid TVL 2": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"nil GaugeAPYs[0]": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs:  []*GaugeAPY{nil},
		},
		"invalid Duration": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"negative Duration": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "-2s", APY: "1.2"},
			},
		},
		"invalid APY": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "x"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
		"negative APY": &PoolMetrics{
			HighestAPY: "10.5",
			TVL:        "1000.5usd",
			GaugeAPYs: []*GaugeAPY{
				&GaugeAPY{GaugeId: 1, Duration: "1s", APY: "-1.1"},
				&GaugeAPY{GaugeId: 2, Duration: "2s", APY: "1.2"},
			},
		},
	}
}

func TestMsgCreatePoolPosition_ValidateBasic(t *testing.T) {
	samplePoolMetricsMap := createSamplePoolMetricsMap()

	tests := []struct {
		name string
		msg  MsgCreatePoolPosition
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreatePoolPosition{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["valid"],
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				Metrics:         samplePoolMetricsMap["valid"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil Metrics",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Metrics",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid HighestAPY",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid HighestAPY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "negative HighestAPY",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["negative HighestAPY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 1",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid TVL 1"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 2",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid TVL 2"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil GaugeAPYs[0]",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["nil GaugeAPYs[0]"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid Duration",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid Duration"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "negative Duration",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["negative Duration"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid APY",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid APY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "negative APY",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["negative APY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgCreatePoolPosition{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Metrics: samplePoolMetricsMap["valid"],
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgUpdatePoolPosition_ValidateBasic(t *testing.T) {
	samplePoolMetricsMap := createSamplePoolMetricsMap()

	tests := []struct {
		name string
		msg  MsgUpdatePoolPosition
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdatePoolPosition{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["valid"],
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				Metrics:         samplePoolMetricsMap["valid"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil Metrics",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Metrics",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid HighestAPY",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid HighestAPY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 1",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid TVL 1"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 2",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid TVL 2"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "nil GaugeAPYs[0]",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["nil GaugeAPYs[0]"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid Duration",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid Duration"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "negative Duration",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["negative Duration"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid APY",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["invalid APY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "negative APY",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         samplePoolMetricsMap["negative APY"],
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgUpdatePoolPosition{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Metrics: samplePoolMetricsMap["valid"],
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgDeletePoolPosition_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgDeletePoolPosition
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgDeletePoolPosition{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgDeletePoolPosition{
				Creator: sample.AccAddress(),
				PoolId:  "1",
			},
		}, {
			name: "empty PoolId",
			msg: MsgDeletePoolPosition{
				Creator: sample.AccAddress(),
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}
