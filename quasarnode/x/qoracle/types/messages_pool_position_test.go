package types

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgCreatePoolPosition_ValidateBasic(t *testing.T) {
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
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
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
			name: "invalid APY",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "a", TVL: "1000.5usd"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 1",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "usd"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 2",
			msg: MsgCreatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "2"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgCreatePoolPosition{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Metrics: &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
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
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
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
			name: "invalid APY",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "a", TVL: "1000.5usd"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 1",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "usd"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid TVL 2",
			msg: MsgUpdatePoolPosition{
				Creator:         sample.AccAddress(),
				PoolId:          "1",
				Metrics:         &PoolMetrics{APY: "10.5", TVL: "2"},
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgUpdatePoolPosition{
				Creator: sample.AccAddress(),
				PoolId:  "1",
				Metrics: &PoolMetrics{APY: "10.5", TVL: "1000.5usd"},
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
