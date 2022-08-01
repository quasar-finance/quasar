package types

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil/sample"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/stretchr/testify/require"
)

func TestMsgCreatePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgCreatePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreatePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomIn",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomOut",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Price",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgCreatePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "cba",
				Price:    "1.2",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "a",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "1",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "c",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "1",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid Price",
			msg: MsgCreatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "x",
				LastUpdatedTime: 1,
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

func TestMsgUpdatePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgUpdatePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdatePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
		}, {
			name: "empty PoolId",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomIn",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomOut",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty Price",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgUpdatePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "cba",
				Price:    "1.2",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "a",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "1",
				DenomOut:        "cba",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "c",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "1",
				Price:           "1.2",
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid Price",
			msg: MsgUpdatePoolSpotPrice{
				Creator:         sample.AccAddressStr(),
				PoolId:          "1",
				DenomIn:         "abc",
				DenomOut:        "cba",
				Price:           "x",
				LastUpdatedTime: 1,
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

func TestMsgDeletePoolSpotPrice_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgDeletePoolSpotPrice
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgDeletePoolSpotPrice{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "cba",
			},
		}, {
			name: "empty PoolId",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				DenomIn:  "abc",
				DenomOut: "cba",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomIn",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomOut: "cba",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty DenomOut",
			msg: MsgDeletePoolSpotPrice{
				Creator: sample.AccAddressStr(),
				PoolId:  "1",
				DenomIn: "abc",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "a",
				DenomOut: "cba",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomIn",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "1",
				DenomOut: "cba",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "c",
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "invalid DenomOut",
			msg: MsgDeletePoolSpotPrice{
				Creator:  sample.AccAddressStr(),
				PoolId:   "1",
				DenomIn:  "abc",
				DenomOut: "1",
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
